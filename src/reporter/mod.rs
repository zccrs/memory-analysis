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
        report.push_str(&format!("本报告是 {} 相对于 {} 的内存使用情况。\n\n", new_identifier, old_identifier));
        report.push_str(&format!("总进程数量（{}）：{}\n", new_identifier, diff.new_processes.len() + diff.changed_processes.len()));
        report.push_str(&format!("总进程数量（{}）：{}\n", old_identifier, diff.removed_processes.len() + diff.changed_processes.len()));
        report.push_str(&format!("新增进程数量：{}\n\n", diff.new_processes.len()));

        report.push_str(&format!("本报告对比了 {} 和 {} 的内存使用情况。\n\n", old_identifier, new_identifier));

        // 总体内存差异
        report.push_str("## 总体内存差异\n\n");
        report.push_str(&format!("内存变化：{}\n\n",
            Analyzer::format_bytes(diff.total_diff)));

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

        // 新增进程详情
        if !diff.new_processes.is_empty() {
            report.push_str("### 新增进程详情\n\n");
            for (name, process) in &diff.new_processes {
                let mem = if process.pss > 0 {
                    process.pss
                } else {
                    process.rss
                };
                report.push_str(&format!("#### {}\n", name));
                report.push_str(&format!("- 内存使用：{}\n", Analyzer::format_bytes(mem as i64)));
                report.push_str(&format!("- 可执行文件：{}\n", process.exe_path.display()));
                report.push_str(&format!("- 打开文件数：{}\n", process.open_files_count));
                report.push_str(&format!("- 加载动态库：{} 个\n\n", process.libraries.len()));
            }
        }

        // 已删除进程详情
        if !diff.removed_processes.is_empty() {
            report.push_str("### 已删除进程详情\n\n");
            for (name, process) in &diff.removed_processes {
                let mem = if process.pss > 0 {
                    process.pss
                } else {
                    process.rss
                };
                report.push_str(&format!("#### {}\n", name));
                report.push_str(&format!("- 释放内存：{}\n", Analyzer::format_bytes(mem as i64)));
                report.push_str(&format!("- 原可执行文件：{}\n\n", process.exe_path.display()));
            }
        }

        // 所有进程详情（包括无变化的进程）
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

    fn generate_csv_report(diff: &MemoryDiff, csv_path: &Path) -> Result<()> {
        let mut wtr = Writer::from_path(csv_path)?;

        // 写入表头
        wtr.write_record(&["进程名", "旧内存占用", "新内存占用", "内存变化"])?;

        // 新增进程
        for (name, process) in &diff.new_processes {
            let mem = if process.pss > 0 {
                process.pss
            } else {
                process.rss
            };
            wtr.write_record(&[name, "0", &mem.to_string(), &mem.to_string()])?;
        }

        // 已删除进程
        for (name, process) in &diff.removed_processes {
            let mem = if process.pss > 0 {
                process.pss
            } else {
                process.rss
            };
            wtr.write_record(&[name, &mem.to_string(), "0", &(-(mem as i64)).to_string()])?;
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
            wtr.write_record(&[name, &old_mem.to_string(), &new_mem.to_string(), &mem_change.to_string()])?;
        }

        wtr.flush()?;
        Ok(())
    }
}
