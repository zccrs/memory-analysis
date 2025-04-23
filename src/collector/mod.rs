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
    pub fn new(temp_dir: PathBuf) -> Self {
        Self {
            executor: Arc::new(Mutex::new(LocalExecutor::new())),
            temp_dir,
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

        // 计算进程内存总和（PSS已经是字节单位）
        system_info.processes_memory = processes.values()
            .map(|p| p.pss)
            .sum();

        // 计算内核占用的内存（总使用内存 - 进程内存总和）
        system_info.kernel_memory = system_info.used_memory.saturating_sub(system_info.processes_memory);

        info!("进程内存总量: {} MB", system_info.processes_memory / 1024 / 1024);
        info!("内核内存占用: {} MB", system_info.kernel_memory / 1024 / 1024);

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

            // 尝试不同的内核文件命名方式
            for kernel_path in &[
                format!("/boot/vmlinuz-{}", kernel_version_clean),
                format!("/boot/vmlinuz-linux"),
                format!("/boot/vmlinuz")
            ] {
                if let Ok((size, _)) = executor.execute_sudo_command(&format!("stat -c %s {} 2>/dev/null", kernel_path)) {
                    if let Ok(s) = size.trim().parse::<u64>() {
                        kernel_file_size = s;
                        break;
                    }
                }
            }

            // 尝试不同的initrd文件命名方式
            for initrd_path in &[
                format!("/boot/initramfs-{}.img", kernel_version_clean),
                format!("/boot/initramfs-linux.img"),
                format!("/boot/initrd.img")
            ] {
                if let Ok((size, _)) = executor.execute_sudo_command(&format!("stat -c %s {} 2>/dev/null", initrd_path)) {
                    if let Ok(s) = size.trim().parse::<u64>() {
                        initrd_file_size = s;
                        break;
                    }
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

        // 获取进程列表
        let ps_output = {
            let executor = self.executor.lock().unwrap();
            let (output, _) = executor.execute_command("ps -e -o pid= -o comm= -o exe=")?;
            output
        };

        // 将进程信息解析为Vec以便并行处理
        let processes: Vec<(i32, String)> = ps_output.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(pid) = parts[0].parse::<i32>() {
                        Some((pid, parts[1].to_string()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let total_count = processes.len();
        info!("共发现 {} 个进程", total_count);

        let progress = Arc::new(Mutex::new(0));

        // 并行处理所有进程
        let processed_processes: HashMap<i32, ProcessInfo> = processes.par_iter()
            .filter_map(|(pid, name)| {
                // 跳过系统进程
                if name.starts_with("kworker") ||
                   name.starts_with("migration") ||
                   name.starts_with("watchdog") ||
                   name.starts_with("ksoftirqd") ||
                   name.starts_with("scsi_") ||
                   name.starts_with("kthread") {
                    {
                        let mut count = skipped_count.lock().unwrap();
                        *count += 1;
                        let mut current = progress.lock().unwrap();
                        *current += 1;
                        if *current % 10 == 0 || *current == total_count {
                            print!("\r进度: {}/{}  ({:.1}%)    ", *current, total_count, (*current as f64 / total_count as f64) * 100.0);
                            std::io::stdout().flush().unwrap();
                        }
                        debug!("跳过系统进程: {} (PID: {})", name, pid);
                    }
                    None
                } else {
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
                            {
                                let mut current = progress_clone.lock().unwrap();
                                *current += 1;
                                if *current % 10 == 0 || *current == total_count {
                                    print!("\r进度: {}/{}  ({:.1}%)    ", *current, total_count, (*current as f64 / total_count as f64) * 100.0);
                                    std::io::stdout().flush().unwrap();
                                }
                            }
                            debug!("收集进程 {} 信息失败: {}", pid, e);
                            None
                        }
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

        // 获取进程基本信息
        let (status, _) = executor.execute_sudo_command(
            &format!("cat /proc/{}/status 2>/dev/null", pid)
        )?;

        let name = status.lines()
            .find(|line| line.starts_with("Name:"))
            .map(|line| line.split_whitespace().nth(1).unwrap_or("unknown"))
            .unwrap_or("unknown")
            .to_string();

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
            pss,
            rss,
            shared_memory,
            open_files_count,
            libraries,
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
