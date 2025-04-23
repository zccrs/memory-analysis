mod types;

use anyhow::{Context, Result};
use log::{debug, info};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::local::LocalExecutor;
pub use types::*;

pub struct Collector {
    executor: LocalExecutor,
    temp_dir: PathBuf,
}

impl Collector {
    pub fn new(temp_dir: PathBuf) -> Self {
        Self {
            executor: LocalExecutor::new(),
            temp_dir,
        }
    }

    pub fn collect(&mut self) -> Result<CollectionResult> {
        info!("开始收集系统信息...");

        // 创建临时目录
        std::fs::create_dir_all(&self.temp_dir)?;

        // 获取当前用户名和组ID
        let (whoami, _) = self.executor.execute_command("whoami")?;
        let username = whoami.trim();

        // 设置目录所有者为当前用户
        self.executor.execute_sudo_command(&format!("chown -R {}:{} {:?}", username, username, self.temp_dir))?;
        debug!("已将临时目录所有者设置为: {}", username);

        // 收集系统信息
        let mut system_info = self.collect_system_info()?;

        // 收集进程信息
        let (processes, skipped_count) = self.collect_processes()?;
        system_info.skipped_processes = skipped_count;

        // 计算进程内存总和
        let total_pss: u64 = processes.values()
            .map(|p| p.pss)
            .sum();
        system_info.processes_memory = total_pss * 1024;  // PSS单位是KB，转换为字节

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

        // 获取主机名
        let (hostname, _) = self.executor.execute_command("hostname")?;

        // 获取内核版本
        let (kernel_version, _) = self.executor.execute_sudo_command("uname -r")?;

        // 获取操作系统信息
        let (os_release, _) = self.executor.execute_command("cat /etc/os-release | grep PRETTY_NAME")?;
        let os_release = os_release
            .trim()
            .strip_prefix("PRETTY_NAME=")
            .unwrap_or("")
            .trim_matches('"')
            .to_string();

        // 获取CPU信息
        let (cpu_info, _) = self.executor.execute_command("cat /proc/cpuinfo | grep 'model name' | head -1")?;
        let cpu_info = cpu_info
            .trim()
            .strip_prefix("model name")
            .unwrap_or("")
            .trim_matches(|c| c == ':' || c == ' ')
            .to_string();

        // 获取页大小
        let (pagesize, _) = self.executor.execute_command("getconf PAGE_SIZE")?;

        // 获取内存信息
        let (meminfo, _) = self.executor.execute_sudo_command("cat /proc/meminfo")?;
        let (total_memory, used_memory) = parse_meminfo(&meminfo)
            .context("Failed to parse meminfo")?;

        // 获取共享内存总量
        let total_shared_memory = self.collect_total_shared_memory()?;

        // 获取内核文件大小
        let kernel_version_clean = kernel_version.trim();
        let mut kernel_file_size = 0;
        let mut initrd_file_size = 0;

        // 尝试不同的内核文件命名方式
        for kernel_path in &[
            format!("/boot/vmlinuz-{}", kernel_version_clean),
            format!("/boot/vmlinuz-linux"),
            format!("/boot/vmlinuz")
        ] {
            if let Ok((size, _)) = self.executor.execute_sudo_command(&format!("stat -c %s {} 2>/dev/null", kernel_path)) {
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
            if let Ok((size, _)) = self.executor.execute_sudo_command(&format!("stat -c %s {} 2>/dev/null", initrd_path)) {
                if let Ok(s) = size.trim().parse::<u64>() {
                    initrd_file_size = s;
                    break;
                }
            }
        }

        Ok(SystemInfo {
            hostname: hostname.trim().to_string(),
            kernel_version: kernel_version.trim().to_string(),
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
        let mut processes = HashMap::new();
        let mut skipped_count = 0;

        // 先获取进程总数
        let (process_count, _) = self.executor.execute_command(
            "ps -e | wc -l"
        )?;
        let total = process_count.trim().parse::<usize>().unwrap_or(0);
        info!("共发现 {} 个进程", total);

        // 获取进程列表
        let (ps_output, _) = self.executor.execute_command(
            "ps -e -o pid= -o comm= -o exe="
        )?;

        for line in ps_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let pid: i32 = parts[0].parse()?;
                let name = parts[1];

                // 跳过kworker、migration等系统进程
                if name.starts_with("kworker") ||
                   name.starts_with("migration") ||
                   name.starts_with("watchdog") ||
                   name.starts_with("ksoftirqd") ||
                   name.starts_with("scsi_") ||
                   name.starts_with("kthread") {
                    skipped_count += 1;
                    debug!("跳过系统进程: {} (PID: {})", name, pid);
                    continue;
                }

                info!("正在处理第 {} 个进程 (PID: {})", processes.len() + 1, pid);
                if let Ok(process) = self.collect_process_info(pid) {
                    processes.insert(pid, process);
                }
            }
        }

        info!("共跳过 {} 个系统进程", skipped_count);
        Ok((processes, skipped_count))
    }

    fn collect_process_info(&mut self, pid: i32) -> Result<ProcessInfo> {
        debug!("收集进程信息 PID: {}", pid);

        // 获取进程基本信息
        let (status, _) = self.executor.execute_sudo_command(
            &format!("cat /proc/{}/status 2>/dev/null", pid)
        )?;

        let name = status.lines()
            .find(|line| line.starts_with("Name:"))
            .map(|line| line.split_whitespace().nth(1).unwrap_or("unknown"))
            .unwrap_or("unknown")
            .to_string();

        // 获取可执行文件路径
        // 获取可执行文件路径，处理namespace隔离的情况
        let (exe_path_str, _) = self.executor.execute_sudo_command(
            &format!("readlink -f /proc/{}/exe 2>/dev/null", pid)
        )?;
        let exe_path = PathBuf::from(exe_path_str.trim());

        // 获取可执行文件大小，如果文件不可访问（比如在namespace中）则返回0
        let exe_size = if exe_path.as_os_str().is_empty() {
            debug!("进程 {} 可能运行在独立namespace中，无法获取可执行文件路径", pid);
            0
        } else {
            match self.executor.execute_sudo_command(
                &format!("stat -c %s {:?} 2>/dev/null", exe_path)
            ) {
                Ok((size, _)) => size.trim().parse().unwrap_or(0),
                Err(e) => {
                    debug!("无法获取进程 {} 的可执行文件大小: {}", pid, e);
                    0
                }
            }
        };

        // 收集内存信息 (PSS/RSS)
        let (mut pss, mut rss) = (0, 0);
        if let Ok((smaps, _)) = self.executor.execute_sudo_command(
            &format!("cat /proc/{}/smaps 2>/dev/null", pid)
        ) {
            for line in smaps.lines() {
                if line.starts_with("Pss:") {
                    pss += line.split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                } else if line.starts_with("Rss:") {
                    rss += line.split_whitespace()
                        .nth(1)
                        .unwrap_or("0")
                        .parse::<u64>()
                        .unwrap_or(0);
                }
            }
        }

        // 获取动态库信息
        let libraries = self.collect_process_libraries(pid)?;

        // 获取打开的文件数量
        let (fd_count, _) = self.executor.execute_sudo_command(
            &format!("ls -l /proc/{}/fd 2>/dev/null | wc -l", pid)
        )?;
        let open_files_count = fd_count.trim().parse().unwrap_or(0);

        // 获取共享内存大小
        let shared_memory = self.collect_process_shared_memory(pid)?;

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

    fn collect_process_libraries(&mut self, pid: i32) -> Result<Vec<LibraryInfo>> {
        let mut libraries = Vec::new();

        // 获取动态库映射
        if let Ok((maps, _)) = self.executor.execute_sudo_command(
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
                            let size = match self.executor.execute_sudo_command(
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

    fn collect_process_shared_memory(&mut self, pid: i32) -> Result<u64> {
        let mut total = 0u64;

        if let Ok((shm, _)) = self.executor.execute_sudo_command(
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
        if let Ok((ipcs, _)) = self.executor.execute_command("ipcs -m") {
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
