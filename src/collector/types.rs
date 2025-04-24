use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub exe_path: PathBuf,
    pub exe_size: u64,
    pub is_kthread: bool,  // 用于标识内核线程
    pub pss: u64,  // 优先使用 PSS
    pub rss: u64,  // 备选使用 RSS
    pub shared_memory: u64,
    pub open_files_count: usize,
    pub libraries: Vec<LibraryInfo>,
    pub user_id: u32,  // 进程所属用户的ID
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LibraryInfo {
    pub path: PathBuf,
    pub size: u64,
    pub loaded_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub kernel_version: String,
    pub os_release: String,
    pub cpu_info: String,
    pub page_size: u64,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_shared_memory: u64,
    pub kernel_memory: u64,  // 内核占用的内存（总内存 - 进程内存总和）
    pub processes_memory: u64,  // 所有进程占用的内存总和
    pub kernel_file_size: u64,  // 内核文件大小
    pub initrd_file_size: u64,  // initrd文件大小
    pub skipped_processes: usize,
    pub total_processes: usize,  // 系统中的总进程数
    pub collection_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionResult {
    pub system_info: SystemInfo,
    pub processes: HashMap<i32, ProcessInfo>,
}
