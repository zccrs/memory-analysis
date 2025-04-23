#!/bin/bash

# 显示用法信息
usage() {
    echo "用法: $0 <远程主机(user@host)> <本地结果目录>"
    echo "示例: $0 zccrs@10.20.7.185 /tmp/test"
    echo ""
    echo "说明:"
    echo "  此脚本会在远程主机上执行memory-analysis程序，"
    echo "  并将生成的结果文件复制到指定的本地目录。"
    exit 1
}

# 检查参数
if [ $# -ne 2 ]; then
    usage
fi

REMOTE_HOST="$1"
LOCAL_RESULT_DIR="$2"
REMOTE_TEMP_DIR="/tmp/memory-analysis-$(date +%s)"
BINARY_NAME="memory-analysis"
SSH_CONTROL_PATH="/tmp/ssh-memory-analysis-$$.socket"

# 清理函数
cleanup() {
    # 终止远程进程
    if [ -n "$REMOTE_PID" ]; then
        ssh -o "ControlPath=$SSH_CONTROL_PATH" "$REMOTE_HOST" "sudo kill -TERM $REMOTE_PID" 2>/dev/null
    fi
    # 清理远程临时目录
    ssh -o "ControlPath=$SSH_CONTROL_PATH" "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR" 2>/dev/null
    # 关闭SSH主连接
    ssh -O exit -o "ControlPath=$SSH_CONTROL_PATH" "$REMOTE_HOST" 2>/dev/null
    # 删除控制socket
    rm -f "$SSH_CONTROL_PATH"
    # 恢复终端状态
    stty sane 2>/dev/null || true
}

# 信号处理函数
handle_interrupt() {
    echo -e "\n收到中断信号，正在清理..."
    cleanup
    exit 1
}

# 设置清理钩子
trap cleanup EXIT
trap handle_interrupt INT TERM

# 确保本地结果目录存在
mkdir -p "$LOCAL_RESULT_DIR"

echo "Step 1: 建立SSH连接..."
# 创建主SSH连接
ssh -nNf -o "ControlMaster=yes" -o "ControlPath=$SSH_CONTROL_PATH" "$REMOTE_HOST"

echo "Step 1: 编译最新版本(静态链接)..."
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu || {
    echo "错误: 编译失败"
    exit 1
}

echo "Step 2: 创建远程临时目录..."
ssh -o "ControlPath=$SSH_CONTROL_PATH" "$REMOTE_HOST" "mkdir -p $REMOTE_TEMP_DIR" || {
    echo "错误: 无法在远程主机上创建目录"
    exit 1
}

echo "Step 3: 复制程序到远程主机..."
scp -o "ControlPath=$SSH_CONTROL_PATH" "./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME" "$REMOTE_HOST:$REMOTE_TEMP_DIR/" || {
    echo "错误: 无法复制程序到远程主机"
    ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"
    exit 1
}

echo "Step 4: 在远程主机上执行程序..."
ssh -tt -o "ControlPath=$SSH_CONTROL_PATH" "$REMOTE_HOST" "cd $REMOTE_TEMP_DIR && chmod +x $BINARY_NAME && sudo ./$BINARY_NAME . & REMOTE_PID=\$! && wait \$REMOTE_PID" || {
    echo "错误: 远程执行失败"
    ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"
    exit 1
}

echo "Step 5: 复制结果文件到本地..."
scp -o "ControlPath=$SSH_CONTROL_PATH" -r "$REMOTE_HOST:$REMOTE_TEMP_DIR/*" "$LOCAL_RESULT_DIR/" || {
    echo "错误: 无法复制结果文件"
    ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"
    exit 1
}

echo "完成！结果文件已保存到: $LOCAL_RESULT_DIR"
# cleanup会在脚本退出时自动执行
