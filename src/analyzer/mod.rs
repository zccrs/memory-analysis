use crate::collector::{CollectionResult, ProcessInfo, SystemInfo};
use crate::reporter::Reporter;
use anyhow::Result;
use log::{debug, info};
use std::collections::HashMap;
use byte_unit::Byte;
use regex::Regex;

#[derive(Debug, serde::Serialize)]
pub struct MemoryDiff {
    pub total_diff: i64,
    pub new_processes: HashMap<String, ProcessInfo>,
    pub removed_processes: HashMap<String, ProcessInfo>,
    pub changed_processes: HashMap<String, ProcessDiff>,
    pub system_changes: SystemDiff,
    pub current_user_id: u32,
    pub old_os_release: String,  // 旧系统版本名称
    pub new_os_release: String,  // 新系统版本名称
    pub old_system_info: SystemInfo,  // 旧系统信息
    pub new_system_info: SystemInfo,  // 新系统信息
}

#[derive(Debug, serde::Serialize)]
pub struct ProcessDiff {
    pub old_process: ProcessInfo,
    pub new_process: ProcessInfo,
    pub memory_diff: i64,
    pub library_changes: Vec<LibraryChange>,
    pub exe_size_diff: i64,
    pub open_files_diff: i32,
    pub shared_memory_diff: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct LibraryChange {
    pub old_path: Option<String>,
    pub new_path: Option<String>,
    pub size_diff: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct SystemDiff {
    pub pagesize_diff: i64,
    pub kernel_version_changed: bool,
    pub shared_memory_diff: i64,
    pub old_kernel_size: Option<i64>,
    pub new_kernel_size: Option<i64>,
    pub old_initramfs_size: Option<i64>,
    pub new_initramfs_size: Option<i64>,
}

pub struct Analyzer;

impl Analyzer {
    // 提取文件的基本名称（只保留字母）
    fn extract_base_name(path: &str) -> String {
        lazy_static::lazy_static! {
            static ref ALPHA_RE: Regex = Regex::new(r"[a-zA-Z]+").unwrap();
        }

        // 获取文件名
        let file_name = path.split('/').last().unwrap_or(path);

        // 提取所有字母序列并连接
        ALPHA_RE.find_iter(file_name)
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join("")
            .to_lowercase()
    }

    pub fn analyze(host1_data: CollectionResult, host2_data: CollectionResult) -> Result<MemoryDiff> {
        info!("开始分析内存差异...");

        let mut diff = MemoryDiff {
            total_diff: 0,
            new_processes: HashMap::new(),
            removed_processes: HashMap::new(),
            changed_processes: HashMap::new(),
            system_changes: SystemDiff {
                pagesize_diff: host2_data.system_info.page_size as i64 - host1_data.system_info.page_size as i64,
                kernel_version_changed: host1_data.system_info.kernel_version != host2_data.system_info.kernel_version,
                shared_memory_diff: host2_data.system_info.total_shared_memory as i64
                    - host1_data.system_info.total_shared_memory as i64,
                old_kernel_size: Some(host1_data.system_info.kernel_file_size as i64),
                new_kernel_size: Some(host2_data.system_info.kernel_file_size as i64),
                old_initramfs_size: Some(host1_data.system_info.initrd_file_size as i64),
                new_initramfs_size: Some(host2_data.system_info.initrd_file_size as i64),
            },
            current_user_id: unsafe { libc::geteuid() },
            old_os_release: host1_data.system_info.os_release.clone(),
            new_os_release: host2_data.system_info.os_release.clone(),
            old_system_info: host1_data.system_info.clone(),
            new_system_info: host2_data.system_info.clone(),
        };

        // 分析进程变化
        Self::analyze_process_changes(
            &host1_data.processes,
            &host2_data.processes,
            &mut diff
        )?;

        // 计算总内存差异
        diff.total_diff = host2_data.system_info.used_memory as i64
            - host1_data.system_info.used_memory as i64;

        Ok(diff)
    }

    fn analyze_process_changes(
        old_processes: &HashMap<i32, ProcessInfo>,
        new_processes: &HashMap<i32, ProcessInfo>,
        diff: &mut MemoryDiff,
    ) -> Result<()> {
        debug!("分析进程变化...");

        // 创建进程映射，对于内核进程使用name，普通进程使用exe_path文件名（去掉数字）作为key
        let old_by_key: HashMap<_, _> = old_processes
            .values()
            .map(|p| {
                let key = if Reporter::is_kernel_process(p) {
                    p.name.clone()
                } else {
                    Self::extract_base_name(&p.exe_path.to_string_lossy())
                };
                ((p.pid, key), p.clone())
            })
            .collect();

        let new_by_key: HashMap<_, _> = new_processes
            .values()
            .map(|p| {
                let key = if Reporter::is_kernel_process(p) {
                    p.name.clone()
                } else {
                    Self::extract_base_name(&p.exe_path.to_string_lossy())
                };
                ((p.pid, key), p.clone())
            })
            .collect();

        debug!("原始的旧进程数量: {}, 原始的新进程数量: {}", old_processes.len(), new_processes.len());
        debug!("解析后的旧进程数量: {}, 解析后的新进程数量: {}", old_by_key.len(), new_by_key.len());

        let mut usedOldPids = Vec::new();
        // 遍历新进程，查找新增和变化的进程
        for (key, new_process) in &new_by_key {
            let mut same_process = false;
            let mut found_process = None;
            for (old_key, old_process) in &old_by_key {
                if key.1 == old_key.1 && !usedOldPids.contains(&old_key.0) {
                    found_process = Some(old_process);
                    same_process = true;
                    usedOldPids.push(old_key.0);
                    break;
                }
            }

            if same_process {
                Self::analyze_process_diff(found_process.unwrap(), new_process, diff)?;
            } else {
                // 新增进程
                // 使用进程的原始标识作为map的key
                diff.new_processes.insert(new_process.hash_key_string(), new_process.clone());
            }
        }

        // 查找已删除的进程
        for (key, old_process) in &old_by_key {
            if !usedOldPids.contains(&key.0) {
                // 使用进程的原始标识作为map的key
                diff.removed_processes.insert(old_process.hash_key_string(), old_process.clone());
            }
        }

        Ok(())
    }

    fn analyze_process_diff(
        old_proc: &ProcessInfo,
        new_proc: &ProcessInfo,
        diff: &mut MemoryDiff,
    ) -> Result<()> {
        let memory_diff = if new_proc.pss > 0 && old_proc.pss > 0 {
            new_proc.pss as i64 - old_proc.pss as i64
        } else {
            new_proc.rss as i64 - old_proc.rss as i64
        };

        let mut library_changes = Vec::new();

        // 使用基本名称创建动态库映射
        let old_libs: HashMap<_, _> = old_proc.libraries
            .iter()
            .map(|lib| {
                let base_name = Self::extract_base_name(&lib.path.to_string_lossy());
                (base_name, (lib.path.to_string_lossy().to_string(), lib))
            })
            .collect();

        let new_libs: HashMap<_, _> = new_proc.libraries
            .iter()
            .map(|lib| {
                let base_name = Self::extract_base_name(&lib.path.to_string_lossy());
                (base_name, (lib.path.to_string_lossy().to_string(), lib))
            })
            .collect();

        // 查找修改和删除的库
        for (base_name, (old_path, old_lib)) in &old_libs {
            if let Some((new_path, new_lib)) = new_libs.get(base_name) {
                if old_lib.size != new_lib.size {
                    library_changes.push(LibraryChange {
                        old_path: Some(old_path.clone()),
                        new_path: Some(new_path.clone()),
                        size_diff: new_lib.size as i64 - old_lib.size as i64,
                    });
                }
            } else {
                library_changes.push(LibraryChange {
                    old_path: Some(old_path.clone()),
                    new_path: None,
                    size_diff: -(old_lib.size as i64),
                });
            }
        }

        // 查找新增的库
        for (base_name, (new_path, new_lib)) in &new_libs {
            if !old_libs.contains_key(base_name) {
                library_changes.push(LibraryChange {
                    old_path: None,
                    new_path: Some(new_path.clone()),
                    size_diff: new_lib.size as i64,
                });
            }
        }

        diff.changed_processes.insert(
            format!("{} -> {}", old_proc.hash_key_string(), new_proc.hash_key_string()),
            ProcessDiff {
            old_process: old_proc.clone(),
            new_process: new_proc.clone(),
            memory_diff,
            library_changes,
            exe_size_diff: new_proc.exe_size as i64 - old_proc.exe_size as i64,
            open_files_diff: new_proc.open_files_count as i32 - old_proc.open_files_count as i32,
            shared_memory_diff: new_proc.shared_memory as i64 - old_proc.shared_memory as i64,
        },);

        Ok(())
    }

    pub fn format_bytes(bytes: i64) -> String {
        let byte = Byte::from_bytes(bytes.abs() as u128);
        let adjusted = byte.get_appropriate_unit(false);
        format!("{}{} {}", if bytes < 0 { "-" } else { "" }, adjusted.get_value(), adjusted.get_unit())
    }
}
