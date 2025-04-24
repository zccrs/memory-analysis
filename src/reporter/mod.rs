use crate::analyzer::{Analyzer, MemoryDiff};
use crate::collector::ProcessInfo;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use log::info;
use csv::Writer; // 添加 CSV 写入器

#[derive(Debug, Copy, Clone)]
pub enum ProcessChangeType {
    New,
    Removed,
    Changed,
}

pub struct Reporter;

impl Reporter {
    fn is_kernel_process(process: &ProcessInfo) -> bool {
        // 内核线程有两种判断方式：
        // 1. /proc/[pid]/status 中的 Kthread:1
        // 2. 可执行文件大小为 0
        process.is_kthread || process.exe_size == 0
    }

    pub fn generate_report(diff: &MemoryDiff, output_dir: &Path, old_identifier: &str, new_identifier: &str) -> Result<(PathBuf, PathBuf, PathBuf)> {
        info!("生成分析报告...");

        let json_path = output_dir.join("diff_report.json");
        let md_path = output_dir.join("diff_report_中文.md");
        let csv_path = output_dir.join("process_memory_changes.csv");

        // 生成 JSON 格式报告
        let json = serde_json::to_string_pretty(diff)?;
        fs::write(&json_path, json)?;
        crate::utils::fix_file_owner(&json_path)?;

        // 生成中文 Markdown 报告
        let markdown = Self::generate_markdown_report(diff, old_identifier, new_identifier)?;
        fs::write(&md_path, markdown)?;
        crate::utils::fix_file_owner(&md_path)?;

        // 生成 CSV 报告
        Self::generate_csv_report(diff, &csv_path)?;
        crate::utils::fix_file_owner(&csv_path)?;

        Ok((json_path, md_path, csv_path))
    }

    fn generate_markdown_report(diff: &MemoryDiff, old_identifier: &str, new_identifier: &str) -> Result<String> {
        let mut report = String::new();

        // 添加报告标题
        report.push_str("# 内存使用差异分析报告\n\n");

        // 添加名词解释
        report.push_str("## 名词解释\n\n");
        report.push_str("* **PSS (Proportional Set Size)**: 按比例分配的物理内存大小，将共享内存按使用比例分配给各个进程\n");
        report.push_str("* **RSS (Resident Set Size)**: 进程实际使用的物理内存大小，包含共享内存\n");
        report.push_str("* **新增进程**: 在新系统中出现，但在旧系统中不存在的进程\n");
        report.push_str("* **移除进程**: 在旧系统中存在，但在新系统中不存在的进程\n");
        report.push_str("* **内存占用变化**: \n");
        report.push_str("  * 🔴 红色表示内存增加或新增项目\n");
        report.push_str("  * 🟢 绿色表示内存减少或移除项目\n\n");

        // 添加统计方式说明
        report.push_str("## 关于内存统计方式\n\n");
        report.push_str("本工具使用以下方式统计内存：\n\n");
        report.push_str("1. **进程内存计算方式**\n");
        report.push_str("   - 优先使用 PSS (Proportional Set Size) 来计算进程内存\n");
        report.push_str("   - PSS 会将共享内存按比例分配给各个进程，更准确反映实际占用\n");
        report.push_str("   - 这与 top 命令使用的 RSS (Resident Set Size) 不同\n");
        report.push_str("   - RSS 会将共享内存完全计入每个进程，可能导致总和偏大\n\n");
        report.push_str("2. **数据来源**\n");
        report.push_str("   - 进程 PSS/RSS：/proc/{pid}/smaps\n");
        report.push_str("   - 系统总内存：/proc/meminfo\n");
        report.push_str("   - 进程内存总和 = 所有进程的 PSS 总和\n");
        report.push_str("   - 内核内存 = 系统总使用内存 - 进程内存总和\n\n");
        report.push_str("3. **统计范围说明**\n");
        report.push_str("   - 可通过 --max-processes 参数限制采集进程数量\n");
        report.push_str("   - 被跳过的进程数量会记录在统计信息中\n\n");

        report.push_str("4. **进程状态说明**\n");
        report.push_str("   - 新增进程：在新系统中出现，但在旧系统中不存在的进程\n");
        report.push_str("   - 移除进程：在旧系统中存在，但在新系统中不存在的进程\n");
        report.push_str("   - 变化进程：在新旧系统中都存在，但内存使用或其他特征发生变化的进程\n\n");

        // 使用 os_release 或备选名称作为系统标识
        let old_name = if !diff.old_os_release.trim().is_empty() {
            &diff.old_os_release
        } else if !old_identifier.trim().is_empty() {
            old_identifier
        } else {
            "旧系统"
        };

        let new_name = if !diff.new_os_release.trim().is_empty() {
            &diff.new_os_release
        } else if !new_identifier.trim().is_empty() {
            new_identifier
        } else {
            "新系统"
        };

        report.push_str(&format!("本报告是 {} 相对于 {} 的内存使用情况。\n\n", new_name, old_name));
        // 统计新旧系统的总进程数（包括被跳过的进程）
        let new_total = diff.new_processes.len() + diff.changed_processes.len() + diff.new_system_info.skipped_processes;
        let old_total = diff.removed_processes.len() + diff.changed_processes.len() + diff.old_system_info.skipped_processes;
        report.push_str(&format!("总进程数量（{}）：{}\n", new_name, new_total));
        report.push_str(&format!("总进程数量（{}）：{}\n", old_name, old_total));
        report.push_str(&format!("新增进程数量：{}\n", diff.new_processes.len()));
        report.push_str(&format!("移除进程数量：{}\n\n", diff.removed_processes.len()));

        report.push_str(&format!("本报告对比了 {} 和 {} 的内存使用情况。\n\n", old_name, new_name));

        // 统计新旧系统的基本信息
        report.push_str("# 综述\n\n");

        // 旧系统信息
        report.push_str(&format!("## {}\n\n", old_name));
        // 统计旧系统的进程总数（包括deleted进程和changed进程中的old进程）
        let old_total = diff.removed_processes.len() + diff.changed_processes.len();

        // 创建一个包含所有旧进程的迭代器
        let old_processes = diff.removed_processes.values()
            .map(|p| p as &ProcessInfo)
            .chain(diff.changed_processes.values().map(|p| &p.old_process));

        // 统计内核进程数量
        let old_kernel_count = old_processes
            .clone()
            .filter(|p| Self::is_kernel_process(p))
            .count();
        info!("{}内核进程数量：{}", old_identifier, old_kernel_count);

        // 统计系统进程数量（非用户进程）
        let old_system_count = old_processes
            .clone()
            .filter(|p| !Self::is_kernel_process(p) && p.user_id != diff.current_user_id)
            .count();

        // 统计用户进程数量
        let old_user_count = old_processes
            .filter(|p| !Self::is_kernel_process(p) && p.user_id == diff.current_user_id)
            .count();

        report.push_str(&format!("- CPU信息：{}\n", diff.old_system_info.cpu_info));
        report.push_str(&format!("- 主机名：{}\n", diff.old_system_info.hostname));
        report.push_str(&format!("- 内核版本：{}\n", diff.old_system_info.kernel_version));
        report.push_str(&format!("- 操作系统：{}\n", diff.old_system_info.os_release));
        report.push_str(&format!("- 页大小：{}\n", Analyzer::format_bytes(diff.old_system_info.page_size as i64)));
        report.push_str("- 系统内存：\n");
        report.push_str(&format!("  - 总内存：{}\n", Analyzer::format_bytes(diff.old_system_info.total_memory as i64)));
        report.push_str(&format!("  - 已使用内存：{}\n", Analyzer::format_bytes(diff.old_system_info.used_memory as i64)));
        report.push_str(&format!("  - 可用内存：{}\n", Analyzer::format_bytes((diff.old_system_info.total_memory - diff.old_system_info.used_memory) as i64)));
        report.push_str(&format!("  - 内核内存：{}\n", Analyzer::format_bytes(diff.old_system_info.kernel_memory as i64)));
        report.push_str(&format!("  - 进程总内存：{}\n", Analyzer::format_bytes(diff.old_system_info.processes_memory as i64)));
        report.push_str(&format!("  - 共享内存总量：{}\n", Analyzer::format_bytes(diff.old_system_info.total_shared_memory as i64)));
        report.push_str(&format!("- 进程信息：\n"));
        report.push_str(&format!("  - 总进程数：{}\n", old_total));
        report.push_str(&format!("  - 内核进程：{}\n", old_kernel_count));
        report.push_str(&format!("  - 系统进程：{} (非用户进程)\n", old_system_count));
        report.push_str(&format!("  - 用户进程：{}\n", old_user_count));
        report.push_str(&format!("  - 被跳过的进程：{}\n", diff.old_system_info.skipped_processes));
        report.push_str(&format!("- 内核文件信息：\n"));
        report.push_str(&format!("  - 内核文件大小：{}\n", Analyzer::format_bytes(diff.old_system_info.kernel_file_size as i64)));
        report.push_str(&format!("  - Initrd文件大小：{}\n\n", Analyzer::format_bytes(diff.old_system_info.initrd_file_size as i64)));

        // 新系统信息
        report.push_str(&format!("## {}\n\n", new_name));
        // 统计新系统中的实际总进程数
        let new_total = diff.new_processes.len() + diff.changed_processes.len() + diff.new_system_info.skipped_processes;

        // 获取所有新系统的进程
        // 包括新增的进程和更改后的进程
        let processes_new_system = diff.new_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.new_process))
            .collect::<Vec<_>>();

        // 统计不同类型的进程数量
        let new_kernel_count = processes_new_system.iter()
            .filter(|p| Self::is_kernel_process(p))
            .count();
        let new_system_count = processes_new_system.iter()
            .filter(|p| !Self::is_kernel_process(p) && p.user_id != diff.current_user_id)
            .count();
        let new_user_count = processes_new_system.iter()
            .filter(|p| !Self::is_kernel_process(p) && p.user_id == diff.current_user_id)
            .count();

        report.push_str(&format!("- CPU信息：{}\n", diff.new_system_info.cpu_info));
        report.push_str(&format!("- 主机名：{}\n", diff.new_system_info.hostname));
        report.push_str(&format!("- 内核版本：{}\n", diff.new_system_info.kernel_version));
        report.push_str(&format!("- 操作系统：{}\n", diff.new_system_info.os_release));
        report.push_str(&format!("- 页大小：{}\n", Analyzer::format_bytes(diff.new_system_info.page_size as i64)));
        report.push_str("- 系统内存：\n");
        report.push_str(&format!("  - 总内存：{}\n", Analyzer::format_bytes(diff.new_system_info.total_memory as i64)));
        report.push_str(&format!("  - 已使用内存：{}\n", Analyzer::format_bytes(diff.new_system_info.used_memory as i64)));
        report.push_str(&format!("  - 可用内存：{}\n", Analyzer::format_bytes((diff.new_system_info.total_memory - diff.new_system_info.used_memory) as i64)));
        report.push_str(&format!("  - 内核内存：{}\n", Analyzer::format_bytes(diff.new_system_info.kernel_memory as i64)));
        report.push_str(&format!("  - 进程总内存：{}\n", Analyzer::format_bytes(diff.new_system_info.processes_memory as i64)));
        report.push_str(&format!("  - 共享内存总量：{}\n", Analyzer::format_bytes(diff.new_system_info.total_shared_memory as i64)));
        report.push_str(&format!("- 进程信息：\n"));
        report.push_str(&format!("  - 总进程数：{}\n", new_total));
        report.push_str(&format!("  - 内核进程：{}\n", new_kernel_count));
        report.push_str(&format!("  - 系统进程：{} (非用户进程)\n", new_system_count));
        report.push_str(&format!("  - 用户进程：{}\n", new_user_count));
        report.push_str(&format!("  - 被跳过的进程：{}\n", diff.new_system_info.skipped_processes));
        report.push_str(&format!("- 内核文件信息：\n"));
        report.push_str(&format!("  - 内核文件大小：{}\n", Analyzer::format_bytes(diff.new_system_info.kernel_file_size as i64)));
        report.push_str(&format!("  - Initrd文件大小：{}\n\n", Analyzer::format_bytes(diff.new_system_info.initrd_file_size as i64)));

        // 计算各类进程的内存使用情况
        let mut kernel_total = 0i64;      // 内核线程
        let mut system_total = 0i64;      // 系统进程（非用户进程）
        let mut user_total = 0i64;       // 当前用户的进程
        let mut total_libs = 0i64;
        let mut total_exe = 0i64;
        let mut total_files = 0i64;
        let mut total_shared = 0i64;

        // 统计所有进程的内存使用（包括新增、删除和变化的进程）
        // 新增进程
        for process in diff.new_processes.values() {
            let mem = if process.pss > 0 { process.pss as i64 } else { process.rss as i64 };
            if Self::is_kernel_process(process) {
                kernel_total += mem;
            } else if process.user_id == diff.current_user_id {
                user_total += mem;
            } else {
                system_total += mem;
            }
        }

        // 移除进程
        for process in diff.removed_processes.values() {
            let mem = if process.pss > 0 { process.pss as i64 } else { process.rss as i64 };
            if Self::is_kernel_process(process) {
                kernel_total -= mem;
            } else if process.user_id == diff.current_user_id {
                user_total -= mem;
            } else {
                system_total -= mem;
            }
        }

        // 变化的进程
        for proc_diff in diff.changed_processes.values() {
            let mem_diff = proc_diff.memory_diff;
            if Self::is_kernel_process(&proc_diff.new_process) {
                kernel_total += mem_diff;
            } else if proc_diff.new_process.user_id == diff.current_user_id {
                user_total += mem_diff;
            } else {
                system_total += mem_diff;
            }
            total_libs += proc_diff.library_changes.iter().map(|l| l.size_diff).sum::<i64>();
            total_exe += proc_diff.exe_size_diff;
            total_files += proc_diff.open_files_diff as i64 * 4096; // 估算每个文件句柄 4KB
            total_shared += proc_diff.shared_memory_diff;
        }

        // 内存变化总览
        report.push_str("## 内存变化总览\n\n");
        report.push_str("```diff\n");

        // 计算进程部分总的内存变化
        let process_total = kernel_total + system_total + user_total;

        // 计算内核文件变化
        let kernel_files_diff = {
            let mut total = 0i64;
            if let (Some(old_kernel), Some(new_kernel)) = (diff.system_changes.old_kernel_size, diff.system_changes.new_kernel_size) {
                total += new_kernel - old_kernel;
            }
            if let (Some(old_initrd), Some(new_initrd)) = (diff.system_changes.old_initramfs_size, diff.system_changes.new_initramfs_size) {
                total += new_initrd - old_initrd;
            }
            total
        };

        // 计算其他部分的内存变化（系统总变化减去进程变化和内核文件变化）
        let other_total = diff.total_diff - process_total - kernel_files_diff;

        // 内核部分
        report.push_str("# 内核内存变化\n");

        // 内核进程变化
        let kernel_change_str = Analyzer::format_bytes(kernel_total);
        if kernel_total > 0 {
            report.push_str(&format!("+ 内核进程内存变化：{}\n", kernel_change_str));
        } else if kernel_total < 0 {
            report.push_str(&format!("- 内核进程内存变化：{}\n", kernel_change_str));
        } else {
            report.push_str(&format!("  内核进程内存变化：{}\n", kernel_change_str));
        }

        // 内核文件变化
        let kernel_files_str = Analyzer::format_bytes(kernel_files_diff);
        if kernel_files_diff > 0 {
            report.push_str(&format!("+ 内核文件变化：{}\n", kernel_files_str));
        } else if kernel_files_diff < 0 {
            report.push_str(&format!("- 内核文件变化：{}\n", kernel_files_str));
        } else {
            report.push_str(&format!("  内核文件变化：{}\n", kernel_files_str));
        }

        let total_kernel_change = kernel_total + kernel_files_diff;
        let total_kernel_str = Analyzer::format_bytes(total_kernel_change);
        if total_kernel_change != 0 {
            report.push_str(&format!("  总内核内存变化：{}\n", total_kernel_str));
        }

        // 系统部分
        report.push_str("\n# 系统内存变化\n");
        let system_change_str = Analyzer::format_bytes(system_total);
        if system_total > 0 {
            report.push_str(&format!("+ 系统进程内存变化：{}\n", system_change_str));
        } else if system_total < 0 {
            report.push_str(&format!("- 系统进程内存变化：{}\n", system_change_str));
        } else {
            report.push_str(&format!("  系统进程内存变化：{}\n", system_change_str));
        }

        // 用户部分
        report.push_str("\n# 用户内存变化\n");
        let user_change_str = Analyzer::format_bytes(user_total);
        if user_total > 0 {
            report.push_str(&format!("+ 用户进程内存变化：{}\n", user_change_str));
        } else if user_total < 0 {
            report.push_str(&format!("- 用户进程内存变化：{}\n", user_change_str));
        } else {
            report.push_str(&format!("  用户进程内存变化：{}\n", user_change_str));
        }

        // 其他系统内存变化（包括缓存、缓冲区等）
        report.push_str("\n# 其他系统内存变化\n");
        let other_change_str = Analyzer::format_bytes(other_total);
        if other_total > 0 {
            report.push_str(&format!("+ 其他内存变化：{}\n", other_change_str));
        } else if other_total < 0 {
            report.push_str(&format!("- 其他内存变化：{}\n", other_change_str));
        } else {
            report.push_str(&format!("  其他内存变化：{}\n", other_change_str));
        }

        // 总计
        report.push_str("\n-------------------\n");
        report.push_str(&format!("  总内存变化：  {}\n", Analyzer::format_bytes(diff.total_diff)));
        report.push_str("```\n\n");

        // 统计总的变化量
        let mut total_new = 0i64;
        let mut total_removed = 0i64;
        let mut total_changed = 0i64;

        // 计算新增进程的总内存
        for process in diff.new_processes.values() {
            total_new += if process.pss > 0 { process.pss as i64 } else { process.rss as i64 };
        }

        // 计算移除进程的总内存
        for process in diff.removed_processes.values() {
            total_removed -= if process.pss > 0 { process.pss as i64 } else { process.rss as i64 };
        }

        // 计算变化进程的总内存差异
        for proc_diff in diff.changed_processes.values() {
            total_changed += proc_diff.memory_diff;
        }

        // 计算总变化
        let process_memory_change = kernel_total + system_total + user_total;
        let libs_and_exe_change = total_libs + total_exe;
        let other_change = total_files + total_shared;
        let kernel_change = diff.total_diff - process_memory_change - libs_and_exe_change - other_change;

        report.push_str("**内存变化详细分析**\n\n");
        report.push_str("1. **进程内存变化**\n");
        report.push_str(&format!("   - 新增进程：{}\n", Analyzer::format_bytes(total_new)));
        report.push_str(&format!("   - 删除进程：{}\n", Analyzer::format_bytes(total_removed)));
        report.push_str(&format!("   - 现有进程变化：{}\n", Analyzer::format_bytes(total_changed)));
        report.push_str(&format!("   - 小计：{}\n", Analyzer::format_bytes(process_memory_change)));
        report.push_str("\n2. **可执行文件和动态库变化**\n");
        report.push_str(&format!("   - 动态库变化：{}\n", Analyzer::format_bytes(total_libs)));
        report.push_str(&format!("   - 可执行文件变化：{}\n", Analyzer::format_bytes(total_exe)));
        report.push_str(&format!("   - 小计：{}\n", Analyzer::format_bytes(libs_and_exe_change)));
        report.push_str("\n3. **其他资源变化**\n");
        report.push_str(&format!("   - 文件句柄：{}\n", Analyzer::format_bytes(total_files)));
        report.push_str(&format!("   - 共享内存：{}\n", Analyzer::format_bytes(total_shared)));
        report.push_str(&format!("   - 小计：{}\n", Analyzer::format_bytes(other_change)));
        report.push_str("\n4. **内核内存变化**\n");
        report.push_str(&format!("   - 内核空间：{}\n", Analyzer::format_bytes(kernel_change)));
        report.push_str("   - 包括：内核数据结构、缓存、内核模块等\n\n");

        // 系统配置变化
        report.push_str("## 系统配置差异\n\n");
        if diff.system_changes.kernel_version_changed {
            report.push_str("- ⚠️ 内核版本发生变化\n");
        }
        if diff.system_changes.pagesize_diff != 0 {
            report.push_str(&format!("- 页大小变化：{} 字节\n",
                diff.system_changes.pagesize_diff));
        }
        if diff.system_changes.shared_memory_diff != 0 {
            report.push_str(&format!("- 共享内存变化：{}\n",
                Analyzer::format_bytes(diff.system_changes.shared_memory_diff)));
        }
        report.push('\n');

        // 内存变化构成
        report.push_str("## 内存变化构成\n\n");
        report.push_str("### 总体变化分类\n\n");

        let categories = [
            ("新增进程", total_new),
            ("删除进程", total_removed),
            ("进程变化", total_changed),
            ("动态库变化", total_libs),
            ("可执行文件变化", total_exe),
            ("文件句柄变化", total_files),
            ("共享内存变化", total_shared),
        ];

        // 生成变化分类的表格
        report.push_str("| 变化类型 | 内存变化 | 占比 |\n");
        report.push_str("|---------|----------|------|\n");

        let total_abs = categories.iter()
            .map(|(_, size)| size.abs())
            .sum::<i64>();

        for (category, size) in categories.iter() {
            if *size != 0 {
                let percentage = (size.abs() as f64 / total_abs as f64 * 100.0).round();
                report.push_str(&format!("| {} | {} | {:.1}% |\n",
                    category,
                    Analyzer::format_bytes(*size),
                    percentage
                ));
            }
        }
        report.push('\n');

        // 分类详细信息
        report.push_str("# 详细内存分析\n\n");

        // 内核部分详细信息
        report.push_str("## 内核内存变化\n\n");

        // 内核文件变化
        report.push_str("### 内核文件变化\n\n");
        report.push_str("| 文件 | 原大小 | 新大小 | 变化 |\n");
        report.push_str("|------|---------|---------|-------|\n");
        if let Some(old_vmlinuz) = diff.system_changes.old_kernel_size {
            if let Some(new_vmlinuz) = diff.system_changes.new_kernel_size {
                let change = new_vmlinuz - old_vmlinuz;
                let change_color = if change > 0 { "🔴" } else { "🟢" };
                report.push_str(&format!("| vmlinuz | {} | {} | {} {} |\n",
                    Analyzer::format_bytes(old_vmlinuz),
                    Analyzer::format_bytes(new_vmlinuz),
                    change_color,
                    Analyzer::format_bytes(change)));
            }
        }
        if let Some(old_initramfs) = diff.system_changes.old_initramfs_size {
            if let Some(new_initramfs) = diff.system_changes.new_initramfs_size {
                let change = new_initramfs - old_initramfs;
                let change_color = if change > 0 { "🔴" } else { "🟢" };
                report.push_str(&format!("| initramfs | {} | {} | {} {} |\n",
                    Analyzer::format_bytes(old_initramfs),
                    Analyzer::format_bytes(new_initramfs),
                    change_color,
                    Analyzer::format_bytes(change)));
            }
        }
        report.push_str("\n");

        // 内核线程变化
        report.push_str("### 内核线程变化\n\n");
        let kernel_processes: Vec<_> = diff.new_processes.iter()
            .filter(|(_, p)| Self::is_kernel_process(p))
            .collect();
        let removed_kernel_processes: Vec<_> = diff.removed_processes.iter()
            .filter(|(_, p)| Self::is_kernel_process(p))
            .collect();

        if !kernel_processes.is_empty() {
            report.push_str("#### 新增内核线程\n\n");
            for (name, _process) in kernel_processes {
                report.push_str(&format!("🔴 {}\n", name));
            }
            report.push_str("\n");
        }

        if !removed_kernel_processes.is_empty() {
            report.push_str("#### 移除内核线程\n\n");
            for (name, _process) in removed_kernel_processes {
                report.push_str(&format!("🟢 {}\n", name));
            }
            report.push_str("\n");
        }

        // 对进程按类别分类
        let mut kernel_processes = Vec::new();
        let mut system_processes = Vec::new();
        let mut user_processes = Vec::new();

        // 收集所有进程信息并分类
        for (name, process) in &diff.new_processes {
            let mem = if process.pss > 0 { process.pss } else { process.rss } as i64;
            let entry = (name, mem, ProcessChangeType::New, process);
            if Self::is_kernel_process(process) {
                kernel_processes.push(entry);
            } else if process.user_id == diff.current_user_id {
                user_processes.push(entry);
            } else {
                system_processes.push(entry);
            }
        }
        for (name, process) in &diff.removed_processes {
            let mem = if process.pss > 0 { process.pss } else { process.rss } as i64;
            let entry = (name, -mem, ProcessChangeType::Removed, process);
            if Self::is_kernel_process(process) {
                kernel_processes.push(entry);
            } else if process.user_id == diff.current_user_id {
                user_processes.push(entry);
            } else {
                system_processes.push(entry);
            }
        }
        for (name, proc_diff) in &diff.changed_processes {
            let entry = (name, proc_diff.memory_diff, ProcessChangeType::Changed, &proc_diff.new_process);
            if Self::is_kernel_process(&proc_diff.new_process) {
                kernel_processes.push(entry);
            } else if proc_diff.new_process.user_id == diff.current_user_id {
                user_processes.push(entry);
            } else {
                system_processes.push(entry);
            }
        }

        // 对每个类别内的进程按内存变化大小排序
        kernel_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());
        system_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());
        user_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());

        // 系统进程详细信息
        report.push_str("## 系统进程变化\n\n");
        Self::write_process_details(&mut report, &system_processes);

        // 用户进程详细信息
        report.push_str("## 用户进程变化\n\n");
        Self::write_process_details(&mut report, &user_processes);

        // 对所有进程进行排序
        let mut sorted_processes = Vec::new();
        for (name, process) in &diff.new_processes {
            let mem = if process.pss > 0 { process.pss } else { process.rss } as i64;
            sorted_processes.push((name, mem, ProcessChangeType::New, process));
        }
        for (name, process) in &diff.removed_processes {
            let mem = if process.pss > 0 { process.pss } else { process.rss } as i64;
            sorted_processes.push((name, -mem, ProcessChangeType::Removed, process));
        }
        for (name, proc_diff) in &diff.changed_processes {
            sorted_processes.push((name, proc_diff.memory_diff, ProcessChangeType::Changed, &proc_diff.new_process));
        }

        // 按内存变化大小排序（绝对值降序）
        sorted_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());

        // 输出所有进程变化信息

        // 变化的进程的详细信息
        if !diff.changed_processes.is_empty() {
            report.push_str("### 进程详情\n\n");
            for (name, proc_diff) in &diff.changed_processes {
                report.push_str(&format!("#### {}\n", name));
                report.push_str(&format!("- 内存使用：{} -> {} (变化：{})\n",
                    Analyzer::format_bytes(if proc_diff.old_process.pss > 0 { proc_diff.old_process.pss as i64 } else { proc_diff.old_process.rss as i64 }),
                    Analyzer::format_bytes(if proc_diff.new_process.pss > 0 { proc_diff.new_process.pss as i64 } else { proc_diff.new_process.rss as i64 }),
                    Analyzer::format_bytes(proc_diff.memory_diff)));

                if proc_diff.exe_size_diff != 0 {
                    report.push_str(&format!("- 可执行文件大小变化：{}\n",
                        Analyzer::format_bytes(proc_diff.exe_size_diff)));
                }

                if proc_diff.open_files_diff != 0 {
                    report.push_str(&format!("- 打开文件数变化：{:+}\n",
                        proc_diff.open_files_diff));
                }

                // 动态库变化
                if !proc_diff.library_changes.is_empty() {
                    report.push_str("- 动态库变化：\n");
                    for lib in &proc_diff.library_changes {
                        match (&lib.old_path, &lib.new_path) {
                            (Some(old), Some(new)) if old == new => {
                if old == new {
                    report.push_str(&format!("  - 库大小变化 {}：{}\n",
                        old,
                        Analyzer::format_bytes(lib.size_diff)));
                } else {
                    report.push_str(&format!("  - 库路径变更并且大小变化：\n    - 原路径：{}\n    - 新路径：{}\n    - 大小变化：{}\n",
                        old, new, Analyzer::format_bytes(lib.size_diff)));
                }
                            }
                            (Some(old), None) => {
                                report.push_str(&format!("  - 移除库 {}\n", old));
                            }
                            (None, Some(new)) => {
                                report.push_str(&format!("  - 新增库 {}\n", new));
                            }
                            _ => {}
                        }
                    }
                }
                report.push('\n');
            }
        }

        Ok(report)
    }

    fn write_process_details(report: &mut String, processes: &[(&String, i64, ProcessChangeType, &ProcessInfo)]) {
        // 根据ProcessChangeType对进程进行分类
        let mut new_procs = Vec::new();
        let mut removed_procs = Vec::new();
        let mut changed_procs = Vec::new();

        for &(name, mem, change_type, process) in processes {
            match change_type {
                ProcessChangeType::New => new_procs.push((name, mem, process)),
                ProcessChangeType::Removed => removed_procs.push((name, mem, process)),
                ProcessChangeType::Changed => changed_procs.push((name, mem, process)),
            }
        }

        // 输出新增进程信息
        if !new_procs.is_empty() {
            report.push_str("### 新增进程\n\n");
            for (name, mem, process) in new_procs {
                report.push_str(&format!("#### {} (🔴 +{})\n", name, Analyzer::format_bytes(mem.abs())));
                report.push_str(&format!("- 可执行文件路径：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n", process.libraries.len()));
                report.push_str("\n");
            }
        }

        // 输出移除进程信息
        if !removed_procs.is_empty() {
            report.push_str("### 移除进程\n\n");
            for (name, mem, process) in removed_procs {
                report.push_str(&format!("#### {} (🟢 {})\n", name, Analyzer::format_bytes(-mem.abs())));
                report.push_str(&format!("- 可执行文件路径：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n", process.libraries.len()));
                report.push_str("\n");
            }
        }

        // 输出变化进程信息
        if !changed_procs.is_empty() {
            report.push_str("### 变化进程\n\n");
            for (name, mem, process) in changed_procs {
                let change_color = if mem > 0 { "🔴" } else { "🟢" };
                report.push_str(&format!("#### {} ({} {})\n", name, change_color, Analyzer::format_bytes(mem)));
                report.push_str(&format!("- 可执行文件路径：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n", process.libraries.len()));
                report.push_str("\n");
            }
        }
    }

    fn bytes_to_mb(bytes: i64) -> String {
        format!("{:.2}", bytes as f64 / (1024.0 * 1024.0))
    }

    fn generate_csv_report(diff: &MemoryDiff, csv_path: &Path) -> Result<()> {
        let mut wtr = Writer::from_path(csv_path)?;

        // 写入表头
        wtr.write_record(&["进程名", "旧内存占用 (MB)", "新内存占用 (MB)", "内存变化 (MB)"])?;

        // 新增进程
        for (name, process) in &diff.new_processes {
            let mem = if process.pss > 0 {
                process.pss
            } else {
                process.rss
            };
            wtr.write_record(&[name, "0", &Self::bytes_to_mb(mem as i64), &Self::bytes_to_mb(mem as i64)])?;
        }

        // 已删除进程
        for (name, process) in &diff.removed_processes {
            let mem = if process.pss > 0 {
                process.pss
            } else {
                process.rss
            };
            wtr.write_record(&[name, &Self::bytes_to_mb(mem as i64), "0", &Self::bytes_to_mb(-(mem as i64))])?;
        }

        // 变化的进程
        for (name, proc_diff) in &diff.changed_processes {
            let old_mem = if proc_diff.old_process.pss > 0 {
                proc_diff.old_process.pss
            } else {
                proc_diff.old_process.rss
            };
            let new_mem = if proc_diff.new_process.pss > 0 {
                proc_diff.new_process.pss
            } else {
                proc_diff.new_process.rss
            };
            let mem_change = (new_mem as i64) - (old_mem as i64);
            wtr.write_record(&[name, &Self::bytes_to_mb(old_mem as i64), &Self::bytes_to_mb(new_mem as i64), &Self::bytes_to_mb(mem_change)])?;
        }

        wtr.flush()?;
        Ok(())
    }
}
