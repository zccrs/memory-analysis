mod types;

use anyhow::{Context, Result};
use log::{debug, info};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::io::Write;
use crate::local::LocalExecutor;
pub use types::*;

pub struct Collector {
    executor: Arc<Mutex<LocalExecutor>>,
    temp_dir: PathBuf,
    max_processes: Option<usize>,
}

impl Collector {
    // ... 其他方法 ...

    pub fn collect_single_process(&self, pid: i32) -> Result<String> {
        let executor = Arc::new(Mutex::new(LocalExecutor::new()));
        match self.collect_process_info_parallel(pid, executor) {
            Ok(info) => {
                let mut output = String::new();
                output.push_str(&format!("进程信息 (PID: {})\n", pid));
                output.push_str(&format!("名称: {}\n", info.name));
                output.push_str(&format!("可执行文件: {:?}\n", info.exe_path));
                output.push_str(&format!("可执行文件大小: {}\n", format_size(info.exe_size)));
                output.push_str(&format!("PSS: {}\n", format_size(info.pss)));
                output.push_str(&format!("RSS: {}\n", format_size(info.rss)));
                output.push_str(&format!("共享内存: {}\n", format_size(info.shared_memory)));
                output.push_str(&format!("打开文件数: {}\n", info.open_files_count));
                output.push_str("\n动态库信息:\n");
                for lib in &info.libraries {
                    output.push_str(&format!("- {:?} ({})\n",
                        lib.path,
                        format_size(lib.size)));
                }
                Ok(output)
            }
            Err(e) => anyhow::bail!("无法获取进程 {} 的信息: {}", pid, e)
        }
    }
}

impl Collector {
    pub fn new(temp_dir: PathBuf, max_processes: Option<usize>) -> Self {
        Self {
            executor: Arc::new(Mutex::new(LocalExecutor::new())),
            temp_dir,
            max_processes,
        }
    }

    pub fn collect(&mut self) -> Result<CollectionResult> {
        info!("开始收集系统信息...");

        // 创建临时目录
        std::fs::create_dir_all(&self.temp_dir)?;

        // 获取当前用户名和组ID
        let username = {
            let mut executor = self.executor.lock().unwrap();
            let (whoami, _) = executor.execute_command("whoami")?;
            // 设置目录所有者为当前用户
            executor.execute_sudo_command(&format!("chown -R {}:{} {:?}", whoami.trim(), whoami.trim(), self.temp_dir))?;
            whoami.trim().to_string()
        };
        debug!("已将临时目录所有者设置为: {}", username);

        // 收集系统信息
        let mut system_info = self.collect_system_info()?;

        // 收集进程信息
        let (processes, skipped_count) = self.collect_processes()?;
        system_info.skipped_processes = skipped_count;

        // 计算进程内存总和（使用PSS）
        system_info.processes_memory = processes.values()
            .map(|p| p.pss)
            .sum();

        // 计算内核占用的内存
        system_info.kernel_memory = system_info.used_memory.saturating_sub(system_info.processes_memory);

        info!("系统内存概况:");
        info!("- 总物理内存: {} MB", system_info.total_memory / 1024 / 1024);
        info!("- 已用内存: {} MB", system_info.used_memory / 1024 / 1024);
        info!("- 剩余内存: {} MB", (system_info.total_memory - system_info.used_memory) / 1024 / 1024);
        info!("\n实际进程内存使用:");
        info!("- 进程内存总量(PSS): {} MB", system_info.processes_memory / 1024 / 1024);
        info!("- 内核内存占用: {} MB", system_info.kernel_memory / 1024 / 1024);
        info!("- 共享内存总量: {} MB", system_info.total_shared_memory / 1024 / 1024);

        Ok(CollectionResult {
            system_info,
            processes,
        })
    }

    fn collect_system_info(&mut self) -> Result<SystemInfo> {
        info!("收集系统基本信息...");

        let (hostname, kernel_version, os_release, cpu_info, pagesize, meminfo, kernel_file_size, initrd_file_size) = {
            let mut executor = self.executor.lock().unwrap();

            let (hostname, _) = executor.execute_command("hostname")?;
            let hostname = hostname.trim().to_string();

            let (kernel_str, _) = executor.execute_sudo_command("uname -r")?;
            let kernel_version = kernel_str.trim().to_string();

            let (os_str, _) = executor.execute_command("cat /etc/os-release | grep PRETTY_NAME")?;
            let os_release = os_str
                .trim()
                .strip_prefix("PRETTY_NAME=")
                .unwrap_or("")
                .trim_matches('"')
                .to_string();

            let (cpu_str, _) = executor.execute_command("cat /proc/cpuinfo | grep 'model name' | head -1")?;
            let cpu_info = cpu_str
                .trim()
                .strip_prefix("model name")
                .unwrap_or("")
                .trim_matches(|c| c == ':' || c == ' ')
                .to_string();

            let (pagesize_str, _) = executor.execute_command("getconf PAGE_SIZE")?;
            let pagesize = pagesize_str.trim().to_string();

            let (meminfo_str, _) = executor.execute_sudo_command("cat /proc/meminfo")?;
            let meminfo = meminfo_str;

            // 获取内核和initrd文件大小
            let kernel_version_clean = kernel_version.trim();
            let mut kernel_file_size = 0;
            let mut initrd_file_size = 0;

            debug!("正在搜索当前正在使用的内核文件...");

            // 首先尝试通过/proc/cmdline获取当前使用的内核文件
            let mut current_kernel_path = None;
            if let Ok((cmdline, _)) = executor.execute_sudo_command("cat /proc/cmdline") {
                debug!("内核启动参数: {}", cmdline.trim());
                for part in cmdline.split_whitespace() {
                    if part.starts_with("BOOT_IMAGE=") {
                        current_kernel_path = Some(part.trim_start_matches("BOOT_IMAGE=").to_string());
                        break;
                    }
                }
            }

            // 如果从cmdline获取失败，使用uname -r获取当前运行的内核版本
            if current_kernel_path.is_none() {
                let (kernel_release, _) = executor.execute_sudo_command("uname -r")?;
                let kernel_release = kernel_release.trim();
                debug!("当前运行的内核版本: {}", kernel_release);

                // 构造可能的内核文件路径
                let possible_paths = vec![
                    format!("/boot/vmlinuz-{}", kernel_release),
                    format!("/usr/lib/modules/{}/vmlinuz", kernel_release),
                    format!("/usr/lib/boot/vmlinuz-{}", kernel_release)
                ];

                // 检查这些路径是否存在
                for path in &possible_paths {
                    if let Ok((_, _)) = executor.execute_sudo_command(&format!("test -f {}", path)) {
                        current_kernel_path = Some(path.clone());
                        debug!("根据内核版本找到内核文件: {}", path);
                        break;
                    }
                }
            }

            // 无论是否找到当前内核，都进行完整搜索以确保不会遗漏
            let (kernel_find, _) = executor.execute_sudo_command(
                &format!(
                    "find /boot /usr/lib/modules /usr/lib/boot -type f \\( -name 'vmlinuz*' -o -name 'vmlinux*' -o -name 'kernel*' \\) 2>/dev/null"
                )
            )?;

            let mut kernel_paths: Vec<String> = kernel_find.split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            // 如果找到了当前正在使用的内核文件路径，将其放在列表最前面
            if let Some(current_path) = current_kernel_path {
                if let Some(pos) = kernel_paths.iter().position(|p| p.contains(&current_path)) {
                    let current = kernel_paths.remove(pos);
                    kernel_paths.insert(0, current);
                } else {
                    kernel_paths.insert(0, current_path);
                }
            }

            debug!("发现以下内核文件（按优先级排序）:\n{}", kernel_paths.join("\n"));

            // 遍历排序后的内核文件列表
            for kernel_path in &kernel_paths {
                match executor.execute_sudo_command(&format!("stat -c %s {}", kernel_path)) {
                    Ok((size, _)) => {
                        if let Ok(s) = size.trim().parse::<u64>() {
                            kernel_file_size = s;
                            debug!("使用内核文件: {} (大小: {})", kernel_path, format_size(s));
                            break;
                        }
                    }
                    Err(e) => debug!("无法获取内核文件 {} 的大小: {}", kernel_path, e),
                }
            }

            debug!("正在搜索当前正在使用的initramfs文件...");

            // 从/proc/cmdline中查找initrd信息
            let mut current_initrd_path = None;
            if let Ok((cmdline, _)) = executor.execute_sudo_command("cat /proc/cmdline") {
                for part in cmdline.split_whitespace() {
                    if part.starts_with("initrd=") || part.starts_with("initramfs=") {
                        current_initrd_path = Some(part.split('=').nth(1).unwrap().to_string());
                        break;
                    }
                }
            }

            // 使用find命令搜索可能的initramfs文件
            let (initrd_find, _) = executor.execute_sudo_command(
                &format!(
                    "find /boot /usr/lib/modules /usr/lib/boot -type f \\( -name 'initramfs*' -o -name 'initrd*' -o -name 'booster*' \\) 2>/dev/null"
                )
            )?;

            let mut initrd_paths: Vec<String> = initrd_find.split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            // 如果找到了当前正在使用的initramfs文件路径，将其放在列表最前面
            if let Some(current_path) = current_initrd_path {
                if let Some(pos) = initrd_paths.iter().position(|p| p.contains(&current_path)) {
                    let current = initrd_paths.remove(pos);
                    initrd_paths.insert(0, current);
                } else {
                    initrd_paths.insert(0, current_path);
                }
            } else {
                // 如果没有从cmdline找到，将与当前内核版本匹配的文件放在前面
                initrd_paths.sort_by(|a, b| {
                    let a_matches = a.contains(kernel_version_clean);
                    let b_matches = b.contains(kernel_version_clean);
                    b_matches.cmp(&a_matches)
                });
            }

            debug!("发现以下initramfs文件（按优先级排序）:\n{}", initrd_paths.join("\n"));

            // 遍历排序后的initramfs文件列表
            for initrd_path in &initrd_paths {
                match executor.execute_sudo_command(&format!("stat -c %s {}", initrd_path)) {
                    Ok((size, _)) => {
                        if let Ok(s) = size.trim().parse::<u64>() {
                            initrd_file_size = s;
                            debug!("使用initramfs文件: {} (大小: {})", initrd_path, format_size(s));
                            break;
                        }
                    }
                    Err(e) => debug!("无法获取initramfs文件 {} 的大小: {}", initrd_path, e),
                }
            }

            if kernel_file_size == 0 {
                debug!("警告：未找到任何内核文件");
                // 尝试使用uname -v获取更多信息
                if let Ok((uname_v, _)) = executor.execute_sudo_command("uname -v") {
                    debug!("内核构建信息: {}", uname_v.trim());
                }
            }

            if initrd_file_size == 0 {
                debug!("警告：未找到任何initramfs文件");
                // 检查dracut或mkinitcpio是否安装
                if let Ok((initramfs_tools, _)) = executor.execute_sudo_command("which dracut mkinitcpio 2>/dev/null") {
                    debug!("已安装的initramfs工具: {}", initramfs_tools.trim());
                }
            }

            (hostname,
             kernel_version,
             os_release,
             cpu_info,
             pagesize,
             meminfo,
             kernel_file_size,
             initrd_file_size)
        }; // 释放executor锁

        // 解析内存信息
        let (total_memory, used_memory) = parse_meminfo(&meminfo)
            .context("Failed to parse meminfo")?;

        // 获取共享内存总量
        let total_shared_memory = self.collect_total_shared_memory()?;

        Ok(SystemInfo {
            hostname,
            kernel_version,
            os_release,
            cpu_info,
            page_size: pagesize.trim().parse()?,
            total_memory,
            used_memory,
            total_shared_memory,
            kernel_memory: 0,       // 将在collect_processes后更新
            processes_memory: 0,    // 将在collect_processes后更新
            kernel_file_size,
            initrd_file_size,
            skipped_processes: 0,   // 将在collect_processes中更新
            collection_time: chrono::Utc::now(),
        })
    }

    fn collect_processes(&mut self) -> Result<(HashMap<i32, ProcessInfo>, usize)> {
        info!("收集进程信息...");
        let executor = self.executor.clone();
        let skipped_count = Arc::new(Mutex::new(0));

        // 使用ps命令获取进程列表并按RSS排序（内存使用量从大到小）
        let ps_output = {
            let executor = self.executor.lock().unwrap();
            let (output, _) = executor.execute_command("ps -e -o pid=,rss=,comm= --sort=-rss")?;
            output
        };

        // 将进程信息解析为Vec以便并行处理
        let processes: Vec<(i32, String)> = ps_output.lines()
            .filter_map(|line| {
                // ps 命令输出的格式是: "  PID COMMAND  "，需要处理前导空格
                let mut parts = line.trim().split_whitespace();
                if let (Some(pid_str), Some(_rss_str), Some(comm_str)) = (parts.next(), parts.next(), parts.next()) {
                    if let Ok(pid) = pid_str.parse::<i32>() {
                        return Some((pid, comm_str.to_string()));
                    }
                }
                None
            })
            .collect();

        let total_count = processes.len();
        info!("共发现 {} 个进程", total_count);

        let progress = Arc::new(Mutex::new(0));

        // 如果指定了最大进程数量限制，截取进程列表
        let processes_to_collect = if let Some(max) = self.max_processes {
            info!("采集进程数量已限制为 {}", max);
            processes.into_iter().take(max).collect::<Vec<_>>()
        } else {
            processes
        };

        // 并行处理进程
        let processed_processes: HashMap<i32, ProcessInfo> = processes_to_collect.par_iter()
            .filter_map(|(pid, _name)| {
                let executor_clone = executor.clone();
                let progress_clone = progress.clone();
                match self.collect_process_info_parallel(*pid, executor_clone) {
                    Ok(info) => {
                        let mut current = progress_clone.lock().unwrap();
                        *current += 1;
                        if *current % 10 == 0 || *current == total_count {
                            print!("\r进度: {}/{}  ({:.1}%)    ", *current, total_count, (*current as f64 / total_count as f64) * 100.0);
                            std::io::stdout().flush().unwrap();
                        }
                        debug!("成功收集进程信息 PID: {}", pid);
                        Some((*pid, info))
                    }
                    Err(e) => {
                        let mut current = progress_clone.lock().unwrap();
                        *current += 1;
                        if *current % 10 == 0 || *current == total_count {
                            print!("\r进度: {}/{}  ({:.1}%)    ", *current, total_count, (*current as f64 / total_count as f64) * 100.0);
                            std::io::stdout().flush().unwrap();
                        }
                        debug!("收集进程 {} 信息失败: {}", pid, e);
                        None
                    }
                }
            })
            .collect();

        let final_skipped_count = *skipped_count.lock().unwrap();
        info!("共跳过 {} 个系统进程", final_skipped_count);
        Ok((processed_processes, final_skipped_count))
    }

    fn collect_process_info_parallel(&self, pid: i32, executor: Arc<Mutex<LocalExecutor>>) -> Result<ProcessInfo> {
        debug!("收集进程信息 PID: {}", pid);

        let mut executor_guard = executor.lock().unwrap();
        let executor = &mut *executor_guard;

        // 获取进程基本信息和用户ID
        let (status, _) = executor.execute_sudo_command(
            &format!("cat /proc/{}/status 2>/dev/null", pid)
        )?;

        let name = status.lines()
            .find(|line| line.starts_with("Name:"))
            .map(|line| line.split_whitespace().nth(1).unwrap_or("unknown"))
            .unwrap_or("unknown")
            .to_string();

        // 从status中获取用户ID
        let user_id = status.lines()
            .find(|line| line.starts_with("Uid:"))
            .and_then(|line| line.split_whitespace().nth(1))  // 获取real uid
            .and_then(|uid| uid.parse::<u32>().ok())
            .unwrap_or_else(|| unsafe { libc::geteuid() });

        // 判断是否为内核线程
        let is_kthread = status.lines()
            .find(|line| line.starts_with("Kthread:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .map(|val| val == "1")
            .unwrap_or(false);

        // 获取可执行文件路径
        let (exe_path_str, _) = executor.execute_sudo_command(
            &format!("readlink -f /proc/{}/exe 2>/dev/null", pid)
        )?;
        let exe_path = PathBuf::from(exe_path_str.trim());

        // 获取可执行文件大小
        let exe_size = if exe_path.as_os_str().is_empty() {
            debug!("进程 {} 可能运行在独立namespace中，无法获取可执行文件路径", pid);
            0
        } else {
            match executor.execute_sudo_command(
                &format!("stat -c %s {:?} 2>/dev/null", exe_path)
            ) {
                Ok((size, _)) => size.trim().parse().unwrap_or(0),
                Err(e) => {
                    debug!("无法获取进程 {} 的可执行文件大小: {}", pid, e);
                    0
                }
            }
        };

        // 收集内存信息 (PSS/RSS)，单位从KB转换为字节
        let (mut pss, mut rss) = (0, 0);
        if let Ok((smaps, _)) = executor.execute_sudo_command(
            &format!("cat /proc/{}/smaps 2>/dev/null", pid)
        ) {
            for line in smaps.lines() {
                if line.starts_with("Pss:") {
                    let kb = line.split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                    pss += kb * 1024;  // KB转换为字节
                } else if line.starts_with("Rss:") {
                    let kb = line.split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                    rss += kb * 1024;  // KB转换为字节
                }
            }
        }

        // 获取动态库信息
        let libraries = self.collect_process_libraries_parallel(pid, executor)?;

        // 获取打开的文件数量
        let (fd_count, _) = executor.execute_sudo_command(
            &format!("ls -l /proc/{}/fd 2>/dev/null | wc -l", pid)
        )?;
        let open_files_count = fd_count.trim().parse().unwrap_or(0);

        // 获取共享内存大小
        let shared_memory = self.collect_process_shared_memory_parallel(pid, executor)?;

        Ok(ProcessInfo {
            pid,
            name,
            exe_path,
            exe_size,
            is_kthread,
            pss,
            rss,
            shared_memory,
            open_files_count,
            libraries,
            user_id,
        })
    }


    fn collect_process_libraries_parallel(
        &self,
        pid: i32,
        executor: &mut LocalExecutor
    ) -> Result<Vec<LibraryInfo>> {
        let mut libraries = Vec::new();

        // 获取动态库映射
        if let Ok((maps, _)) = executor.execute_sudo_command(
            &format!("cat /proc/{}/maps 2>/dev/null", pid)
        ) {
            let mut seen_paths = std::collections::HashSet::new();

            for line in maps.lines() {
                if line.contains(".so") && line.contains("/") {
                    let path = line.split_whitespace()
                        .last()
                        .map(PathBuf::from);

                    if let Some(path) = path {
                        if seen_paths.insert(path.clone()) {
                            let size = match executor.execute_sudo_command(
                                &format!("stat -c %s {:?} 2>/dev/null", path)
                            ) {
                                Ok((size_str, _)) => size_str.trim().parse().unwrap_or(0),
                                Err(e) => {
                                    debug!("无法获取动态库 {:?} 的大小: {}", path, e);
                                    0
                                }
                            };

                            libraries.push(LibraryInfo {
                                path,
                                size,
                                loaded_size: 0, // TODO: 计算实际加载大小
                            });
                        }
                    }
                }
            }
        }

        Ok(libraries)
    }

    fn collect_process_shared_memory_parallel(
        &self,
        pid: i32,
        executor: &mut LocalExecutor
    ) -> Result<u64> {
        let mut total = 0u64;
        if let Ok((shm, _)) = executor.execute_sudo_command(
            &format!("cat /proc/{}/maps | grep -i shm", pid)
        ) {
            for line in shm.lines() {
                if let Some(size) = parse_memory_range(line) {
                    total += size;
                }
            }
        }

        Ok(total)
    }

    fn collect_total_shared_memory(&mut self) -> Result<u64> {
        let mut total = 0u64;

        // 获取 SysV 共享内存
        if let Ok((ipcs, _)) = self.executor.lock().unwrap().execute_command("ipcs -m") {
            for line in ipcs.lines().skip(3) {
                if let Some(size) = line.split_whitespace().nth(4) {
                    if let Ok(bytes) = size.parse::<u64>() {
                        total += bytes;
                    }
                }
            }
        }

        Ok(total)
    }
}

fn parse_meminfo(content: &str) -> Result<(u64, u64)> {
    let mut total = 0;
    let mut available = 0;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(kb) = line.split_whitespace().nth(1) {
                total = kb.parse::<u64>()? * 1024;
            }
        } else if line.starts_with("MemAvailable:") {
            if let Some(kb) = line.split_whitespace().nth(1) {
                available = kb.parse::<u64>()? * 1024;
            }
        }
    }

    if total == 0 {
        anyhow::bail!("MemTotal not found in /proc/meminfo");
    }

    Ok((total, total - available))
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / 1024.0 / 1024.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn parse_memory_range(line: &str) -> Option<u64> {
    let parts: Vec<&str> = line.split_whitespace().next()?.split('-').collect();
    if parts.len() == 2 {
        if let (Ok(start), Ok(end)) = (
            u64::from_str_radix(parts[0], 16),
            u64::from_str_radix(parts[1], 16)
        ) {
            return Some(end - start);
        }
    }
    None
}
