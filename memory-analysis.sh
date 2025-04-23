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

# 确保本地结果目录存在
mkdir -p "$LOCAL_RESULT_DIR"

echo "Step 1: 编译最新版本(静态链接)..."
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu || {
    echo "错误: 编译失败"
    exit 1
}

echo "Step 2: 创建远程临时目录..."
ssh "$REMOTE_HOST" "mkdir -p $REMOTE_TEMP_DIR" || {
    echo "错误: 无法在远程主机上创建目录"
    exit 1
}

echo "Step 3: 复制程序到远程主机..."
scp "./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME" "$REMOTE_HOST:$REMOTE_TEMP_DIR/" || {
    echo "错误: 无法复制程序到远程主机"
    ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"
    exit 1
}

echo "Step 4: 在远程主机上执行程序..."
ssh "$REMOTE_HOST" "cd $REMOTE_TEMP_DIR && chmod +x $BINARY_NAME && sudo ./$BINARY_NAME ." || {
    echo "错误: 远程执行失败"
    ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"
    exit 1
}

echo "Step 5: 复制结果文件到本地..."
scp -r "$REMOTE_HOST:$REMOTE_TEMP_DIR/*" "$LOCAL_RESULT_DIR/" || {
    echo "错误: 无法复制结果文件"
    ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"
    exit 1
}

echo "Step 6: 清理远程临时文件..."
ssh "$REMOTE_HOST" "rm -rf $REMOTE_TEMP_DIR"

echo "完成！结果文件已保存到: $LOCAL_RESULT_DIR"
