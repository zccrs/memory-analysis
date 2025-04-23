# Memory Analysis Tool (内存分析工具)

这是一个用于分析 Linux 系统进程物理内存占用差异的命令行工具。它可以帮助您分析系统升级前后的内存使用变化，找出内存增长的原因。

## 特性

- ✨ 支持两种数据采集模式：本地实时采集和历史数据对比
- 🔐 自动处理 sudo 权限
- 📊 生成详细的中文分析报告
- 🔍 分析多个内存变化因素
- 📝 支持 JSON 和 Markdown 格式输出
- 💾 自动保存每次采集的数据供后续对比

## 安装

1. 确保系统已安装 Rust 工具链：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 克隆并编译项目：

```bash
git clone https://github.com/yourusername/memory-analysis.git
cd memory-analysis
cargo build --release
```

编译后的二进制文件将位于 `target/release/memory-analysis`。

## 使用方法

memory-analysis提供两种工作模式：采集模式和对比模式。目录的名称将作为采集说明，例如"升级前"、"升级后"等。

### 远程主机对比分析

如果需要对比两台不同主机的内存状态，可以使用`memory-analysis.sh`脚本：

```bash
./memory-analysis.sh <第一台主机(user@host)> <第二台主机(user@host)> <本地结果目录>
```

例如：
```bash
./memory-analysis.sh zccrs@127.0.0.1 zccrs@10.20.7.185 /tmp/test
```

此脚本会：
1. 自动编译最新版本的memory-analysis程序（静态链接）
2. 将程序复制到两台远程主机
3. 在两台主机上执行数据采集
4. 将采集结果复制回本地
5. 进行差异分析并生成报告

使用此脚本前请确保：
- 两台主机都能通过SSH访问
- 本地已安装Rust工具链
- 远程主机具有sudo权限

分析报告将保存在指定的本地结果目录中。

### 基本用法

#### 1. 采集模式
采集当前系统状态，将数据保存到指定目录：

```bash
memory-analysis <输出目录>
```

采集模式会在指定目录中生成一个同名的json文件，例如：
```
memory-analysis ./升级前    # 生成 ./升级前/升级前.json
```

#### 2. 对比模式
对比两次采集的数据。支持两种方式：

1. 使用目录进行对比：
```bash
memory-analysis --diff <第一次采集目录> <第二次采集目录>
```
此方式会自动在两个目录中查找采集数据文件进行对比，并在第二个目录中生成分析报告。

2. 直接使用JSON文件对比：
```bash
memory-analysis --diff <第一个JSON文件> <第二个JSON文件>
```
此方式直接使用指定的JSON文件进行对比。如果第二个参数是JSON文件，则会在当前目录下生成分析报告。

### 使用场景示例

#### 1. 系统升级前后的内存变化分析

```bash
# 升级前采集数据
memory-analysis ./升级前

# 升级后采集数据
memory-analysis ./升级后

# 分析内存变化
memory-analysis --diff ./升级前 ./升级后
```

#### 2. 应用启动前后的内存变化分析

```bash
# 启动前采集数据
memory-analysis ./启动前

# 启动后采集数据
memory-analysis ./启动后

# 分析内存变化
memory-analysis --diff ./启动前 ./启动后
```

#### 3. 长期运行的内存泄漏分析

```bash
# 记录初始状态
memory-analysis ./开始运行

# 运行24小时后采集数据
memory-analysis ./运行24h

# 分析内存变化
memory-analysis --diff ./开始运行 ./运行24h
```

### 输出文件说明

#### 采集模式输出
- `<目录名>/<目录名>.json`: 系统状态采集数据

#### 对比模式输出（保存在第二个目录）
- `diff_report.json`: 差异分析的 JSON 格式报告
- `diff_report_中文.md`: 中文 Markdown 格式的详细分析报告

### 完整参数说明

```bash
memory-analysis [输出目录|选项]

参数:
    <输出目录>              采集模式：指定数据保存目录，目录名作为采集说明

选项:
    --diff <文件1> <文件2>    对比模式：比较两个采集数据目录或JSON文件
    --pid <PID>            单进程模式：指定进程ID，直接输出该进程的内存使用情况
    --log-level <LEVEL>     日志级别 (debug, info, warn, error) [默认: info]
    --temp-dir <DIR>        临时目录 [默认: /tmp/memdiff]
    --max-processes <NUM>    限制采集进程的最大数量
```

### 单进程分析示例

如果只想查看单个进程的内存使用情况，可以使用`--pid`参数：

```bash
# 分析指定进程的内存使用情况
memory-analysis --pid 12345

# 输出结果示例：
进程信息 (PID: 12345)
名称: example
可执行文件: "/usr/bin/example"
可执行文件大小: 2.5 MB
PSS: 15.7 MB
RSS: 25.3 MB
共享内存: 128.0 KB
打开文件数: 12

动态库信息:
- "/usr/lib/libc.so.6" (2.1 MB)
- "/usr/lib/libstdc++.so.6" (1.8 MB)
...
```

### 权限要求

- sudo 权限（用于读取 /proc 下的信息）

## 输出文件

工具会在指定的输出目录生成以下文件：

- `<DESC>.json`: 本次采集的原始数据
- `diff_report.json`: 差异分析的 JSON 格式报告
- `diff_report_中文.md`: 中文 Markdown 格式的详细分析报告

## 分析报告内容

### 总体分析
- 总体内存使用差异
- 系统配置变化（内核版本、页大小等）
- 共享内存变化

### 详细分析
- 新增进程列表及其内存占用
- 已删除进程列表及释放的内存
- 发生变化的进程详情：
  - 内存使用变化
  - 可执行文件大小变化
  - 动态库变化
  - 打开文件数量变化
  - 共享内存使用变化

## 注意事项

1. 确保两台主机都能通过 SSH 访问
2. 需要 sudo 权限来读取某些系统信息
3. 远程主机无需预装任何特殊软件
4. 建议使用最新版本的 Rust 工具链编译

## 常见问题

1. 权限不足
   - 确保用户有 sudo 权限
   - 检查 /proc 文件系统的访问权限

2. JSON文件格式错误
   - 确保对比数据文件未被修改
   - 使用最新版本的工具重新采集数据

3. 使用memory-analysis.sh脚本时的Ctrl+C问题
   - 在SSH连接建立阶段，按Ctrl+C可能会导致脚本无法正常退出
   - 建议等待SSH连接完全建立后再使用Ctrl+C中断

## 贡献指南

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
