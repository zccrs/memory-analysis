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

fn handle_diff_command(_args: &Args, targets: Vec<PathBuf>) -> Result<()> {
    info!("开始对比分析...");

    // 获取json文件路径
    let get_json_path = |target: &PathBuf| -> Result<PathBuf> {
        if target.is_dir() {
            // 从目录中查找最新的json文件
            let mut jsons: Vec<_> = std::fs::read_dir(target)?
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map_or(false, |ext| ext == "json"))
                .collect();

            jsons.sort_by(|a, b| b.metadata().unwrap().modified().unwrap()
                .cmp(&a.metadata().unwrap().modified().unwrap()));

            jsons.first()
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("目录 {} 中没有找到采集数据文件", target.display()))
        } else {
            // 直接返回json文件路径
            Ok(target.clone())
        }
    };

    let file1 = get_json_path(&targets[0])?;
    let file2 = get_json_path(&targets[1])?;

    info!("对比数据文件:");
    info!("- {}", file1.display());
    info!("- {}", file2.display());

    // 加载两个数据文件
    let result1: collector::CollectionResult = serde_json::from_reader(std::fs::File::open(file1)?)?;
    let result2: collector::CollectionResult = serde_json::from_reader(std::fs::File::open(file2)?)?;

    // 分析差异
    info!("分析内存差异...");
    let diff = Analyzer::analyze(result1, result2)?;

    // 确定报告保存位置
    let report_dir = if targets[1].is_dir() {
        targets[1].clone()
    } else {
        // 如果target2是文件，则使用当前目录
        std::env::current_dir()?
    };

    // 生成报告
    info!("生成分析报告...");
    let desc1 = targets[0].file_stem().and_then(|s| s.to_str()).unwrap_or("旧数据");
    let desc2 = targets[1].file_stem().and_then(|s| s.to_str()).unwrap_or("新数据");
    let (json_path, md_path, csv_path) = Reporter::generate_report(&diff, &report_dir, desc1, desc2)?;

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

fn handle_single_process(_args: &Args, pid: i32) -> Result<()> {
    info!("开始采集单个进程信息...");

    let collector = Collector::new(PathBuf::from("/tmp"));
    let result = collector.collect_single_process(pid)?;
    println!("\n{}", result);

    Ok(())
}

fn run(args: Args) -> Result<()> {
    // 验证参数
    args.validate()?;

    // 处理采集或对比命令
    if let Some(ref output_dir) = args.output {
        handle_collect(&args, output_dir.clone())
    } else if let Some(ref targets) = args.diff_targets {
        handle_diff_command(&args, targets.clone())
    } else if let Some(pid) = args.pid {
        handle_single_process(&args, pid)
    } else {
        unreachable!("由参数分组保证了必须有一个模式被选择")
    }
}
