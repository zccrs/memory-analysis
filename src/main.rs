mod cli;
mod local;
mod collector;
mod analyzer;
mod reporter;
mod utils;

use anyhow::Result;
use log::{error, info};
use std::path::PathBuf;
use std::process;

use cli::Args;
use collector::Collector;
use analyzer::Analyzer;
use reporter::Reporter;

#[tokio::main]
async fn main() {
    // 解析命令行参数
    let args = Args::parse_args();

    // 初始化日志系统
    env_logger::Builder::new()
        .filter_level(match args.log_level.as_str() {
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        })
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    info!("日志系统初始化完成，级别: {}", args.log_level);

    if let Err(e) = run(args) {
        error!("{:#}", e);
        process::exit(1);
    }
}

fn handle_diff_command(_args: &Args, dirs: Vec<PathBuf>) -> Result<()> {
    info!("开始对比分析...");

    // 从两个目录中查找最新的json文件
    let get_latest_json = |dir: &PathBuf| -> Result<PathBuf> {
        let mut jsons: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |ext| ext == "json") && p.file_name().map_or(false, |name| name.to_str().map_or(false, |s| s.starts_with("test"))))
            .collect();

        jsons.sort_by(|a, b| b.metadata().unwrap().modified().unwrap()
            .cmp(&a.metadata().unwrap().modified().unwrap()));

        jsons.first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("目录 {} 中没有找到采集数据文件", dir.display()))
    };

    let file1 = get_latest_json(&dirs[0])?;
    let file2 = get_latest_json(&dirs[1])?;

    info!("对比数据文件:");
    info!("- {}", file1.display());
    info!("- {}", file2.display());

    // 加载两个数据文件
    let result1: collector::CollectionResult = serde_json::from_reader(std::fs::File::open(file1)?)?;
    let result2: collector::CollectionResult = serde_json::from_reader(std::fs::File::open(file2)?)?;

    // 分析差异
    info!("分析内存差异...");
    let diff = Analyzer::analyze(result1, result2)?;

    // 生成报告并保存到第二个目录
    info!("生成分析报告...");
    let (json_path, md_path, csv_path) = Reporter::generate_report(&diff, &dirs[1], dirs[0].to_str().unwrap_or("旧数据"), dirs[1].to_str().unwrap_or("新数据"))?;

    info!("分析完成！报告已保存到以下位置:");
    info!("- JSON报告: {}", json_path.display());
    info!("- 中文报告: {}", md_path.display());
    info!("- CSV报告: {}", csv_path.display());

    Ok(())
}

fn handle_collect(args: &Args, output_dir: PathBuf) -> Result<()> {
    info!("开始内存采集...");

    // 收集当前系统状态
    let mut collector = Collector::new(args.temp_dir.clone());
    let result = collector.collect()?;

    // 保存采集数据，使用目录名作为文件名
    info!("保存采集数据...");
    let output_name = output_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("默认采集");

    let data_path = output_dir.join(format!("{}.json", output_name));
    serde_json::to_writer_pretty(
        std::fs::File::create(&data_path)?,
        &result
    )?;

    // 修改文件所有者
    utils::fix_file_owner(&data_path)?;

    info!("采集完成！数据已保存到: {}", data_path.display());

    Ok(())
}

fn run(args: Args) -> Result<()> {
    // 验证参数
    args.validate()?;

    // 处理采集或对比命令
    if let Some(ref output_dir) = args.output {
        handle_collect(&args, output_dir.clone())
    } else if let Some(ref dirs) = args.diff_dirs {
        handle_diff_command(&args, dirs.clone())
    } else {
        unreachable!("由参数分组保证了必须有一个模式被选择")
    }
}
