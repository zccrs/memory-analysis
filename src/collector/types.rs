use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessMemInfo {
    pub pss: u64,          // PSS (Proportional Set Size)
    pub rss: u64,          // RSS (Resident Set Size)
    pub private: u64,      // Private Memory
    pub shared: u64,       // Shared Memory
    pub swap: u64,         // Swap Usage
    pub cache: u64,        // Cache Memory
    pub text_size: u64,    // 代码段大小(.text)
    pub data_size: u64,    // 数据段大小(.data + .bss)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessFile {
    pub path: PathBuf,
    pub size: u64,
    pub access_mode: String, // 读/写/执行权限
    pub file_type: String,   // 普通文件/socket/pipe等
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub exe_path: PathBuf,
    pub exe_size: u64,
    pub is_kthread: bool,  // 用于标识内核线程
    pub mem_info: ProcessMemInfo,
    pub shared_memory: u64,
    pub open_files: Vec<ProcessFile>,
    pub libraries: Vec<LibraryInfo>,
    pub user_id: u32,  // 进程所属用户的ID
}

impl ProcessInfo {
    // 获取进程的唯一标识，内核线程使用name，普通进程使用exe_path
    pub fn hash_key(&self) -> (i32, String) {
        if self.is_kthread || self.exe_size == 0 {
            (self.pid, self.name.clone())
        } else {
            (self.pid, self.exe_path.to_string_lossy().to_string())
        }
    }

    pub fn hash_key_string(&self) -> String {
        let (pid, name) = self.hash_key();
        format!("{}:{}", pid, name)
    }
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
    pub meminfo: HashMap<String, u64>,  // /proc/meminfo 的所有字段
    pub kernel_config: HashMap<String, String>, // 内核编译选项
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionResult {
    pub system_info: SystemInfo,
    pub processes: HashMap<i32, ProcessInfo>,
}
