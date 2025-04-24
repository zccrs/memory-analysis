use crate::collector::{CollectionResult, ProcessInfo};
use anyhow::Result;
use log::{info, debug};
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
            old_os_release: host1_data.system_info.os_release,
            new_os_release: host2_data.system_info.os_release,
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

        // 创建进程名和可执行文件到进程信息的映射
        let old_by_name_and_exe: HashMap<_, _> = old_processes
            .values()
            .map(|p| {
                let exe_base_name = Self::extract_base_name(&p.exe_path.to_string_lossy());
                ((p.name.clone(), exe_base_name), p.clone())
            })
            .collect();
        let new_by_name_and_exe: HashMap<_, _> = new_processes
            .values()
            .map(|p| {
                let exe_base_name = Self::extract_base_name(&p.exe_path.to_string_lossy());
                ((p.name.clone(), exe_base_name), p.clone())
            })
            .collect();

        // 查找新增的进程（确保同名进程不会重复计入）
        for ((name, exe_base), process) in &new_by_name_and_exe {
            // 检查是否在旧进程中存在同名同exe的进程
            let old_exists = old_by_name_and_exe.iter().any(|((old_name, old_exe), _)| {
                name == old_name && exe_base == old_exe
            });

            // 如果在旧进程中不存在，且不是简单的exe路径变化，则认为是新增进程
            if !old_exists {
                // 进一步检查是否只是exe路径变化了
                let similar_process_exists = old_by_name_and_exe.iter().any(|((old_name, _), _)| {
                    name == old_name
                });

                if !similar_process_exists {
                    diff.new_processes.insert(name.clone(), process.clone());
                }
            }
        }

        for ((name, exe_base), process) in &old_by_name_and_exe {
            let new_exists = new_by_name_and_exe.iter().any(|((new_name, new_exe), _)| {
                name == new_name && exe_base == new_exe
            });
            if !new_exists {
                diff.removed_processes.insert(name.clone(), process.clone());
            }
        }

        // 分析相同进程的变化
        for ((old_name, old_exe), old_proc) in &old_by_name_and_exe {
            if let Some(new_proc) = new_by_name_and_exe.iter()
                .find(|((new_name, new_exe), _)| old_name == new_name && old_exe == new_exe)
                .map(|(_, p)| p)
            {
                Self::analyze_process_diff(old_proc, new_proc, diff)?;
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

        // 只记录确实发生变化的进程
        if memory_diff != 0
            || !library_changes.is_empty()
            || new_proc.exe_size != old_proc.exe_size
            || new_proc.open_files_count != old_proc.open_files_count
            || new_proc.shared_memory != old_proc.shared_memory {
            diff.changed_processes.insert(
            old_proc.name.clone(),
            ProcessDiff {
                old_process: old_proc.clone(),
                new_process: new_proc.clone(),
                memory_diff,
                library_changes,
                exe_size_diff: new_proc.exe_size as i64 - old_proc.exe_size as i64,
                open_files_diff: new_proc.open_files_count as i32 - old_proc.open_files_count as i32,
                shared_memory_diff: new_proc.shared_memory as i64 - old_proc.shared_memory as i64,
            },
        );
        }

        Ok(())
    }

    pub fn format_bytes(bytes: i64) -> String {
        let byte = Byte::from_bytes(bytes.abs() as u128);
        let adjusted = byte.get_appropriate_unit(false);
        format!("{}{} {}", if bytes < 0 { "-" } else { "" }, adjusted.get_value(), adjusted.get_unit())
    }
}
