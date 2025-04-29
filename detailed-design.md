# 内存分析工具 - 详细设计

## 1. 数据结构设计

### 1.1 进程信息结构

```rust
pub struct ProcessInfo {
    // 基本信息
    pub pid: i32,                    // 进程ID
    pub name: String,               // 进程名称
    pub user_id: u32,              // 用户ID
    pub is_kthread: bool,          // 是否为内核线程

    // 内存信息
    pub mem_info: MemoryInfo,      // 内存使用信息
    pub open_files: Vec<String>,   // 打开的文件列表
    pub libraries: Vec<Library>,    // 加载的动态库
    pub exe_path: PathBuf,         // 可执行文件路径
    pub exe_size: u64,             // 可执行文件大小
}

pub struct MemoryInfo {
    pub rss: u64,                  // 常驻内存大小
    pub pss: u64,                  // 比例共享内存
    pub shared: u64,               // 共享内存大小
    pub swap: u64,                 // 交换空间使用
}

pub struct Library {
    pub path: PathBuf,             // 库文件路径
    pub size: u64,                 // 库文件大小
}
```

### 1.2 差异分析结构

```rust
pub struct MemoryDiff {
    // 系统信息
    pub old_os_release: String,    // 旧系统版本
    pub new_os_release: String,    // 新系统版本
    pub current_user_id: u32,      // 当前用户ID

    // 进程变化
    pub new_processes: HashMap<String, ProcessInfo>,     // 新增进程
    pub removed_processes: HashMap<String, ProcessInfo>, // 移除进程
    pub changed_processes: HashMap<String, ProcessDiff>, // 变化进程

    // 总体变化
    pub total_diff: i64,           // 总内存变化
}

pub struct ProcessDiff {
    pub old_process: ProcessInfo,  // 旧进程信息
    pub new_process: ProcessInfo,  // 新进程信息
    pub memory_diff: i64,          // 内存变化
    pub exe_size_diff: i64,        // 可执行文件大小变化
    pub open_files_diff: i32,      // 打开文件数变化
    pub library_changes: Vec<LibraryChange>, // 动态库变化
}

pub struct LibraryChange {
    pub old_path: Option<PathBuf>, // 旧库路径
    pub new_path: Option<PathBuf>, // 新库路径
    pub size_diff: i64,            // 大小变化
}
```

## 2. 模块实现详解

### 2.1 数据采集模块 (Collector)

#### 2.1.1 本地数据采集
1. 遍历 /proc 目录获取进程列表
2. 对每个进程：
   - 解析 /proc/[pid]/status 获取基本信息
   - 解析 /proc/[pid]/smaps 获取详细内存信息
   - 解析 /proc/[pid]/maps 获取内存映射
   - 读取 /proc/[pid]/fd/ 获取打开文件
3. 采集系统信息：
   - 读取 /etc/os-release
   - 获取系统内存统计

#### 2.1.2 远程数据采集
1. 通过 SSH 连接远程主机
2. 在远程主机上：
   - 复制工具程序
   - 执行数据采集
   - 获取采集结果
3. 清理远程临时文件

### 2.2 数据分析模块 (Analyzer)

#### 2.2.1 进程匹配算法
1. 按以下优先级匹配进程：
   - PID + 可执行文件路径完全匹配
   - 进程名 + 可执行文件路径匹配
   - 进程名 + 用户ID匹配

#### 2.2.2 内存差异计算
1. 计算 PSS 差异：
   ```rust
   fn calculate_memory_diff(old: &ProcessInfo, new: &ProcessInfo) -> i64 {
       let old_mem = if old.mem_info.pss > 0 {
           old.mem_info.pss
       } else {
           old.mem_info.rss
       } as i64;

       let new_mem = if new.mem_info.pss > 0 {
           new.mem_info.pss
       } else {
           new.mem_info.rss
       } as i64;

       new_mem - old_mem
   }
   ```

#### 2.2.3 进程分类算法
1. 内核进程判断：
   ```rust
   fn is_kernel_process(process: &ProcessInfo) -> bool {
       process.is_kthread || process.exe_size == 0
   }
   ```

2. 系统进程判断：
   ```rust
   fn is_system_process(process: &ProcessInfo) -> bool {
       let system_paths = ["/usr/bin/", "/usr/sbin/", "/lib/", "/bin/"];
       system_paths.iter().any(|path| process.exe_path.starts_with(path))
   }
   ```

### 2.3 报告生成模块 (Reporter)

#### 2.3.1 JSON 报告生成
- 使用 serde_json 序列化完整数据
- 保持原始数据结构
- 便于后续程序处理

#### 2.3.2 Markdown 报告生成
1. 报告结构：
   - 系统概述
   - 进程变化统计
   - 内存变化概要
   - 内核进程变化
   - 系统进程变化
   - 用户进程变化
   - 详细进程变化

2. 格式化函数：
   ```rust
   fn format_bytes(bytes: i64) -> String {
       let abs = bytes.abs() as f64;
       if abs < 1024.0 {
           format!("{:+} B", bytes)
       } else if abs < 1024.0 * 1024.0 {
           format!("{:+.1} KB", bytes as f64 / 1024.0)
       } else if abs < 1024.0 * 1024.0 * 1024.0 {
           format!("{:+.1} MB", bytes as f64 / (1024.0 * 1024.0))
       } else {
           format!("{:+.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
       }
   }
   ```

#### 2.3.3 HTML 报告生成
1. 页面结构：
   ```html
   <!DOCTYPE html>
   <html lang="zh">
   <head>
       <meta charset="UTF-8">
       <meta name="viewport" content="width=device-width, initial-scale=1.0">
       <title>内存使用差异分析报告</title>
       <style>
           /* 响应式布局样式 */
       </style>
   </head>
   <body>
       <!-- 报告内容 -->
   </body>
   </html>
   ```

2. 视觉样式：
   - 使用响应式栅格系统
   - 清晰的层次结构
   - 颜色编码区分增减
   - 适配移动设备

## 3. 错误处理

### 3.1 错误类型设计
```rust
#[derive(Debug)]
pub enum AnalysisError {
    IoError(std::io::Error),
    ParseError(String),
    ProcessError(String),
    RemoteError(String),
}
```

### 3.2 错误处理策略
1. 采集阶段：
   - 单进程错误不影响整体采集
   - 记录错误但继续执行
   - 保存部分采集结果

2. 分析阶段：
   - 数据完整性检查
   - 错误数据跳过处理
   - 记录异常情况

3. 报告生成：
   - 错误信息包含在报告中
   - 保证报告格式正确
   - 提供错误详情

## 4. 性能优化

### 4.1 并行采集优化
```rust
pub async fn parallel_collect(processes: Vec<i32>) -> Vec<ProcessInfo> {
    let mut handles = Vec::new();
    for pid in processes {
        handles.push(tokio::spawn(async move {
            collect_process_info(pid).await
        }));
    }
    join_all(handles).await
}
```

### 4.2 内存使用优化
1. 流式处理大文件
2. 使用迭代器避免中间集合
3. 重用缓冲区
4. 及时释放资源

### 4.3 IO优化
1. 使用异步IO
2. 批量读取文件
3. 缓存常用数据
4. 减少文件操作

## 5. 测试策略

### 5.1 单元测试
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_process_matching() {
        // 测试进程匹配逻辑
    }

    #[test]
    fn test_memory_diff_calculation() {
        // 测试内存差异计算
    }
}
```

### 5.2 集成测试
1. 测试完整工作流
2. 验证报告生成
3. 检查错误处理

### 5.3 性能测试
1. 大规模数据测试
2. 并发性能测试
3. 资源使用监控
