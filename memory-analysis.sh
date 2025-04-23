#!/bin/bash

# 显示用法信息
usage() {
    echo "用法: $0 <第一台主机(user@host)> <第二台主机(user@host)> <本地结果目录> [进程数量限制]"
    echo "示例:"
    echo "  $0 zccrs@127.0.0.1 zccrs@10.20.7.185 /tmp/test"
    echo "  $0 zccrs@127.0.0.1 zccrs@10.20.7.185 /tmp/test 5"
    echo ""
    echo "说明:"
    echo "  此脚本会分别在两台主机上执行memory-analysis程序，"
    echo "  并将结果保存到指定目录后进行对比分析。"
    echo "  可选参数进程数量限制用于控制每台主机上采集的最大进程数。"
    exit 1
}

# 检查参数
if [ $# -lt 3 ] || [ $# -gt 4 ]; then
    usage
fi

# 设置进程数量限制参数
MAX_PROCESSES=""
if [ $# -eq 4 ]; then
    MAX_PROCESSES="--max-processes $4"
fi

REMOTE_HOST1="$1"
REMOTE_HOST2="$2"
LOCAL_RESULT_DIR="$3"
TIMESTAMP=$(date +%s)
REMOTE_TEMP_DIR1="/tmp/memory-analysis-1-$TIMESTAMP"
REMOTE_TEMP_DIR2="/tmp/memory-analysis-2-$TIMESTAMP"
BINARY_NAME="memory-analysis"
SSH_CONTROL_PATH1="/tmp/ssh-memory-analysis-1-$$.socket"
SSH_CONTROL_PATH2="/tmp/ssh-memory-analysis-2-$$.socket"

# 清理函数
cleanup() {
    # 终止远程进程
    if [ -n "$REMOTE_PID1" ]; then
        ssh $SSH_OPTS1 "$REMOTE_HOST1" "sudo kill -TERM $REMOTE_PID1" 2>/dev/null
    fi
    if [ -n "$REMOTE_PID2" ]; then
        ssh $SSH_OPTS2 "$REMOTE_HOST2" "sudo kill -TERM $REMOTE_PID2" 2>/dev/null
    fi
    # 清理远程临时目录
    ssh $SSH_OPTS1 "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1" 2>/dev/null
    ssh $SSH_OPTS2 "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2" 2>/dev/null
    # 关闭SSH主连接
    ssh -O exit $SSH_OPTS1 "$REMOTE_HOST1" 2>/dev/null
    ssh -O exit $SSH_OPTS2 "$REMOTE_HOST2" 2>/dev/null
    # 删除控制socket
    rm -f "$SSH_CONTROL_PATH1" "$SSH_CONTROL_PATH2"
    # 恢复终端状态
    stty sane 2>/dev/null || true
}

# 保存SSH进程ID的变量
SSH_PID1=""
SSH_PID2=""

# 信号处理函数
handle_interrupt() {
    echo -e "\n收到中断信号，正在清理..."
    # 终止正在运行的SSH进程
    if [ -n "$SSH_PID1" ]; then
        kill -TERM $SSH_PID1 2>/dev/null
    fi
    if [ -n "$SSH_PID2" ]; then
        kill -TERM $SSH_PID2 2>/dev/null
    fi
    cleanup
    exit 1
}

# 设置清理钩子
trap cleanup EXIT
trap handle_interrupt INT TERM QUIT

# 确保本地结果目录及子目录存在
mkdir -p "$LOCAL_RESULT_DIR/host1"
mkdir -p "$LOCAL_RESULT_DIR/host2"

# SSH复用连接选项
SSH_OPTS1="-o ControlMaster=auto -o ControlPath=$SSH_CONTROL_PATH1 -o ControlPersist=yes"
SSH_OPTS2="-o ControlMaster=auto -o ControlPath=$SSH_CONTROL_PATH2 -o ControlPersist=yes"

echo "Step 1: 测试SSH连接..."
# 测试SSH连接并建立复用
# 测试SSH连接并保存进程ID
ssh $SSH_OPTS1 "$REMOTE_HOST1" "echo '主机1连接成功'" & SSH_PID1=$!
wait $SSH_PID1 || {
    echo "错误: 无法连接到主机1"
    exit 1
}

ssh $SSH_OPTS2 "$REMOTE_HOST2" "echo '主机2连接成功'" & SSH_PID2=$!
wait $SSH_PID2 || {
    echo "错误: 无法连接到主机2"
    exit 1
}

# 连接成功后清除进程ID
SSH_PID1=""
SSH_PID2=""

echo "Step 1: 编译最新版本(静态链接)..."
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu || {
    echo "错误: 编译失败"
    exit 1
}

echo "Step 2: 创建远程临时目录..."
ssh $SSH_OPTS1 "$REMOTE_HOST1" "mkdir -p $REMOTE_TEMP_DIR1" || {
    echo "错误: 无法在主机1上创建目录"
    exit 1
}

ssh $SSH_OPTS2 "$REMOTE_HOST2" "mkdir -p $REMOTE_TEMP_DIR2" || {
    echo "错误: 无法在主机2上创建目录"
    exit 1
}

echo "Step 3: 复制程序到远程主机..."
scp $SSH_OPTS1 "./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME" "$REMOTE_HOST1:$REMOTE_TEMP_DIR1/" || {
    echo "错误: 无法复制程序到主机1"
    ssh "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1"
    exit 1
}

scp $SSH_OPTS2 "./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME" "$REMOTE_HOST2:$REMOTE_TEMP_DIR2/" || {
    echo "错误: 无法复制程序到主机2"
    ssh "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2"
    exit 1
}

echo "Step 4: 在远程主机上执行程序..."

# 获取sudo密码（将在两台主机上使用）
read -s -p "请输入sudo密码: " SUDO_PASSWORD
echo

echo "在主机1上采集数据..."
ssh -tt $SSH_OPTS1 "$REMOTE_HOST1" "cd $REMOTE_TEMP_DIR1 && chmod +x $BINARY_NAME && echo '$SUDO_PASSWORD' | sudo -S ./$BINARY_NAME $MAX_PROCESSES ." & SSH_PID1=$!
wait $SSH_PID1 || {
    echo "错误: 主机1执行失败"
    ssh "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1"
    exit 1
}
SSH_PID1=""

echo "在主机2上采集数据..."
ssh -tt $SSH_OPTS2 "$REMOTE_HOST2" "cd $REMOTE_TEMP_DIR2 && chmod +x $BINARY_NAME && echo '$SUDO_PASSWORD' | sudo -S ./$BINARY_NAME $MAX_PROCESSES ." & SSH_PID2=$!
wait $SSH_PID2 || {
    echo "错误: 主机2执行失败"
    ssh "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2"
    exit 1
}
SSH_PID2=""

echo "Step 5: 复制结果文件到本地..."
scp $SSH_OPTS1 -r "$REMOTE_HOST1:$REMOTE_TEMP_DIR1/*" "$LOCAL_RESULT_DIR/host1/" & SSH_PID1=$!
wait $SSH_PID1 || {
    echo "错误: 无法从主机1复制结果文件"
    ssh "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1"
    exit 1
}
SSH_PID1=""

scp $SSH_OPTS2 -r "$REMOTE_HOST2:$REMOTE_TEMP_DIR2/*" "$LOCAL_RESULT_DIR/host2/" & SSH_PID2=$!
wait $SSH_PID2 || {
    echo "错误: 无法从主机2复制结果文件"
    ssh "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2"
    exit 1
}
SSH_PID2=""

echo "Step 6: 执行对比分析..."
echo "使用memory-analysis进行差异分析..."
./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME --diff "$LOCAL_RESULT_DIR/host1" "$LOCAL_RESULT_DIR/host2" || {
    echo "错误: 差异分析失败"
    exit 1
}

echo "完成！"
echo "结果文件保存在: $LOCAL_RESULT_DIR"
echo "差异分析报告: $LOCAL_RESULT_DIR/host2/diff_report_中文.md"
# cleanup会在脚本退出时自动执行
