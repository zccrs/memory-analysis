use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "memdiff",
    about = "对比Linux系统内存使用差异",
    version
)]
#[command(group = clap::ArgGroup::new("mode").required(true))]
pub struct Args {
    /// 采集模式：输出目录路径，目录名作为采集说明
    #[arg(group = "mode")]
    pub output: Option<PathBuf>,

    /// 对比模式：指定两个数据目录进行对比
    #[arg(long = "diff", num_args = 2, group = "mode")]
    pub diff_dirs: Option<Vec<PathBuf>>,

    /// 日志级别 (debug, info, warn, error)
    #[arg(long = "log-level", default_value = "info")]
    pub log_level: String,

    /// 临时目录
    #[arg(long = "temp-dir", default_value = "/tmp/memdiff")]
    pub temp_dir: PathBuf,
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

        // 验证对比目录
        if let Some(ref dirs) = self.diff_dirs {
            for dir in dirs {
                if !dir.exists() || !dir.is_dir() {
                    anyhow::bail!("指定的目录不存在: {}", dir.display());
                }
                // 检查目录下是否有采集数据文件
                let has_json = std::fs::read_dir(dir)?
                    .filter_map(|e| e.ok())
                    .any(|e| e.path().extension().map_or(false, |ext| ext == "json"));
                if !has_json {
                    anyhow::bail!("目录 {} 下没有找到采集数据文件", dir.display());
                }
            }
        }
        Ok(())
    }
}
