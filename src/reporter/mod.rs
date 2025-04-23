use crate::analyzer::{Analyzer, MemoryDiff};
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

        // 添加内存统计方式说明
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
        report.push_str("   - 跳过 kworker 等内核工作线程\n");
        report.push_str("   - 可通过 --max-processes 参数限制采集进程数量\n");
        report.push_str("   - 被跳过的进程数量会记录在统计信息中\n\n");

        report.push_str("4. **进程状态说明**\n");
        report.push_str("   - "进程已终止"：表示该进程在第一次采集时存在，但在第二次采集时已不存在\n");
        report.push_str("   - 这类进程会显示释放的内存大小和原可执行文件路径\n");
        report.push_str("   - 进程终止通常意味着其占用的内存已被释放\n\n");

        report.push_str(&format!("本报告是 {} 相对于 {} 的内存使用情况。\n\n", new_identifier, old_identifier));
        report.push_str(&format!("总进程数量（{}）：{}\n", new_identifier, diff.new_processes.len() + diff.changed_processes.len()));
        report.push_str(&format!("总进程数量（{}）：{}\n", old_identifier, diff.removed_processes.len() + diff.changed_processes.len()));
        report.push_str(&format!("新增进程数量：{}\n", diff.new_processes.len()));
        report.push_str(&format!("移除进程数量：{}\n\n", diff.removed_processes.len()));

        report.push_str(&format!("本报告对比了 {} 和 {} 的内存使用情况。\n\n", old_identifier, new_identifier));

        // 计算各种变化的内存占比
        let mut total_new = 0i64;
        let mut total_removed = 0i64;
        let mut total_changed = 0i64;
        let mut total_libs = 0i64;
        let mut total_exe = 0i64;
        let mut total_files = 0i64;
        let mut total_shared = 0i64;

        // 统计新增进程
        for process in diff.new_processes.values() {
            total_new += if process.pss > 0 {
                process.pss as i64
            } else {
                process.rss as i64
            };
        }

        // 统计已删除进程
        for process in diff.removed_processes.values() {
            total_removed -= if process.pss > 0 {
                process.pss as i64
            } else {
                process.rss as i64
            };
        }

        // 统计变化的进程
        for proc_diff in diff.changed_processes.values() {
            total_changed += proc_diff.memory_diff;
            total_libs += proc_diff.library_changes.iter()
                .map(|l| l.size_diff)
                .sum::<i64>();
            total_exe += proc_diff.exe_size_diff;
            total_files += proc_diff.open_files_diff as i64 * 4096; // 估算每个文件句柄 4KB
            total_shared += proc_diff.shared_memory_diff;
        }

        // 总体内存差异
        report.push_str("## 总体内存差异\n\n");

        // 计算实际内存变化的组成
        let process_memory_change = total_new + total_removed + total_changed;
        let libs_and_exe_change = total_libs + total_exe;
        let other_change = total_files + total_shared;
        let kernel_change = diff.total_diff - process_memory_change - libs_and_exe_change - other_change;

        report.push_str("```diff\n");
        report.push_str(&format!("+ 新增进程内存：{}\n", Analyzer::format_bytes(total_new)));
        report.push_str(&format!("- 移除进程内存：{}\n", Analyzer::format_bytes(total_removed)));
        report.push_str(&format!("@ 现有进程变化：{}\n", Analyzer::format_bytes(total_changed)));
        report.push_str(&format!("  动态库变化：   {}\n", Analyzer::format_bytes(total_libs)));
        report.push_str(&format!("  其他资源变化： {}\n", Analyzer::format_bytes(other_change)));
        report.push_str(&format!("  内核空间变化： {}\n", Analyzer::format_bytes(kernel_change)));
        report.push_str("-------------------\n");
        report.push_str(&format!("  总内存变化：   {}\n", Analyzer::format_bytes(diff.total_diff)));
        report.push_str("```\n\n");
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

        // 按内存变化大小排序的进程详情
        let mut sorted_processes = Vec::new();

        // 收集所有进程信息并计算内存变化
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

        // 打印详细信息
        report.push_str("### 进程内存变化详情（按变化量排序）\n\n");
        for (name, mem_change, is_current, process) in sorted_processes {
            report.push_str(&format!("#### {} ({:+})\n", name, Analyzer::format_bytes(mem_change)));
            if is_current {
                report.push_str(&format!("- 可执行文件：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n", process.libraries.len()));
            } else {
                report.push_str("- 进程已终止\n");
                report.push_str(&format!("- 原可执行文件：{}\n", process.exe_path.display()));
            }
            report.push_str("\n");
        }

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
                                report.push_str(&format!("  - 库大小变化 {}：{}\n",
                                    old,
                                    Analyzer::format_bytes(lib.size_diff)));
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
