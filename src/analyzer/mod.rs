use crate::collector::{CollectionResult, ProcessInfo};
use anyhow::Result;
use log::{info, debug};
use std::collections::HashMap;
use byte_unit::Byte;

#[derive(Debug, serde::Serialize)]
pub struct MemoryDiff {
    pub total_diff: i64,
    pub new_processes: HashMap<String, ProcessInfo>,
    pub removed_processes: HashMap<String, ProcessInfo>,
    pub changed_processes: HashMap<String, ProcessDiff>,
    pub system_changes: SystemDiff,
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
}

pub struct Analyzer;

impl Analyzer {
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
            },
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

        // 创建进程名到进程信息的映射
        let old_by_name: HashMap<_, _> = old_processes
            .values()
            .map(|p| (p.name.clone(), p.clone()))
            .collect();

        let new_by_name: HashMap<_, _> = new_processes
            .values()
            .map(|p| (p.name.clone(), p.clone()))
            .collect();

        // 查找新增和删除的进程
        for (name, process) in &new_by_name {
            if !old_by_name.contains_key(name) {
                diff.new_processes.insert(name.clone(), process.clone());
            }
        }

        for (name, process) in &old_by_name {
            if !new_by_name.contains_key(name) {
                diff.removed_processes.insert(name.clone(), process.clone());
            }
        }

        // 分析相同名称进程的变化
        for (name, old_proc) in &old_by_name {
            if let Some(new_proc) = new_by_name.get(name) {
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

        // 分析动态库变化
        let old_libs: HashMap<_, _> = old_proc.libraries
            .iter()
            .map(|lib| (lib.path.to_string_lossy().to_string(), lib))
            .collect();

        let new_libs: HashMap<_, _> = new_proc.libraries
            .iter()
            .map(|lib| (lib.path.to_string_lossy().to_string(), lib))
            .collect();

        // 查找修改和删除的库
        for (path, old_lib) in &old_libs {
            if let Some(new_lib) = new_libs.get(path) {
                if old_lib.size != new_lib.size {
                    library_changes.push(LibraryChange {
                        old_path: Some(path.clone()),
                        new_path: Some(path.clone()),
                        size_diff: new_lib.size as i64 - old_lib.size as i64,
                    });
                }
            } else {
                library_changes.push(LibraryChange {
                    old_path: Some(path.clone()),
                    new_path: None,
                    size_diff: -(old_lib.size as i64),
                });
            }
        }

        // 查找新增的库
        for (path, new_lib) in &new_libs {
            if !old_libs.contains_key(path) {
                library_changes.push(LibraryChange {
                    old_path: None,
                    new_path: Some(path.clone()),
                    size_diff: new_lib.size as i64,
                });
            }
        }

        // 记录所有进程的变化情况，包括无变化的进程
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

        Ok(())
    }

    pub fn format_bytes(bytes: i64) -> String {
        let byte = Byte::from_bytes(bytes.abs() as u128);
        let adjusted = byte.get_appropriate_unit(false);
        format!("{}{} {}", if bytes < 0 { "-" } else { "" }, adjusted.get_value(), adjusted.get_unit())
    }
}
