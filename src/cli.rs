use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "memdiff", about = "对比Linux系统内存使用差异", version = "0.1.0")]
#[command(group = clap::ArgGroup::new("mode").required(true))]
pub struct Args {
    /// 采集模式：输出目录路径，目录名作为采集说明
    #[arg(group = "mode")]
    pub output: Option<PathBuf>,

    /// 对比模式：指定两个数据目录或json文件进行对比
    #[arg(long = "diff", num_args = 2, group = "mode")]
    pub diff_targets: Option<Vec<PathBuf>>,

    /// 日志级别 (debug, info, warn, error)
    #[arg(long = "log-level", default_value = "info")]
    pub log_level: String,

    /// 临时目录
    #[arg(long = "temp-dir", default_value = "/tmp/memdiff")]
    pub temp_dir: PathBuf,

    /// 单进程模式：指定进程ID，直接在终端输出采集结果
    #[arg(long = "pid", group = "mode")]
    pub pid: Option<i32>,

    /// 最大采集进程数量，达到后终止采集
    #[arg(long = "max-processes")]
    pub max_processes: Option<usize>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        // 验证日志级别
        match self.log_level.to_lowercase().as_str() {
            "debug" | "info" | "warn" | "error" => Ok::<(), anyhow::Error>(()),
            _ => anyhow::bail!("无效的日志级别，可选值: debug, info, warn, error"),
        }?;

        // 检查输出目录
        if let Some(ref path) = self.output {
            // 创建输出目录（如果不存在）
            if !path.exists() {
                std::fs::create_dir_all(path)?;
            }
        }

        // 验证对比目标（目录或json文件）
        if let Some(ref targets) = self.diff_targets {
            for target in targets {
                if !target.exists() {
                    anyhow::bail!("指定的路径不存在: {}", target.display());
                }

                if target.is_dir() {
                    // 如果是目录，检查是否包含json文件
                    let has_json = std::fs::read_dir(target)?
                        .filter_map(|e| e.ok())
                        .any(|e| e.path().extension().map_or(false, |ext| ext == "json"));
                    if !has_json {
                        anyhow::bail!("目录 {} 下没有找到采集数据文件", target.display());
                    }
                } else {
                    // 如果是文件，检查是否是json文件
                    if target.extension().map_or(false, |ext| ext != "json") {
                        anyhow::bail!("文件 {} 不是json格式", target.display());
                    }
                }
            }
        }
        Ok(())
    }
}
