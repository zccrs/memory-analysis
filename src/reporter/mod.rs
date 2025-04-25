use crate::analyzer::{Analyzer, MemoryDiff};
use crate::collector::ProcessInfo;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use serde_json;
use log::info;
use csv::Writer;

#[derive(Debug, Copy, Clone)]
pub enum ProcessChangeType {
    New,
    Removed,
    Changed,
}

pub struct Reporter;

impl Reporter {
    pub fn is_kernel_process(process: &ProcessInfo) -> bool {
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

        report.push_str("# 内存使用差异分析报告\n\n");

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

        // 初始化总变化量
        let mut total_new = 0i64;
        let mut total_removed = 0i64;
        let mut total_changed = 0i64;

        // 计算新增进程的总内存
        for process in diff.new_processes.values() {
            let mem = if process.mem_info.pss > 0 { process.mem_info.pss } else { process.mem_info.rss } as i64;
            total_new += mem;
        }

        // 计算移除进程的总内存
        for process in diff.removed_processes.values() {
            let mem = if process.mem_info.pss > 0 { process.mem_info.pss } else { process.mem_info.rss } as i64;
            total_removed += mem;
        }

        // 计算变化进程的总内存差异
        for proc_diff in diff.changed_processes.values() {
            total_changed += proc_diff.memory_diff;
        }

        report.push_str("## 系统概述\n\n");
        report.push_str(&format!("本报告对比了 {} 和 {} 之间的内存使用变化。\n\n", old_name, new_name));
        report.push_str(&format!("### 进程变化统计\n\n"));
        report.push_str(&format!("- 新增进程数量：{}\n", diff.new_processes.len()));
        report.push_str(&format!("- 移除进程数量：{}\n", diff.removed_processes.len()));
        report.push_str(&format!("- 变化进程数量：{}\n\n", diff.changed_processes.len()));

        report.push_str("### 内存变化概要\n\n");
        report.push_str(&format!("- 新增进程内存：{}\n", Analyzer::format_bytes(total_new)));
        report.push_str(&format!("- 移除进程内存：{}\n", Analyzer::format_bytes(-total_removed)));
        report.push_str(&format!("- 变化进程内存：{}\n", Analyzer::format_bytes(total_changed)));
        report.push_str(&format!("- 总内存变化：{}\n\n", Analyzer::format_bytes(diff.total_diff)));

        // 对进程按类别分类
        let mut kernel_processes = Vec::new();
        let mut system_processes = Vec::new();
        let mut user_processes = Vec::new();

        // 收集所有进程信息并分类
        for (name, process) in &diff.new_processes {
            let mem = if process.mem_info.pss > 0 { process.mem_info.pss } else { process.mem_info.rss } as i64;
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
            let mem = if process.mem_info.pss > 0 { process.mem_info.pss } else { process.mem_info.rss } as i64;
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

        // 按内存变化大小排序
        kernel_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());
        system_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());
        user_processes.sort_by_key(|(_, mem, _, _)| -mem.abs());

        // 输出不同类型的进程详情
        if !kernel_processes.is_empty() {
            report.push_str("## 内核进程变化\n\n");
            Self::write_process_details(&mut report, &kernel_processes);
        }

        if !system_processes.is_empty() {
            report.push_str("## 系统进程变化\n\n");
            Self::write_process_details(&mut report, &system_processes);
        }

        if !user_processes.is_empty() {
            report.push_str("## 用户进程变化\n\n");
            Self::write_process_details(&mut report, &user_processes);
        }

        // 变化进程详情
        if !diff.changed_processes.is_empty() {
            report.push_str("## 进程详细变化\n\n");
            for (name, proc_diff) in &diff.changed_processes {
                let change_symbol = if proc_diff.memory_diff > 0 { "🔴" } else { "🟢" };
                report.push_str(&format!("### {} ({} {})\n", name, change_symbol,
                    Analyzer::format_bytes(proc_diff.memory_diff)));
                report.push_str(&format!("- 内存使用变化：{} -> {} ({})\n",
                    Analyzer::format_bytes(if proc_diff.old_process.mem_info.pss > 0 { proc_diff.old_process.mem_info.pss as i64 } else { proc_diff.old_process.mem_info.rss as i64 }),
                    Analyzer::format_bytes(if proc_diff.new_process.mem_info.pss > 0 { proc_diff.new_process.mem_info.pss as i64 } else { proc_diff.new_process.mem_info.rss as i64 }),
                    if proc_diff.memory_diff > 0 { format!("增加 {}", Analyzer::format_bytes(proc_diff.memory_diff)) }
                    else { format!("减少 {}", Analyzer::format_bytes(-proc_diff.memory_diff)) }
                ));

                if proc_diff.exe_size_diff != 0 {
                    report.push_str(&format!("- 可执行文件大小变化：{}\n",
                        Analyzer::format_bytes(proc_diff.exe_size_diff)));
                }

                if proc_diff.open_files_diff != 0 {
                    report.push_str(&format!("- 打开文件数变化：{:+}\n", proc_diff.open_files_diff));
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
                            (Some(old), Some(new)) => {
                                report.push_str(&format!("  - 库路径变更：{} -> {}\n",
                                    old, new));
                            }
                            (Some(old), None) => {
                                report.push_str(&format!("  - 🟢移除库 {}\n", old));
                            }
                            (None, Some(new)) => {
                                report.push_str(&format!("  - 🔴新增库 {}\n", new));
                            }
                            _ => {}
                        }
                    }
                }
                report.push_str("\n");
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
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files.len()));
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
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files.len()));
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
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files.len()));
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
            let mem = if process.mem_info.pss > 0 {
                process.mem_info.pss
            } else {
                process.mem_info.rss
            };
            wtr.write_record(&[&name, "0", &Self::bytes_to_mb(mem as i64), &Self::bytes_to_mb(mem as i64)])?;
        }

        // 已删除进程
        for (name, process) in &diff.removed_processes {
            let mem = if process.mem_info.pss > 0 {
                process.mem_info.pss
            } else {
                process.mem_info.rss
            };
            wtr.write_record(&[name, &Self::bytes_to_mb(mem as i64), "0", &Self::bytes_to_mb(-(mem as i64))])?;
        }

        // 变化的进程
        for (name, proc_diff) in &diff.changed_processes {
            let old_mem = if proc_diff.old_process.mem_info.pss > 0 {
                proc_diff.old_process.mem_info.pss
            } else {
                proc_diff.old_process.mem_info.rss
            };
            let new_mem = if proc_diff.new_process.mem_info.pss > 0 {
                proc_diff.new_process.mem_info.pss
            } else {
                proc_diff.new_process.mem_info.rss
            };
            let mem_change = (new_mem as i64) - (old_mem as i64);
            wtr.write_record(&[&name, &Self::bytes_to_mb(old_mem as i64), &Self::bytes_to_mb(new_mem as i64), &Self::bytes_to_mb(mem_change)])?;
        }

        wtr.flush()?;
        Ok(())
    }
}
