#!/bin/bash

# 显示用法信息
usage() {
    echo "用法: $0 <主机(user@host)> <本地结果目录> [进程数量限制] [-d <第二台主机(user@host)>]"
    echo "示例:"
    echo "  $0 zccrs@127.0.0.1 /tmp/test"
    echo "  $0 zccrs@127.0.0.1 /tmp/test 5"
    echo "  $0 zccrs@127.0.0.1 /tmp/test -d zccrs@127.0.0.1"
    echo "  $0 zccrs@127.0.0.1 /tmp/test 5 -d zccrs@127.0.0.1"
    echo ""
    echo "说明:"
    echo "  此脚本会在指定主机上执行memory-analysis程序并收集信息。"
    echo "  如果通过 -d 参数指定第二台主机，则会在两台主机上分别执行并进行对比分析。"
    echo "  可选参数进程数量限制用于控制每台主机上采集的最大进程数。"
    exit 1
}

# 检查参数
if [ $# -lt 2 ] || [ $# -gt 5 ]; then
    usage
fi

# 解析参数
REMOTE_HOST1="$1"
LOCAL_RESULT_DIR="$2"
MAX_PROCESSES=""
REMOTE_HOST2=""

shift 2

while [ $# -gt 0 ]; do
    case "$1" in
        -d)
            shift
            if [ $# -eq 0 ]; then
                echo "错误: -d 参数需要指定主机"
                exit 1
            fi
            REMOTE_HOST2="$1"
            shift
            ;;
        *)
            # 如果第三个参数不是 -d，则认为是进程数量限制
            if [[ "$1" =~ ^[0-9]+$ ]]; then
                MAX_PROCESSES="--max-processes $1"
                shift
            else
                echo "错误: 无效的参数 '$1'"
                usage
            fi
            ;;
    esac
done

if [ -n "$MAX_PROCESSES" ] && [ -n "$REMOTE_HOST2" ]; then
    # 确保进程数量限制参数在 -d 参数之前
    if ! [[ "$*" =~ ^[0-9]+\ -d ]]; then
        echo "错误: 进程数量限制参数必须在 -d 参数之前"
        usage
    fi
fi
TIMESTAMP=$(date +%s)
REMOTE_TEMP_DIR1="/tmp/memory-analysis-1-$TIMESTAMP"
BINARY_NAME="memory-analysis"
SSH_CONTROL_PATH1="/tmp/ssh-memory-analysis-1-$$.socket"

if [ -n "$REMOTE_HOST2" ]; then
    REMOTE_TEMP_DIR2="/tmp/memory-analysis-2-$TIMESTAMP"
    SSH_CONTROL_PATH2="/tmp/ssh-memory-analysis-2-$$.socket"
fi

# 清理函数
cleanup() {
    # 检查主机1的SSH连接是否已建立
    if [ -S "$SSH_CONTROL_PATH1" ]; then
        # 终止远程进程
        if [ -n "$REMOTE_PID1" ]; then
            ssh $SSH_OPTS1 "$REMOTE_HOST1" "sudo kill -TERM $REMOTE_PID1" 2>/dev/null
        fi
        # 清理远程临时目录
        ssh $SSH_OPTS1 "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1" 2>/dev/null
        # 关闭SSH主连接
        ssh -O exit $SSH_OPTS1 "$REMOTE_HOST1" 2>/dev/null
    fi
    # 删除控制socket（无论连接是否建立都需要清理）
    rm -f "$SSH_CONTROL_PATH1"

    # 检查主机2的SSH连接是否已建立
    if [ -n "$REMOTE_HOST2" ]; then
        if [ -S "$SSH_CONTROL_PATH2" ]; then
            # 终止远程进程
            if [ -n "$REMOTE_PID2" ]; then
                ssh $SSH_OPTS2 "$REMOTE_HOST2" "sudo kill -TERM $REMOTE_PID2" 2>/dev/null
            fi
            # 清理远程临时目录
            ssh $SSH_OPTS2 "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2" 2>/dev/null
            # 关闭SSH主连接
            ssh -O exit $SSH_OPTS2 "$REMOTE_HOST2" 2>/dev/null
        fi
        # 删除控制socket（无论连接是否建立都需要清理）
        rm -f "$SSH_CONTROL_PATH2"
    fi

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
        wait $SSH_PID1 2>/dev/null
    fi
    if [ -n "$REMOTE_HOST2" ] && [ -n "$SSH_PID2" ]; then
        kill -TERM $SSH_PID2 2>/dev/null
        wait $SSH_PID2 2>/dev/null
    fi

    # 只在SSH连接成功建立后才执行完整的清理操作
    if [ -S "$SSH_CONTROL_PATH1" ] || [ -S "$SSH_CONTROL_PATH2" ]; then
        cleanup
    else
        # SSH连接尚未建立，只需要恢复终端状态
        stty sane 2>/dev/null || true
    fi
    exit 1
}

# 设置清理钩子
trap cleanup EXIT
trap handle_interrupt INT TERM QUIT

# 确保本地结果目录及子目录存在
mkdir -p "$LOCAL_RESULT_DIR/host1"
if [ -n "$REMOTE_HOST2" ]; then
    mkdir -p "$LOCAL_RESULT_DIR/host2"
fi

# SSH复用连接选项
SSH_OPTS1="-o ControlMaster=auto -o ControlPath=$SSH_CONTROL_PATH1 -o ControlPersist=yes"
if [ -n "$REMOTE_HOST2" ]; then
    SSH_OPTS2="-o ControlMaster=auto -o ControlPath=$SSH_CONTROL_PATH2 -o ControlPersist=yes"
fi

echo "Step 1: 测试SSH连接..."
# 测试SSH连接并建立复用
# 测试SSH连接并保存进程ID
ssh $SSH_OPTS1 "$REMOTE_HOST1" "echo '主机1连接成功'" & SSH_PID1=$!
wait $SSH_PID1 || {
    echo "错误: 无法连接到主机1"
    exit 1
}

if [ -n "$REMOTE_HOST2" ]; then
    ssh $SSH_OPTS2 "$REMOTE_HOST2" "echo '主机2连接成功'" & SSH_PID2=$!
    wait $SSH_PID2 || {
        echo "错误: 无法连接到主机2"
        exit 1
    }
fi

# 连接成功后清除进程ID
SSH_PID1=""
if [ -n "$REMOTE_HOST2" ]; then
    SSH_PID2=""
fi

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

if [ -n "$REMOTE_HOST2" ]; then
    ssh $SSH_OPTS2 "$REMOTE_HOST2" "mkdir -p $REMOTE_TEMP_DIR2" || {
        echo "错误: 无法在主机2上创建目录"
        exit 1
    }
fi

echo "Step 3: 复制程序到远程主机..."
scp $SSH_OPTS1 "./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME" "$REMOTE_HOST1:$REMOTE_TEMP_DIR1/" || {
    echo "错误: 无法复制程序到主机1"
    ssh "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1"
    exit 1
}

if [ -n "$REMOTE_HOST2" ]; then
    scp $SSH_OPTS2 "./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME" "$REMOTE_HOST2:$REMOTE_TEMP_DIR2/" || {
        echo "错误: 无法复制程序到主机2"
        ssh "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2"
        exit 1
    }
fi

echo "Step 4: 在远程主机上执行程序..."

# 获取sudo密码
read -s -p "请输入sudo密码: " SUDO_PASSWORD
echo

# 在主机1上启动采集
echo "在主机1上启动采集数据..."
ssh -tt $SSH_OPTS1 "$REMOTE_HOST1" "cd $REMOTE_TEMP_DIR1 && chmod +x $BINARY_NAME && echo '$SUDO_PASSWORD' | sudo -S ./$BINARY_NAME --log-level=debug $MAX_PROCESSES ." & SSH_PID1=$!

# 如果有第二台主机，同时启动采集
if [ -n "$REMOTE_HOST2" ]; then
    echo "在主机2上启动采集数据..."
    ssh -tt $SSH_OPTS2 "$REMOTE_HOST2" "cd $REMOTE_TEMP_DIR2 && chmod +x $BINARY_NAME && echo '$SUDO_PASSWORD' | sudo -S ./$BINARY_NAME --log-level=debug $MAX_PROCESSES ." & SSH_PID2=$!
fi

# 等待主机1完成并检查结果
wait $SSH_PID1
SSH_STATUS1=$?

if [ $SSH_STATUS1 -ne 0 ]; then
    echo "错误: 主机1执行失败，正在获取更多信息..."
    ssh $SSH_OPTS1 "$REMOTE_HOST1" "echo '$SUDO_PASSWORD' | sudo -S ls -l /boot/ /usr/lib/modules/ /usr/lib/boot/ 2>/dev/null"
    ssh $SSH_OPTS1 "$REMOTE_HOST1" "echo '$SUDO_PASSWORD' | sudo -S uname -a"
    ssh $SSH_OPTS1 "$REMOTE_HOST1" "echo '$SUDO_PASSWORD' | sudo -S id"
    ssh "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1"

    # 如果主机1失败且主机2正在运行，终止主机2的任务
    if [ -n "$REMOTE_HOST2" ] && [ -n "$SSH_PID2" ]; then
        echo "由于主机1失败，正在终止主机2的任务..."
        kill -TERM $SSH_PID2 2>/dev/null
        ssh $SSH_OPTS2 "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2" 2>/dev/null
    fi

    exit 1
fi
SSH_PID1=""

# 如果有第二台主机，等待其完成并检查结果
if [ -n "$REMOTE_HOST2" ]; then
    echo "等待主机2采集完成..."
    wait $SSH_PID2
    SSH_STATUS2=$?

    if [ $SSH_STATUS2 -ne 0 ]; then
        echo "错误: 主机2执行失败，正在获取更多信息..."
        ssh $SSH_OPTS2 "$REMOTE_HOST2" "echo '$SUDO_PASSWORD' | sudo -S ls -l /boot/ /usr/lib/modules/ /usr/lib/boot/ 2>/dev/null"
        ssh $SSH_OPTS2 "$REMOTE_HOST2" "echo '$SUDO_PASSWORD' | sudo -S uname -a"
        ssh $SSH_OPTS2 "$REMOTE_HOST2" "echo '$SUDO_PASSWORD' | sudo -S id"
        ssh "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2"
        exit 1
    fi
    SSH_PID2=""
    echo "两台主机的采集任务都已完成。"
else
    echo "主机1采集任务已完成。"
fi

echo "Step 5: 并行复制结果文件到本地..."
# 启动主机1的复制任务
echo "开始从主机1复制文件..."
scp $SSH_OPTS1 -r "$REMOTE_HOST1:$REMOTE_TEMP_DIR1/*" "$LOCAL_RESULT_DIR/host1/" & SSH_PID1=$!

# 如果有第二台主机，同时启动复制
if [ -n "$REMOTE_HOST2" ]; then
    echo "开始从主机2复制文件..."
    scp $SSH_OPTS2 -r "$REMOTE_HOST2:$REMOTE_TEMP_DIR2/*" "$LOCAL_RESULT_DIR/host2/" & SSH_PID2=$!
fi

# 等待主机1的复制完成
echo "等待主机1文件复制完成..."
wait $SSH_PID1 || {
    echo "错误: 无法从主机1复制结果文件"
    ssh "$REMOTE_HOST1" "rm -rf $REMOTE_TEMP_DIR1"
    # 如果主机2正在复制，等待其完成
    if [ -n "$REMOTE_HOST2" ] && [ -n "$SSH_PID2" ]; then
        wait $SSH_PID2
    fi
    exit 1
}
SSH_PID1=""

# 如果有第二台主机，等待其复制完成
if [ -n "$REMOTE_HOST2" ]; then
    echo "等待主机2文件复制完成..."
    wait $SSH_PID2 || {
        echo "错误: 无法从主机2复制结果文件"
        ssh "$REMOTE_HOST2" "rm -rf $REMOTE_TEMP_DIR2"
        exit 1
    }
    SSH_PID2=""
    echo "两台主机的文件都已复制完成。"
else
    echo "主机1的文件已复制完成。"
fi

if [ -n "$REMOTE_HOST2" ]; then
    echo "Step 6: 执行对比分析..."
    echo "使用memory-analysis进行差异分析..."
    ./target/x86_64-unknown-linux-gnu/release/$BINARY_NAME --diff "$LOCAL_RESULT_DIR/host1" "$LOCAL_RESULT_DIR/host2" || {
        echo "错误: 差异分析失败"
        exit 1
    }
fi

echo "完成！"
if [ -n "$REMOTE_HOST2" ]; then
    echo "结果文件保存在: $LOCAL_RESULT_DIR"
    echo "差异分析报告: $LOCAL_RESULT_DIR/host2/diff_report_中文.md"
else
    echo "结果文件保存在: $LOCAL_RESULT_DIR/host1"
fi
# cleanup会在脚本退出时自动执行
