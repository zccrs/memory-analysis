use crate::analyzer::{Analyzer, MemoryDiff};
use crate::collector::ProcessInfo;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use log::info;
use csv::Writer; // 添加 CSV 写入器

pub struct Reporter;

impl Reporter {
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
        report.push_str(&format!("总进程数量（{}）：{}\n", new_name, diff.new_processes.len() + diff.changed_processes.len()));
        report.push_str(&format!("总进程数量（{}）：{}\n", old_name, diff.removed_processes.len() + diff.changed_processes.len()));
        report.push_str(&format!("新增进程数量：{}\n", diff.new_processes.len()));
        report.push_str(&format!("移除进程数量：{}\n\n", diff.removed_processes.len()));

        report.push_str(&format!("本报告对比了 {} 和 {} 的内存使用情况。\n\n", old_name, new_name));

        // 统计新旧系统的基本信息
        report.push_str("# 综述\n\n");

        // 旧系统信息
        report.push_str(&format!("## {}\n\n", old_name));
        let old_total = diff.removed_processes.len() + diff.changed_processes.len();
        let old_kernel_count = diff.removed_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.old_process))
            .filter(|p| p.exe_path.to_string_lossy().contains("kernel"))
            .count();
        let old_system_count = diff.removed_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.old_process))
            .filter(|p| !p.exe_path.to_string_lossy().contains("kernel") && p.user_id != diff.current_user_id)
            .count();
        let old_user_count = diff.removed_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.old_process))
            .filter(|p| p.user_id == diff.current_user_id)
            .count();

        report.push_str(&format!("- 总进程数：{}\n", old_total));
        report.push_str(&format!("  - 内核进程：{}\n", old_kernel_count));
        report.push_str(&format!("  - 系统进程：{}\n", old_system_count));
        report.push_str(&format!("  - 用户进程：{}\n\n", old_user_count));

        // 新系统信息
        report.push_str(&format!("## {}\n\n", new_name));
        let new_total = diff.new_processes.len() + diff.changed_processes.len();
        let new_kernel_count = diff.new_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.new_process))
            .filter(|p| p.exe_path.to_string_lossy().contains("kernel"))
            .count();
        let new_system_count = diff.new_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.new_process))
            .filter(|p| !p.exe_path.to_string_lossy().contains("kernel") && p.user_id != diff.current_user_id)
            .count();
        let new_user_count = diff.new_processes.values()
            .chain(diff.changed_processes.values().map(|p| &p.new_process))
            .filter(|p| p.user_id == diff.current_user_id)
            .count();

        report.push_str(&format!("- 总进程数：{}\n", new_total));
        report.push_str(&format!("  - 内核进程：{}\n", new_kernel_count));
        report.push_str(&format!("  - 系统进程：{}\n", new_system_count));
        report.push_str(&format!("  - 用户进程：{}\n\n", new_user_count));

        // 计算各类进程的内存变化
        let mut kernel_new = 0i64;
        let mut kernel_removed = 0i64;
        let mut kernel_changed = 0i64;
        let mut system_new = 0i64;
        let mut system_removed = 0i64;
        let mut system_changed = 0i64;
        let mut user_new = 0i64;
        let mut user_removed = 0i64;
        let mut user_changed = 0i64;
        let mut total_libs = 0i64;
        let mut total_exe = 0i64;
        let mut total_files = 0i64;
        let mut total_shared = 0i64;

        // 统计新增进程
        for process in diff.new_processes.values() {
            let mem = if process.pss > 0 { process.pss as i64 } else { process.rss as i64 };
            if process.exe_path.to_string_lossy().contains("kernel") {
                kernel_new += mem;
            } else if process.user_id == diff.current_user_id {
                user_new += mem;
            } else {
                system_new += mem;
            }
        }

        // 统计已删除进程
        for process in diff.removed_processes.values() {
            let mem = if process.pss > 0 { process.pss as i64 } else { process.rss as i64 };
            if process.exe_path.to_string_lossy().contains("kernel") {
                kernel_removed -= mem;
            } else if process.user_id == diff.current_user_id {
                user_removed -= mem;
            } else {
                system_removed -= mem;
            }
        }

        // 统计变化的进程
        for proc_diff in diff.changed_processes.values() {
            if proc_diff.new_process.exe_path.to_string_lossy().contains("kernel") {
                kernel_changed += proc_diff.memory_diff;
            } else if proc_diff.new_process.user_id == diff.current_user_id {
                user_changed += proc_diff.memory_diff;
            } else {
                system_changed += proc_diff.memory_diff;
            }
            total_libs += proc_diff.library_changes.iter().map(|l| l.size_diff).sum::<i64>();
            total_exe += proc_diff.exe_size_diff;
            total_files += proc_diff.open_files_diff as i64 * 4096; // 估算每个文件句柄 4KB
            total_shared += proc_diff.shared_memory_diff;
        }

        // 计算各类总变化
        let kernel_total = kernel_new + kernel_removed + kernel_changed;
        let system_total = system_new + system_removed + system_changed;
        let user_total = user_new + user_removed + user_changed;

        // 内存变化总览
        report.push_str("## 内存变化总览\n\n");
        report.push_str("```diff\n");

        // 内核部分
        report.push_str("# 内核内存变化\n");
        if kernel_new > 0 {
            report.push_str(&format!("+ 新增内核进程：{}\n", Analyzer::format_bytes(kernel_new)));
        }
        if kernel_removed < 0 {
            report.push_str(&format!("- 移除内核进程：{}\n", Analyzer::format_bytes(kernel_removed)));
        }
        if kernel_changed != 0 {
            report.push_str(&format!("@ 内核进程变化：{}\n", Analyzer::format_bytes(kernel_changed)));
        }
        report.push_str(&format!("  内核总变化：  {}\n", Analyzer::format_bytes(kernel_total)));

        // 系统部分
        report.push_str("\n# 系统内存变化\n");
        if system_new > 0 {
            report.push_str(&format!("+ 新增系统进程：{}\n", Analyzer::format_bytes(system_new)));
        }
        if system_removed < 0 {
            report.push_str(&format!("- 移除系统进程：{}\n", Analyzer::format_bytes(system_removed)));
        }
        if system_changed != 0 {
            report.push_str(&format!("@ 系统进程变化：{}\n", Analyzer::format_bytes(system_changed)));
        }
        report.push_str(&format!("  系统总变化：  {}\n", Analyzer::format_bytes(system_total)));

        // 用户部分
        report.push_str("\n# 用户内存变化\n");
        if user_new > 0 {
            report.push_str(&format!("+ 新增用户进程：{}\n", Analyzer::format_bytes(user_new)));
        }
        if user_removed < 0 {
            report.push_str(&format!("- 移除用户进程：{}\n", Analyzer::format_bytes(user_removed)));
        }
        if user_changed != 0 {
            report.push_str(&format!("@ 用户进程变化：{}\n", Analyzer::format_bytes(user_changed)));
        }
        report.push_str(&format!("  用户总变化：  {}\n", Analyzer::format_bytes(user_total)));

        // 总计
        report.push_str("\n-------------------\n");
        report.push_str(&format!("  总内存变化：  {}\n", Analyzer::format_bytes(diff.total_diff)));
        report.push_str("```\n\n");
        // 计算总变化
        let total_new = kernel_new + system_new + user_new;
        let total_removed = kernel_removed + system_removed + user_removed;
        let total_changed = kernel_changed + system_changed + user_changed;
        let process_memory_change = total_new + total_removed + total_changed;
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
            .filter(|(_, p)| p.exe_path.to_string_lossy().contains("kernel"))
            .collect();
        let removed_kernel_processes: Vec<_> = diff.removed_processes.iter()
            .filter(|(_, p)| p.exe_path.to_string_lossy().contains("kernel"))
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
            let entry = (name, mem, true, process);
            if process.exe_path.to_string_lossy().contains("kernel") {
                kernel_processes.push(entry);
            } else if process.user_id == diff.current_user_id {
                user_processes.push(entry);
            } else {
                system_processes.push(entry);
            }
        }
        for (name, process) in &diff.removed_processes {
            let mem = if process.pss > 0 { process.pss } else { process.rss } as i64;
            let entry = (name, -mem, false, process);
            if process.exe_path.to_string_lossy().contains("kernel") {
                kernel_processes.push(entry);
            } else if process.user_id == diff.current_user_id {
                user_processes.push(entry);
            } else {
                system_processes.push(entry);
            }
        }
        for (name, proc_diff) in &diff.changed_processes {
            let entry = (name, proc_diff.memory_diff, true, &proc_diff.new_process);
            if proc_diff.new_process.exe_path.to_string_lossy().contains("kernel") {
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
            sorted_processes.push((name, mem, true, process));
        }
        for (name, process) in &diff.removed_processes {
            let mem = if process.pss > 0 { process.pss } else { process.rss } as i64;
            sorted_processes.push((name, -mem, false, process));
        }
        for (name, proc_diff) in &diff.changed_processes {
            sorted_processes.push((name, proc_diff.memory_diff, true, &proc_diff.new_process));
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

    fn write_process_details(report: &mut String, processes: &[(&String, i64, bool, &ProcessInfo)]) {
        // 新增进程
        let new_processes: Vec<_> = processes.iter()
            .filter(|(_, mem, is_current, _)| *is_current && *mem > 0)
            .collect();
        if !new_processes.is_empty() {
            report.push_str("### 新增进程\n\n");
            for (name, mem, _, process) in new_processes {
                report.push_str(&format!("#### {} (🔴 +{})\n", name, Analyzer::format_bytes(*mem)));
                report.push_str(&format!("- 可执行文件路径：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n", process.libraries.len()));
                report.push_str("\n");
            }
        }

        // 移除进程
        let removed_processes: Vec<_> = processes.iter()
            .filter(|(_, _mem, is_current, _)| !*is_current)
            .collect();
        if !removed_processes.is_empty() {
            report.push_str("### 移除进程\n\n");
            for (name, mem, _, process) in removed_processes {
                report.push_str(&format!("#### {} (🟢 {})\n", name, Analyzer::format_bytes(*mem)));
                report.push_str(&format!("- 可执行文件路径：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n", process.libraries.len()));
                report.push_str("\n");
            }
        }

        // 变化进程
        let changed_processes: Vec<_> = processes.iter()
            .filter(|(_, mem, is_current, _)| *is_current && *mem < 0)
            .collect();
        if !changed_processes.is_empty() {
            report.push_str("### 变化进程\n\n");
            for (name, mem, _, process) in changed_processes {
                let change_color = if *mem > 0 { "🔴" } else { "🟢" };
                report.push_str(&format!("#### {} ({} {})\n", name, change_color, Analyzer::format_bytes(*mem)));
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
