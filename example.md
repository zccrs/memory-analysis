# 内存使用差异分析报告

## 名词解释

* **PSS (Proportional Set Size)**: 按比例分配的物理内存大小，将共享内存按使用比例分配给各个进程
* **RSS (Resident Set Size)**: 进程实际使用的物理内存大小，包含共享内存
* **新增进程**: 在新系统中出现，但在旧系统中不存在的进程
* **移除进程**: 在旧系统中存在，但在新系统中不存在的进程
* **内存占用变化**: 
  * 🔴 红色表示内存增加或新增项目
  * 🟢 绿色表示内存减少或移除项目

## 关于内存统计方式

本工具使用以下方式统计内存：

1. **进程内存计算方式**
   - 优先使用 PSS (Proportional Set Size) 来计算进程内存
   - PSS 会将共享内存按比例分配给各个进程，更准确反映实际占用
   - 这与 top 命令使用的 RSS (Resident Set Size) 不同
   - RSS 会将共享内存完全计入每个进程，可能导致总和偏大

2. **数据来源**
   - 进程 PSS/RSS：/proc/{pid}/smaps
   - 系统总内存：/proc/meminfo
   - 进程内存总和 = 所有进程的 PSS 总和
   - 内核内存 = 系统总使用内存 - 进程内存总和

3. **统计范围说明**
   - 可通过 --max-processes 参数限制采集进程数量
   - 被跳过的进程数量会记录在统计信息中

4. **进程状态说明**
   - 新增进程：在新系统中出现，但在旧系统中不存在的进程
   - 移除进程：在旧系统中存在，但在新系统中不存在的进程
   - 变化进程：在新旧系统中都存在，但内存使用或其他特征发生变化的进程

本报告是 Deepin 25 相对于 Deepin 20.9 的内存使用情况。

总进程数量（Deepin 25）：261
总进程数量（Deepin 20.9）：332
新增进程数量：131
移除进程数量：202

本报告对比了 Deepin 20.9 和 Deepin 25 的内存使用情况。

# 综述

## Deepin 20.9

- CPU信息：	: 13th Gen Intel(R) Core(TM) i5-13400
- 主机名：test-PC
- 内核版本：5.18.17-amd64-desktop-hwe
- 操作系统：Deepin 20.9
- 页大小：4.096 KB
- 系统内存：
  - 总内存：16.544382976 GB
  - 已使用内存：1.444737024 GB
  - 可用内存：15.099645952 GB
  - 内核内存：357.915648 MB
  - 进程总内存：1.086821376 GB
  - 共享内存总量：5.312512 MB
- 进程信息：
  - 总进程数：332
  - 内核进程：234
  - 系统进程：88 (非用户进程)
  - 用户进程：10
  - 被跳过的进程：0
- 内核文件信息：
  - 内核文件大小：11.772968 MB
  - Initrd文件大小：96.960332 MB

## Deepin 25

- CPU信息：	: 11th Gen Intel(R) Core(TM) i5-11300H @ 3.10GHz
- 主机名：localhost.localdomain
- 内核版本：6.6.84-amd64-desktop-hwe
- 操作系统：Deepin 25
- 页大小：4.096 KB
- 系统内存：
  - 总内存：16.53231616 GB
  - 已使用内存：2.062127104 GB
  - 可用内存：14.470189056 GB
  - 内核内存：797.75744 MB
  - 进程总内存：1.264369664 GB
  - 共享内存总量：4.194304 MB
- 进程信息：
  - 总进程数：261
  - 内核进程：152
  - 系统进程：52 (非用户进程)
  - 用户进程：57
  - 被跳过的进程：0
- 内核文件信息：
  - 内核文件大小：14.163272 MB
  - Initrd文件大小：130.768572 MB

## 内存变化总览

```diff
# 内核内存变化
  内核进程内存变化：0 B
+ 内核文件变化：36.198544 MB
  总内核内存变化：36.198544 MB

# 系统内存变化
- 系统进程内存变化：-362.217472 MB

# 用户内存变化
+ 用户进程内存变化：539.76576 MB

# 其他系统内存变化
+ 其他内存变化：403.643248 MB

-------------------
  总内存变化：  617.39008 MB
```

**内存变化详细分析**

1. **进程内存变化**
   - 新增进程：745.496576 MB
   - 删除进程：-530.591744 MB
   - 现有进程变化：-37.356544 MB
   - 小计：177.548288 MB

2. **可执行文件和动态库变化**
   - 动态库变化：388.580368 MB
   - 可执行文件变化：-7.043416 MB
   - 小计：381.536952 MB

3. **其他资源变化**
   - 文件句柄：1.531904 MB
   - 共享内存：-20.48 KB
   - 小计：1.511424 MB

4. **内核内存变化**
   - 内核空间：56.793416 MB
   - 包括：内核数据结构、缓存、内核模块等

## 系统配置差异

- ⚠️ 内核版本发生变化
- 共享内存变化：-1.118208 MB

## 内存变化构成

### 总体变化分类

| 变化类型 | 内存变化 | 占比 |
|---------|----------|------|
| 新增进程 | 745.496576 MB | 44.0% |
| 删除进程 | -530.591744 MB | 31.0% |
| 进程变化 | -37.356544 MB | 2.0% |
| 动态库变化 | 388.580368 MB | 23.0% |
| 可执行文件变化 | -7.043416 MB | 0.0% |
| 文件句柄变化 | 1.531904 MB | 0.0% |
| 共享内存变化 | -20.48 KB | 0.0% |

# 详细内存分析

## 内核内存变化

### 内核文件变化

| 文件 | 原大小 | 新大小 | 变化 |
|------|---------|---------|-------|
| vmlinuz | 11.772968 MB | 14.163272 MB | 🔴 2.390304 MB |
| initramfs | 96.960332 MB | 130.768572 MB | 🔴 33.80824 MB |

### 内核线程变化

#### 新增内核线程

* 🔴 514:irq/140-iwlwifi:default_queue
* 🔴 270711:kworker/u16:6-events_unbound
* 🔴 78:kworker/R-blkcg
* 🔴 271850:unknown
* 🔴 515:irq/141-iwlwifi:queue_1
* 🔴 522:irq/148-iwlwifi:queue_8
* 🔴 272089:kworker/3:1-cgroup_destroy
* 🔴 272279:kworker/5:1-i915-unordered
* 🔴 523:irq/149-iwlwifi:exception
* 🔴 270474:kworker/u16:3-ext4-rsv-conversion
* 🔴 84:kworker/R-devfr
* 🔴 185:kworker/R-crypt
* 🔴 272260:kworker/1:1
* 🔴 247:kworker/R-ext4-
* 🔴 112:kworker/R-kstrp
* 🔴 270485:kworker/3:0-i915-unordered
* 🔴 118:kworker/R-charg
* 🔴 12:kworker/R-mm_pe
* 🔴 272395:kworker/u17:1-rb_allocator
* 🔴 103:kworker/R-mld
* 🔴 3:pool_workqueue_release
* 🔴 79:kworker/R-tpm_d
* 🔴 65:kworker/R-inet_
* 🔴 270723:kworker/5:3-i915-unordered
* 🔴 218228:kworker/2:0-mm_percpu_wq
* 🔴 272288:kworker/u17:0-rb_allocator
* 🔴 516:irq/142-iwlwifi:queue_2
* 🔴 80:kworker/R-ata_s
* 🔴 517:irq/143-iwlwifi:queue_3
* 🔴 6:kworker/R-slub_
* 🔴 520:irq/146-iwlwifi:queue_6
* 🔴 196:card0-crtc3
* 🔴 7:kworker/R-netns
* 🔴 272463:kworker/7:2
* 🔴 270713:kworker/6:1-events
* 🔴 272328:kworker/0:0
* 🔴 18:rcub/0
* 🔴 98:kworker/R-nvme-
* 🔴 2349:psimon
* 🔴 4:kworker/R-rcu_g
* 🔴 518:irq/144-iwlwifi:queue_4
* 🔴 186:irq/134-GXTP738X:00
* 🔴 83:kworker/R-edac-
* 🔴 5:kworker/R-rcu_p
* 🔴 266150:kworker/1:2-mm_percpu_wq
* 🔴 272574:unknown
* 🔴 272007:kworker/6:0-mm_percpu_wq
* 🔴 187:irq/135-GXTP7863:00
* 🔴 81:kworker/R-md
* 🔴 71:kworker/R-write
* 🔴 1520:runuser
* 🔴 534:irq/150-AudioDSP
* 🔴 582:kworker/R-ext4-
* 🔴 270664:runuser
* 🔴 505:kworker/R-cfg80
* 🔴 272151:kworker/u16:2-events_power_efficient
* 🔴 271654:kworker/2:3-cgroup_destroy
* 🔴 105:kworker/R-ipv6_
* 🔴 100:kworker/R-nvme-
* 🔴 521:irq/147-iwlwifi:queue_7
* 🔴 272345:kworker/u16:0-flush-259:0
* 🔴 270306:kworker/0:1-events
* 🔴 101:kworker/R-vfio-
* 🔴 214382:kworker/u17:2-rb_allocator
* 🔴 269157:kworker/4:1-cgroup_destroy
* 🔴 277:kworker/R-ext4-
* 🔴 218071:irq/139-mei_me
* 🔴 99:kworker/R-nvme-
* 🔴 77:kworker/R-kbloc
* 🔴 142:kworker/2:1H-kblockd
* 🔴 271986:kworker/7:0-events
* 🔴 96:kworker/R-acpi_
* 🔴 581:jbd2/nvme0n1p2-8
* 🔴 97:hwrng
* 🔴 272431:kworker/2:1-i915-unordered
* 🔴 82:kworker/R-md_bi
* 🔴 434:psimon
* 🔴 192:kworker/R-ttm
* 🔴 76:kworker/R-kinte
* 🔴 519:irq/145-iwlwifi:queue_5

#### 移除内核线程

* 🟢 710472:kworker/5:1-cgroup_destroy
* 🟢 681802:kworker/6:1
* 🟢 122:tpm_dev_wq
* 🟢 168:scsi_eh_1
* 🟢 661954:kworker/u32:2-events_unbound
* 🟢 125:edac-poller
* 🟢 934:irq/161-iwlwifi:exception
* 🟢 161:nvme-reset-wq
* 🟢 702373:kworker/11:2-mm_percpu_wq
* 🟢 700447:kworker/10:1-events
* 🟢 186:kworker/12:1H-events_highpri
* 🟢 81:idle_inject/11
* 🟢 691840:kworker/1:1-mm_percpu_wq
* 🟢 171:scsi_tmf_2
* 🟢 174:scsi_eh_4
* 🟢 313418:kworker/14:0-mm_percpu_wq
* 🟢 103:kworker/14:0H-events_highpri
* 🟢 915:irq/149-iwlwifi:queue_3
* 🟢 716654:kworker/15:2
* 🟢 349217:kworker/15:0-events
* 🟢 390:jbd2/nvme0n1p3-8
* 🟢 93:idle_inject/13
* 🟢 175:scsi_tmf_4
* 🟢 89:ksoftirqd/12
* 🟢 158:irq/123-aerdrv
* 🟢 913:irq/147-iwlwifi:queue_1
* 🟢 680884:kworker/4:2-events
* 🟢 92:cpuhp/13
* 🟢 266:kworker/8:1H-events_highpri
* 🟢 80:cpuhp/11
* 🟢 77:ksoftirqd/10
* 🟢 602400:kworker/1:2-events
* 🟢 85:kworker/11:0H-events_highpri
* 🟢 308:kworker/14:1H-events_highpri
* 🟢 929:irq/157-iwlwifi:queue_11
* 🟢 70:migration/9
* 🟢 62:cpuhp/8
* 🟢 678480:kworker/6:2-events
* 🟢 918:irq/151-iwlwifi:queue_5
* 🟢 391:ext4-rsv-conver
* 🟢 319:kworker/11:1H-events_highpri
* 🟢 715100:kworker/5:2-cgroup_destroy
* 🟢 167:scsi_tmf_0
* 🟢 716135:kworker/11:0-cgroup_destroy
* 🟢 712543:kworker/11:1-cgroup_destroy
* 🟢 334:raid5wq
* 🟢 119:kintegrityd
* 🟢 6123:ext4-rsv-conver
* 🟢 111:inet_frag_wq
* 🟢 914:irq/148-iwlwifi:queue_2
* 🟢 165:kworker/2:1H-events_highpri
* 🟢 104:cpuhp/15
* 🟢 673612:kworker/13:1-events
* 🟢 115:writeback
* 🟢 69:idle_inject/9
* 🟢 711370:kworker/3:0-events
* 🟢 181:scsi_tmf_7
* 🟢 716715:kworker/12:1-mm_percpu_wq
* 🟢 714878:kworker/10:0-events
* 🟢 76:migration/10
* 🟢 120:kblockd
* 🟢 347:kworker/10:1H-events_highpri
* 🟢 75:idle_inject/10
* 🟢 533:cfg80211
* 🟢 713928:kworker/13:2-events
* 🟢 921:irq/153-iwlwifi:queue_7
* 🟢 126:devfreq_wq
* 🟢 713923:kworker/9:2-mld
* 🟢 3:rcu_gp
* 🟢 716714:kworker/0:0-cgroup_destroy
* 🟢 680545:kworker/5:0-mm_percpu_wq
* 🟢 179:scsi_tmf_6
* 🟢 97:kworker/13:0H-events_highpri
* 🟢 160:nvme-wq
* 🟢 98:cpuhp/14
* 🟢 712598:kworker/7:2-events
* 🟢 109:kworker/15:0H-events_highpri
* 🟢 185:mld
* 🟢 714437:kworker/12:0-cgroup_destroy
* 🟢 712259:kworker/u33:2-rb_allocator
* 🟢 180:scsi_eh_7
* 🟢 172:scsi_eh_3
* 🟢 933:irq/160-iwlwifi:queue_14
* 🟢 74:cpuhp/10
* 🟢 602399:kworker/3:1-mm_percpu_wq
* 🟢 710086:kworker/12:2-events
* 🟢 182:vfio-irqfd-clea
* 🟢 715095:kworker/u32:0
* 🟢 177:scsi_tmf_5
* 🟢 4:rcu_par_gp
* 🟢 65:ksoftirqd/8
* 🟢 159:acpi_thermal_pm
* 🟢 95:ksoftirqd/13
* 🟢 662083:kworker/4:1-events
* 🟢 170:scsi_eh_2
* 🟢 5:netns
* 🟢 10:mm_percpu_wq
* 🟢 420:kworker/13:1H-events_highpri
* 🟢 920:irq/152-iwlwifi:queue_6
* 🟢 123:ata_sff
* 🟢 321:spi0
* 🟢 100:migration/14
* 🟢 64:migration/8
* 🟢 192:kstrp
* 🟢 712508:kworker/u32:1-events_unbound
* 🟢 712247:kworker/14:1-events
* 🟢 169:scsi_tmf_1
* 🟢 162:nvme-delete-wq
* 🟢 68:cpuhp/9
* 🟢 91:kworker/12:0H-events_highpri
* 🟢 931:irq/158-iwlwifi:queue_12
* 🟢 87:idle_inject/12
* 🟢 187:ipv6_addrconf
* 🟢 82:migration/11
* 🟢 166:scsi_eh_0
* 🟢 714932:kworker/8:1-events
* 🟢 709791:kworker/15:1-events
* 🟢 83:ksoftirqd/11
* 🟢 932:irq/159-iwlwifi:queue_13
* 🟢 667212:kworker/9:0-events
* 🟢 79:kworker/10:0H-events_highpri
* 🟢 927:irq/156-iwlwifi:queue_10
* 🟢 199:zswap-shrink
* 🟢 522:irq/145-mei_me
* 🟢 916:irq/150-iwlwifi:queue_4
* 🟢 912:irq/146-iwlwifi:default_queue
* 🟢 63:idle_inject/8
* 🟢 924:irq/154-iwlwifi:queue_8
* 🟢 715085:kworker/2:2
* 🟢 101:ksoftirqd/14
* 🟢 178:scsi_eh_6
* 🟢 176:scsi_eh_5
* 🟢 711305:kworker/2:0-events
* 🟢 157:irq/122-aerdrv
* 🟢 121:blkcg_punt_bio
* 🟢 173:scsi_tmf_3
* 🟢 716706:kworker/14:2
* 🟢 665211:kworker/2:1-events
* 🟢 374:kworker/9:1H-events_highpri
* 🟢 99:idle_inject/14
* 🟢 106:migration/15
* 🟢 86:cpuhp/12
* 🟢 710261:kworker/0:1-cgroup_destroy
* 🟢 67:kworker/8:0H-events_highpri
* 🟢 662985:kworker/8:0-events
* 🟢 536:cryptd
* 🟢 712673:kworker/10:2-mm_percpu_wq
* 🟢 105:idle_inject/15
* 🟢 88:migration/12
* 🟢 926:irq/155-iwlwifi:queue_9
* 🟢 94:migration/13
* 🟢 71:ksoftirqd/9
* 🟢 6104:ext4-rsv-conver
* 🟢 200:kworker/u33:0-rb_allocator
* 🟢 124:md
* 🟢 73:kworker/9:0H-events_highpri
* 🟢 107:ksoftirqd/15
* 🟢 279:kworker/15:1H-kblockd
* 🟢 248:charger_manager
* 🟢 710441:kworker/8:2-events
* 🟢 666000:kworker/u32:3-events_power_efficient
* 🟢 658088:kworker/0:2-events

## 系统进程变化

系统进程（不含内核进程）的内存变化分析：因新增进程（17个）增加120.361984 MB内存占用，因减少进程（34个）减少了518.812672 MB内存占用，变化进程（35个）因为自身内存占用的变化增加了36.233216 MB内存占用，最终系统进程总体内存减少了362.217472 MB。这些系统进程主要负责提供系统服务、设备管理、网络通信等基础功能，运行在特权模式下。

### 新增进程

#### 730:/usr/bin/deepin-anything-server (🔴 +39.206912 MB)
- 可执行文件路径：/usr/bin/deepin-anything-server
- 打开文件数：11
- 加载动态库：40 个

#### 939:/usr/libexec/lastore-daemon/lastore-daemon (🔴 +16.989184 MB)
- 可执行文件路径：/usr/libexec/lastore-daemon/lastore-daemon
- 打开文件数：14
- 加载动态库：57 个

#### 719:/usr/bin/deepin-service-manager (🔴 +14.631936 MB)
- 可执行文件路径：/usr/bin/deepin-service-manager
- 打开文件数：18
- 加载动态库：231 个

#### 2350:/usr/bin/deepin-service-manager (🔴 +7.66464 MB)
- 可执行文件路径：/usr/bin/deepin-service-manager
- 打开文件数：9
- 加载动态库：142 个

#### 658:/usr/bin/deepin-service-manager (🔴 +7.007232 MB)
- 可执行文件路径：/usr/bin/deepin-service-manager
- 打开文件数：14
- 加载动态库：95 个

#### 2170:/usr/libexec/linglong/ll-package-manager (🔴 +6.045696 MB)
- 可执行文件路径：/usr/libexec/linglong/ll-package-manager
- 打开文件数：12
- 加载动态库：78 个

#### 272479:/usr/libexec/samba/rpcd_lsad (🔴 +5.510144 MB)
- 可执行文件路径：/usr/libexec/samba/rpcd_lsad
- 打开文件数：16
- 加载动态库：148 个

#### 272481:/usr/libexec/samba/rpcd_lsad (🔴 +5.484544 MB)
- 可执行文件路径：/usr/libexec/samba/rpcd_lsad
- 打开文件数：16
- 加载动态库：148 个

#### 622:/usr/lib/deepin-daemon/uos-recovery-service (🔴 +4.430848 MB)
- 可执行文件路径：/usr/lib/deepin-daemon/uos-recovery-service
- 打开文件数：9
- 加载动态库：65 个

#### 272470:/usr/libexec/samba/samba-dcerpcd (🔴 +4.187136 MB)
- 可执行文件路径：/usr/libexec/samba/samba-dcerpcd
- 打开文件数：37
- 加载动态库：123 个

#### 1550:/usr/lib/dde-api-proxy/dbus-proxy/dde-api-dbus-proxy-v1 (🔴 +3.085312 MB)
- 可执行文件路径：/usr/lib/dde-api-proxy/dbus-proxy/dde-api-dbus-proxy-v1
- 打开文件数：20
- 加载动态库：39 个

#### 272467:/usr/lib/openssh/sshd-session (🔴 +2.576384 MB)
- 可执行文件路径：/usr/lib/openssh/sshd-session
- 打开文件数：13
- 加载动态库：52 个

#### 270509:/usr/sbin/lightdm (🔴 +1.256448 MB)
- 可执行文件路径：/usr/sbin/lightdm
- 打开文件数：18
- 加载动态库：26 个

#### 614:/usr/sbin/iio-sensor-proxy (🔴 +1.046528 MB)
- 可执行文件路径：/usr/sbin/iio-sensor-proxy
- 打开文件数：9
- 加载动态库：16 个

#### 1237:/usr/lib/uos-ste/uos-ste-resourced (🔴 +841.728 KB)
- 可执行文件路径：/usr/lib/uos-ste/uos-ste-resourced
- 打开文件数：10
- 加载动态库：20 个

#### 602:/usr/sbin/acpid (🔴 +226.304 KB)
- 可执行文件路径：/usr/sbin/acpid
- 打开文件数：18
- 加载动态库：2 个

#### 617:/usr/sbin/seatd (🔴 +171.008 KB)
- 可执行文件路径：/usr/sbin/seatd
- 打开文件数：7
- 加载动态库：2 个

### 移除进程

#### 4671:/usr/bin/deepin-devicemanager-server (🟢 -128.884736 MB)
- 可执行文件路径：/usr/bin/deepin-devicemanager-server
- 打开文件数：28
- 加载动态库：111 个

#### 712781:/usr/bin/dde-desktop (🟢 -105.24672 MB)
- 可执行文件路径：/usr/bin/dde-desktop
- 打开文件数：32
- 加载动态库：276 个

#### 712840:/usr/bin/dde-dock (🟢 -47.91296 MB)
- 可执行文件路径：/usr/bin/dde-dock
- 打开文件数：35
- 加载动态库：222 个

#### 712672:/usr/lib/xorg/Xorg (🟢 -43.792384 MB)
- 可执行文件路径：/usr/lib/xorg/Xorg
- 打开文件数：77
- 加载动态库：89 个

#### 712694:/usr/bin/startdde (🟢 -26.831872 MB)
- 可执行文件路径：/usr/bin/startdde
- 打开文件数：26
- 加载动态库：93 个

#### 713548:/opt/apps/com.pantum.pantum/files/bin/pantum_scanapp (🟢 -25.450496 MB)
- 可执行文件路径：/opt/apps/com.pantum.pantum/files/bin/pantum_scanapp
- 打开文件数：21
- 加载动态库：141 个

#### 713539:/opt/apps/com.pantum.pantum/files/bin/pantum_scan (🟢 -23.683072 MB)
- 可执行文件路径：/opt/apps/com.pantum.pantum/files/bin/pantum_scan
- 打开文件数：20
- 加载动态库：141 个

#### 713408:/usr/lib/deepin-daemon/dde-osd (🟢 -14.232576 MB)
- 可执行文件路径：/usr/lib/deepin-daemon/dde-osd
- 打开文件数：20
- 加载动态库：137 个

#### 713630:/usr/lib/x86_64-linux-gnu/bamf/bamfdaemon (🟢 -13.425664 MB)
- 可执行文件路径：/usr/lib/x86_64-linux-gnu/bamf/bamfdaemon
- 打开文件数：11
- 加载动态库：83 个

#### 713458:/opt/apps/com.pantum.pantum/files/bin/pantum_navi (🟢 -10.990592 MB)
- 可执行文件路径：/opt/apps/com.pantum.pantum/files/bin/pantum_navi
- 打开文件数：20
- 加载动态库：139 个

#### 713475:/usr/bin/deepin-home-daemon (🟢 -9.675776 MB)
- 可执行文件路径：/usr/bin/deepin-home-daemon
- 打开文件数：19
- 加载动态库：158 个

#### 682362:/usr/bin/x11vnc (🟢 -9.286656 MB)
- 可执行文件路径：/usr/bin/x11vnc
- 打开文件数：13
- 加载动态库：52 个

#### 713467:/usr/bin/dde-printer-helper (🟢 -9.206784 MB)
- 可执行文件路径：/usr/bin/dde-printer-helper
- 打开文件数：28
- 加载动态库：147 个

#### 710726:/usr/bin/deepin-home-appstore-daemon (🟢 -8.240128 MB)
- 可执行文件路径：/usr/bin/deepin-home-appstore-daemon
- 打开文件数：29
- 加载动态库：55 个

#### 6432:/usr/lib/deepin-deepinid-daemon/deepin-sync-helper (🟢 -7.815168 MB)
- 可执行文件路径：/usr/lib/deepin-deepinid-daemon/deepin-sync-helper
- 打开文件数：6
- 加载动态库：4 个

#### 712844:/usr/bin/deepin-wm-dbus (🟢 -5.168128 MB)
- 可执行文件路径：/usr/bin/deepin-wm-dbus
- 打开文件数：15
- 加载动态库：99 个

#### 712984:/usr/bin/kglobalaccel5 (🟢 -4.47488 MB)
- 可执行文件路径：/usr/bin/kglobalaccel5
- 打开文件数：11
- 加载动态库：89 个

#### 6557:/usr/sbin/winbindd (🟢 -3.352576 MB)
- 可执行文件路径：/usr/sbin/winbindd
- 打开文件数：27
- 加载动态库：124 个

#### 713526:/usr/bin/deepin-appstore-session-daemon (🟢 -2.741248 MB)
- 可执行文件路径：/usr/bin/deepin-appstore-session-daemon
- 打开文件数：12
- 加载动态库：49 个

#### 4236:/usr/sbin/rsyslogd (🟢 -2.725888 MB)
- 可执行文件路径：/usr/sbin/rsyslogd
- 打开文件数：15
- 加载动态库：22 个

#### 613457:/usr/sbin/dhclient (🟢 -2.081792 MB)
- 可执行文件路径：/usr/sbin/dhclient
- 打开文件数：7
- 加载动态库：8 个

#### 613455:/usr/sbin/dhclient (🟢 -2.077696 MB)
- 可执行文件路径：/usr/sbin/dhclient
- 打开文件数：8
- 加载动态库：8 个

#### 713473:/usr/bin/deepin-system-monitor-daemon (🟢 -1.885184 MB)
- 可执行文件路径：/usr/bin/deepin-system-monitor-daemon
- 打开文件数：8
- 加载动态库：37 个

#### 710415:/usr/sbin/sshd (🟢 -1.765376 MB)
- 可执行文件路径：/usr/sbin/sshd
- 打开文件数：10
- 加载动态库：52 个

#### 716720:/usr/sbin/sshd (🟢 -1.739776 MB)
- 可执行文件路径：/usr/sbin/sshd
- 打开文件数：10
- 加载动态库：52 个

#### 712766:/usr/lib/gvfs/gvfsd-fuse (🟢 -1.399808 MB)
- 可执行文件路径：/usr/lib/gvfs/gvfsd-fuse
- 打开文件数：11
- 加载动态库：25 个

#### 4899:/usr/sbin/sshd (🟢 -1.262592 MB)
- 可执行文件路径：/usr/sbin/sshd
- 打开文件数：6
- 加载动态库：30 个

#### 6634:/usr/sbin/smbd (🟢 -1.158144 MB)
- 可执行文件路径：/usr/sbin/smbd
- 打开文件数：22
- 加载动态库：138 个

#### 713629:/usr/bin/bash (🟢 -584.704 KB)
- 可执行文件路径：/usr/bin/bash
- 打开文件数：5
- 加载动态库：4 个

#### 712710:/usr/bin/dbus-launch (🟢 -548.864 KB)
- 可执行文件路径：/usr/bin/dbus-launch
- 打开文件数：7
- 加载动态库：18 个

#### 712745:/usr/bin/dbus-launch (🟢 -548.864 KB)
- 可执行文件路径：/usr/bin/dbus-launch
- 打开文件数：7
- 加载动态库：18 个

#### 712671:/usr/bin/xinit (🟢 -327.68 KB)
- 可执行文件路径：/usr/bin/xinit
- 打开文件数：5
- 加载动态库：10 个

#### 712652:/usr/bin/dash (🟢 -157.696 KB)
- 可执行文件路径：/usr/bin/dash
- 打开文件数：5
- 加载动态库：2 个

#### 4224:/usr/bin/dash (🟢 -136.192 KB)
- 可执行文件路径：/usr/bin/dash
- 打开文件数：5
- 加载动态库：2 个

### 变化进程

#### 682361:/usr/lib/xorg/Xorg -> 270516:/usr/lib/xorg/Xorg (🔴 32.431104 MB)
- 可执行文件路径：/usr/lib/xorg/Xorg
- 打开文件数：91
- 加载动态库：83 个

#### 6522:/usr/sbin/nmbd -> 864:/usr/sbin/nmbd (🟢 -13.656064 MB)
- 可执行文件路径：/usr/sbin/nmbd
- 打开文件数：21
- 加载动态库：110 个

#### 444:/usr/lib/systemd/systemd-journald -> 386:/usr/lib/systemd/systemd-journald (🔴 10.69568 MB)
- 可执行文件路径：/usr/lib/systemd/systemd-journald
- 打开文件数：91
- 加载动态库：22 个

#### 6438:/usr/lib/deepin-authenticate/deepin-authentication -> 1489:/usr/lib/deepin-authenticate/deepin-authentication (🟢 -7.117824 MB)
- 可执行文件路径：/usr/lib/deepin-authenticate/deepin-authentication
- 打开文件数：31
- 加载动态库：62 个

#### 4916:/usr/lib/deepin-daemon/dde-system-daemon -> 935:/usr/lib/deepin-daemon/dde-system-daemon (🟢 -6.460416 MB)
- 可执行文件路径：/usr/lib/deepin-daemon/dde-system-daemon
- 打开文件数：73
- 加载动态库：65 个

#### 712711:/usr/bin/dbus-daemon -> 607:/usr/bin/dbus-daemon (🔴 4.174848 MB)
- 可执行文件路径：/usr/bin/dbus-daemon
- 打开文件数：99
- 加载动态库：19 个

#### 5142:/usr/bin/dde-dconfig-daemon -> 718:/usr/bin/dde-dconfig-daemon (🔴 4.164608 MB)
- 可执行文件路径：/usr/bin/dde-dconfig-daemon
- 打开文件数：9
- 加载动态库：30 个

#### 710423:/usr/lib/systemd/systemd -> 1:/usr/lib/systemd/systemd (🔴 3.41504 MB)
- 可执行文件路径：/usr/lib/systemd/systemd
- 打开文件数：233
- 加载动态库：24 个

#### 716735:/tmp/memory-analysis-1-1745493999/memory-analysis -> 272508:/tmp/memory-analysis-2-1745493999/memory-analysis (🔴 3.346432 MB)
- 可执行文件路径：/tmp/memory-analysis-2-1745493999/memory-analysis
- 打开文件数：12
- 加载动态库：0 个

#### 4992:/usr/lib/upower/upowerd -> 820:/usr/libexec/upowerd (🔴 1.803264 MB)
- 可执行文件路径：/usr/libexec/upowerd
- 打开文件数：12
- 加载动态库：23 个

#### 6562:/usr/sbin/winbindd -> 877:/usr/sbin/winbindd (🔴 1.779712 MB)
- 可执行文件路径：/usr/sbin/winbindd
- 打开文件数：24
- 加载动态库：136 个

#### 6635:/usr/sbin/winbindd -> 871:/usr/sbin/winbindd (🔴 1.620992 MB)
- 可执行文件路径：/usr/sbin/winbindd
- 打开文件数：25
- 加载动态库：133 个

#### 4241:/usr/sbin/wpa_supplicant -> 663:/usr/sbin/wpa_supplicant (🔴 1.547264 MB)
- 可执行文件路径：/usr/sbin/wpa_supplicant
- 打开文件数：20
- 加载动态库：18 个

#### 4220:/usr/lib/accountsservice/accounts-daemon -> 601:/usr/libexec/accounts-daemon (🟢 -1.496064 MB)
- 可执行文件路径：/usr/libexec/accounts-daemon
- 打开文件数：9
- 加载动态库：22 个

#### 682356:/usr/sbin/lightdm -> 270670:/usr/sbin/lightdm (🔴 1.366016 MB)
- 可执行文件路径：/usr/sbin/lightdm
- 打开文件数：14
- 加载动态库：51 个

#### 6626:/usr/sbin/smbd -> 881:/usr/sbin/smbd (🟢 -929.792 KB)
- 可执行文件路径：/usr/sbin/smbd
- 打开文件数：36
- 加载动态库：140 个

#### 4794:/usr/lib/policykit-1/polkitd -> 615:/usr/lib/polkit-1/polkitd (🟢 -838.656 KB)
- 可执行文件路径：/usr/lib/polkit-1/polkitd
- 打开文件数：12
- 加载动态库：29 个

#### 4229:/usr/sbin/NetworkManager -> 656:/usr/sbin/NetworkManager (🟢 -790.528 KB)
- 可执行文件路径：/usr/sbin/NetworkManager
- 打开文件数：28
- 加载动态库：68 个

#### 716723:/usr/sbin/sshd -> 766:/usr/sbin/sshd (🔴 672.768 KB)
- 可执行文件路径：/usr/sbin/sshd
- 打开文件数：10
- 加载动态库：6 个

#### 457:/usr/bin/udevadm -> 422:/usr/bin/udevadm (🟢 -553.984 KB)
- 可执行文件路径：/usr/bin/udevadm
- 打开文件数：16
- 加载动态库：11 个

#### 4240:/usr/lib/udisks2/udisksd -> 621:/usr/libexec/udisks2/udisksd (🟢 -481.28 KB)
- 可执行文件路径：/usr/libexec/udisks2/udisksd
- 打开文件数：19
- 加载动态库：62 个

#### 4219:/usr/sbin/ModemManager -> 693:/usr/sbin/ModemManager (🔴 439.296 KB)
- 可执行文件路径：/usr/sbin/ModemManager
- 打开文件数：12
- 加载动态库：80 个

#### 6636:/usr/sbin/winbindd -> 898:/usr/sbin/winbindd (🟢 -405.504 KB)
- 可执行文件路径：/usr/sbin/winbindd
- 打开文件数：22
- 加载动态库：133 个

#### 4237:/usr/sbin/smartd -> 618:/usr/sbin/smartd (🔴 343.04 KB)
- 可执行文件路径：/usr/sbin/smartd
- 打开文件数：4
- 加载动态库：15 个

#### 4235:/usr/lib/bluetooth/bluetoothd -> 605:/usr/libexec/bluetooth/bluetoothd (🔴 322.56 KB)
- 可执行文件路径：/usr/libexec/bluetooth/bluetoothd
- 打开文件数：40
- 加载动态库：21 个

#### 1658:/usr/lib/systemd/systemd-timesyncd -> 596:/usr/lib/systemd/systemd-timesyncd (🔴 307.2 KB)
- 可执行文件路径：/usr/lib/systemd/systemd-timesyncd
- 打开文件数：18
- 加载动态库：27 个

#### 6633:/usr/sbin/smbd -> 893:/usr/sbin/smbd (🔴 291.84 KB)
- 可执行文件路径：/usr/sbin/smbd
- 打开文件数：22
- 加载动态库：139 个

#### 244891:/usr/sbin/cupsd -> 753:/usr/sbin/cupsd (🔴 230.4 KB)
- 可执行文件路径：/usr/sbin/cupsd
- 打开文件数：11
- 加载动态库：35 个

#### 4239:/usr/lib/systemd/systemd-logind -> 620:/usr/lib/systemd/systemd-logind (🔴 206.848 KB)
- 可执行文件路径：/usr/lib/systemd/systemd-logind
- 打开文件数：35
- 加载动态库：22 个

#### 6637:/usr/sbin/smbd -> 894:/usr/sbin/smbd (🟢 -158.72 KB)
- 可执行文件路径：/usr/sbin/smbd
- 打开文件数：23
- 加载动态库：139 个

#### 6297:/usr/sbin/ipwatchd -> 218229:/usr/sbin/ipwatchd (🟢 -140.288 KB)
- 可执行文件路径：/usr/sbin/ipwatchd
- 打开文件数：7
- 加载动态库：12 个

#### 4223:/usr/sbin/avahi-daemon -> 604:/usr/sbin/avahi-daemon (🔴 71.68 KB)
- 可执行文件路径：/usr/sbin/avahi-daemon
- 打开文件数：18
- 加载动态库：17 个

#### 712778:/usr/bin/dash -> 659:/usr/bin/dash (🔴 41.984 KB)
- 可执行文件路径：/usr/bin/dash
- 打开文件数：5
- 加载动态库：2 个

#### 4226:/usr/sbin/cron -> 606:/usr/sbin/cron (🟢 -23.552 KB)
- 可执行文件路径：/usr/sbin/cron
- 打开文件数：6
- 加载动态库：7 个

#### 4243:/usr/sbin/avahi-daemon -> 616:/usr/sbin/avahi-daemon (🔴 13.312 KB)
- 可执行文件路径：/usr/sbin/avahi-daemon
- 打开文件数：7
- 加载动态库：17 个

## 用户进程变化

用户进程的内存变化分析：因新增进程（34个）增加625.134592 MB内存占用，因减少进程（6个）减少了11.779072 MB内存占用，变化进程（23个）因为自身内存占用的变化减少了73.58976 MB内存占用，最终用户进程总体内存增加了539.76576 MB。这些用户进程运行在较低权限的用户空间，主要是用户的日常应用程序、开发工具等。

### 新增进程

#### 271235:/usr/bin/dde-shell (🔴 +175.736832 MB)
- 可执行文件路径：/usr/bin/dde-shell
- 打开文件数：59
- 加载动态库：268 个

#### 271220:/usr/bin/dde-shell (🔴 +79.279104 MB)
- 可执行文件路径：/usr/bin/dde-shell
- 打开文件数：41
- 加载动态库：226 个

#### 271482:/opt/apps/com.cpis/bin/cpis-panel-service (🔴 +72.546304 MB)
- 可执行文件路径：/opt/apps/com.cpis/bin/cpis-panel-service
- 打开文件数：14
- 加载动态库：93 个

#### 272353:/usr/bin/deepin-screensaver (🔴 +37.891072 MB)
- 可执行文件路径：/usr/bin/deepin-screensaver
- 打开文件数：0
- 加载动态库：158 个

#### 271128:/usr/lib/x86_64-linux-gnu/libexec/kglobalacceld (🔴 +32.326656 MB)
- 可执行文件路径：/usr/lib/x86_64-linux-gnu/libexec/kglobalacceld
- 打开文件数：24
- 加载动态库：179 个

#### 271467:/usr/libexec/trayplugin-loader (🔴 +30.1056 MB)
- 可执行文件路径：/usr/libexec/trayplugin-loader
- 打开文件数：27
- 加载动态库：154 个

#### 271356:/usr/bin/fcitx5 (🔴 +27.426816 MB)
- 可执行文件路径：/usr/bin/fcitx5
- 打开文件数：23
- 加载动态库：102 个

#### 271509:/opt/apps/com.cpis/bin/cpis-engine-service (🔴 +19.954688 MB)
- 可执行文件路径：/opt/apps/com.cpis/bin/cpis-engine-service
- 打开文件数：12
- 加载动态库：52 个

#### 271466:/usr/libexec/trayplugin-loader (🔴 +19.530752 MB)
- 可执行文件路径：/usr/libexec/trayplugin-loader
- 打开文件数：22
- 加载动态库：151 个

#### 271468:/usr/libexec/trayplugin-loader (🔴 +18.107392 MB)
- 可执行文件路径：/usr/libexec/trayplugin-loader
- 打开文件数：23
- 加载动态库：137 个

#### 271366:/usr/bin/deepin-service-manager (🔴 +15.536128 MB)
- 可执行文件路径：/usr/bin/deepin-service-manager
- 打开文件数：32
- 加载动态库：168 个

#### 271365:/usr/bin/deepin-service-manager (🔴 +12.071936 MB)
- 可执行文件路径：/usr/bin/deepin-service-manager
- 打开文件数：27
- 加载动态库：159 个

#### 270832:/usr/bin/wireplumber (🔴 +9.582592 MB)
- 可执行文件路径：/usr/bin/wireplumber
- 打开文件数：44
- 加载动态库：86 个

#### 270828:/usr/bin/pipewire (🔴 +8.290304 MB)
- 可执行文件路径：/usr/bin/pipewire
- 打开文件数：69
- 加载动态库：53 个

#### 271496:/opt/apps/com.cpis/bin/cpis-engine-service (🔴 +7.942144 MB)
- 可执行文件路径：/opt/apps/com.cpis/bin/cpis-engine-service
- 打开文件数：29
- 加载动态库：78 个

#### 271318:/usr/bin/deepin-service-manager (🔴 +7.616512 MB)
- 可执行文件路径：/usr/bin/deepin-service-manager
- 打开文件数：15
- 加载动态库：129 个

#### 271226:/usr/bin/dde-application-manager (🔴 +7.53664 MB)
- 可执行文件路径：/usr/bin/dde-application-manager
- 打开文件数：13
- 加载动态库：31 个

#### 271154:/usr/bin/dde-fakewm (🔴 +7.031808 MB)
- 可执行文件路径：/usr/bin/dde-fakewm
- 打开文件数：13
- 加载动态库：132 个

#### 271529:/usr/bin/dde-clipboard-daemon (🔴 +6.192128 MB)
- 可执行文件路径：/usr/bin/dde-clipboard-daemon
- 打开文件数：15
- 加载动态库：127 个

#### 271540:/usr/lib/dde-api-proxy/dbus-proxy/dde-api-dbus-proxy-v1 (🔴 +4.790272 MB)
- 可执行文件路径：/usr/lib/dde-api-proxy/dbus-proxy/dde-api-dbus-proxy-v1
- 打开文件数：34
- 加载动态库：39 个

#### 270827:/usr/libexec/linglong/ll-session-helper (🔴 +3.77856 MB)
- 可执行文件路径：/usr/libexec/linglong/ll-session-helper
- 打开文件数：6
- 加载动态库：75 个

#### 270833:/usr/bin/pipewire (🔴 +3.195904 MB)
- 可执行文件路径：/usr/bin/pipewire
- 打开文件数：26
- 加载动态库：34 个

#### 270843:/usr/bin/dde-session (🔴 +2.680832 MB)
- 可执行文件路径：/usr/bin/dde-session
- 打开文件数：11
- 加载动态库：46 个

#### 271010:/usr/bin/dde-session (🔴 +2.563072 MB)
- 可执行文件路径：/usr/bin/dde-session
- 打开文件数：10
- 加载动态库：46 个

#### 272487:/usr/lib/openssh/sshd-session (🔴 +2.481152 MB)
- 可执行文件路径：/usr/lib/openssh/sshd-session
- 打开文件数：14
- 加载动态库：52 个

#### 270813:/usr/lib/systemd/systemd-executor (🔴 +1.9712 MB)
- 可执行文件路径：/usr/lib/systemd/systemd-executor
- 打开文件数：8
- 加载动态库：45 个

#### 271408:/usr/libexec/gvfsd-trash (🔴 +1.673216 MB)
- 可执行文件路径：/usr/libexec/gvfsd-trash
- 打开文件数：14
- 加载动态库：24 个

#### 270830:/usr/bin/pipewire (🔴 +1.302528 MB)
- 可执行文件路径：/usr/bin/pipewire
- 打开文件数：16
- 加载动态库：21 个

#### 271065:/usr/libexec/at-spi-bus-launcher (🔴 +1.166336 MB)
- 可执行文件路径：/usr/libexec/at-spi-bus-launcher
- 打开文件数：12
- 加载动态库：30 个

#### 271013:/usr/libexec/gcr-ssh-agent (🔴 +1.156096 MB)
- 可执行文件路径：/usr/libexec/gcr-ssh-agent
- 打开文件数：7
- 加载动态库：24 个

#### 271151:/usr/libexec/at-spi2-registryd (🔴 +1.1264 MB)
- 可执行文件路径：/usr/libexec/at-spi2-registryd
- 打开文件数：10
- 加载动态库：31 个

#### 271016:/usr/lib/uos-ste/uos-ste-resourced (🔴 +1.03424 MB)
- 可执行文件路径：/usr/lib/uos-ste/uos-ste-resourced
- 打开文件数：9
- 加载动态库：24 个

#### 271444:/usr/libexec/gvfsd-metadata (🔴 +783.36 KB)
- 可执行文件路径：/usr/libexec/gvfsd-metadata
- 打开文件数：9
- 加载动态库：17 个

#### 271298:/usr/libexec/geoclue-2.0/demos/agent (🔴 +726.016 KB)
- 可执行文件路径：/usr/libexec/geoclue-2.0/demos/agent
- 打开文件数：8
- 加载动态库：17 个

### 移除进程

#### 710436:/usr/bin/pulseaudio (🟢 -4.129792 MB)
- 可执行文件路径：/usr/bin/pulseaudio
- 打开文件数：22
- 加载动态库：68 个

#### 710424:/usr/lib/systemd/systemd (🟢 -3.9936 MB)
- 可执行文件路径：/usr/lib/systemd/systemd
- 打开文件数：8
- 加载动态库：48 个

#### 710444:/usr/lib/gvfs/gvfsd (🟢 -1.273856 MB)
- 可执行文件路径：/usr/lib/gvfs/gvfsd
- 打开文件数：10
- 加载动态库：24 个

#### 710437:/usr/sbin/sshd (🟢 -1.170432 MB)
- 可执行文件路径：/usr/sbin/sshd
- 打开文件数：14
- 加载动态库：52 个

#### 710439:/usr/bin/dbus-daemon (🟢 -660.48 KB)
- 可执行文件路径：/usr/bin/dbus-daemon
- 打开文件数：14
- 加载动态库：19 个

#### 716731:/usr/bin/bash (🟢 -550.912 KB)
- 可执行文件路径：/usr/bin/bash
- 打开文件数：4
- 加载动态库：4 个

### 变化进程

#### 713464:/usr/bin/dde-lock -> 271228:/usr/bin/dde-lock (🟢 -42.912768 MB)
- 可执行文件路径：/usr/bin/dde-lock
- 打开文件数：24
- 加载动态库：179 个

#### 713565:/usr/lib/deepin-daemon/dde-session-daemon -> 271036:/usr/lib/deepin-daemon/dde-session-daemon (🟢 -26.430464 MB)
- 可执行文件路径：/usr/lib/deepin-daemon/dde-session-daemon
- 打开文件数：67
- 加载动态库：91 个

#### 6192:/usr/bin/dde-file-manager-daemon -> 271216:/usr/bin/dde-file-manager-daemon (🟢 -13.47072 MB)
- 可执行文件路径：/usr/bin/dde-file-manager-daemon
- 打开文件数：28
- 加载动态库：178 个

#### 713527:/usr/bin/dde-clipboard -> 271215:/usr/bin/dde-clipboard (🔴 6.844416 MB)
- 可执行文件路径：/usr/bin/dde-clipboard
- 打开文件数：18
- 加载动态库：143 个

#### 713470:/usr/lib/deepin-deepinid-daemon/deepin-deepinid-daemon -> 271587:/usr/lib/deepin-deepinid-daemon/deepin-deepinid-daemon (🔴 2.948096 MB)
- 可执行文件路径：/usr/lib/deepin-deepinid-daemon/deepin-deepinid-daemon
- 打开文件数：15
- 加载动态库：3 个

#### 713415:/usr/lib/polkit-1-dde/dde-polkit-agent -> 271219:/usr/lib/polkit-1-dde/dde-polkit-agent (🔴 2.294784 MB)
- 可执行文件路径：/usr/lib/polkit-1-dde/dde-polkit-agent
- 打开文件数：20
- 加载动态库：145 个

#### 710459:/usr/bin/bash -> 272502:/usr/bin/bash (🟢 -1.747968 MB)
- 可执行文件路径：/usr/bin/bash
- 打开文件数：4
- 加载动态库：3 个

#### 712855:/usr/bin/kwin_x11 -> 271132:/usr/bin/kwin_x11 (🔴 1.026048 MB)
- 可执行文件路径：/usr/bin/kwin_x11
- 打开文件数：36
- 加载动态库：254 个

#### 713220:/usr/lib/dconf/dconf-service -> 271088:/usr/libexec/dconf-service (🟢 -880.64 KB)
- 可执行文件路径：/usr/libexec/dconf-service
- 打开文件数：8
- 加载动态库：13 个

#### 712746:/usr/bin/dbus-daemon -> 271080:/usr/bin/dbus-daemon (🟢 -837.632 KB)
- 可执行文件路径：/usr/bin/dbus-daemon
- 打开文件数：27
- 加载动态库：19 个

#### 716734:/usr/bin/sudo -> 272505:/usr/bin/sudo (🔴 729.088 KB)
- 可执行文件路径：/usr/bin/sudo
- 打开文件数：14
- 加载动态库：41 个

#### 713792:/usr/lib/bluetooth/obexd -> 271067:/usr/libexec/bluetooth/obexd (🟢 -723.968 KB)
- 可执行文件路径：/usr/libexec/bluetooth/obexd
- 打开文件数：9
- 加载动态库：20 个

#### 713389:/usr/bin/gnome-keyring-daemon -> 270835:/usr/bin/gnome-keyring-daemon (🔴 596.992 KB)
- 可执行文件路径：/usr/bin/gnome-keyring-daemon
- 打开文件数：13
- 加载动态库：23 个

#### 713327:/usr/lib/gvfs/gvfs-udisks2-volume-monitor -> 271326:/usr/libexec/gvfs-udisks2-volume-monitor (🟢 -459.776 KB)
- 可执行文件路径：/usr/libexec/gvfs-udisks2-volume-monitor
- 打开文件数：15
- 加载动态库：41 个

#### 712651:/usr/bin/sudo -> 272507:/usr/bin/sudo (🟢 -335.872 KB)
- 可执行文件路径：/usr/bin/sudo
- 打开文件数：13
- 加载动态库：41 个

#### 4228:/usr/bin/dbus-daemon -> 270842:/usr/bin/dbus-daemon (🔴 287.744 KB)
- 可执行文件路径：/usr/bin/dbus-daemon
- 打开文件数：133
- 加载动态库：19 个

#### 1:/usr/lib/systemd/systemd -> 270812:/usr/lib/systemd/systemd (🟢 -234.496 KB)
- 可执行文件路径：/usr/lib/systemd/systemd
- 打开文件数：111
- 加载动态库：24 个

#### 713357:/usr/lib/gvfs/gvfs-afc-volume-monitor -> 271383:/usr/libexec/gvfs-afc-volume-monitor (🟢 -119.808 KB)
- 可执行文件路径：/usr/libexec/gvfs-afc-volume-monitor
- 打开文件数：9
- 加载动态库：19 个

#### 713339:/usr/lib/gvfs/gvfs-gphoto2-volume-monitor -> 271392:/usr/libexec/gvfs-gphoto2-volume-monitor (🟢 -68.608 KB)
- 可执行文件路径：/usr/libexec/gvfs-gphoto2-volume-monitor
- 打开文件数：9
- 加载动态库：20 个

#### 710449:/usr/lib/gvfs/gvfsd-fuse -> 270882:/usr/libexec/gvfsd-fuse (🟢 -53.248 KB)
- 可执行文件路径：/usr/libexec/gvfsd-fuse
- 打开文件数：11
- 加载动态库：18 个

#### 713344:/usr/lib/gvfs/gvfs-goa-volume-monitor -> 271358:/usr/libexec/gvfs-goa-volume-monitor (🟢 -43.008 KB)
- 可执行文件路径：/usr/libexec/gvfs-goa-volume-monitor
- 打开文件数：8
- 加载动态库：14 个

#### 713350:/usr/lib/gvfs/gvfs-mtp-volume-monitor -> 271367:/usr/libexec/gvfs-mtp-volume-monitor (🔴 8.192 KB)
- 可执行文件路径：/usr/libexec/gvfs-mtp-volume-monitor
- 打开文件数：9
- 加载动态库：16 个

#### 712761:/usr/lib/gvfs/gvfsd -> 270872:/usr/libexec/gvfsd (🟢 -6.144 KB)
- 可执行文件路径：/usr/libexec/gvfsd
- 打开文件数：11
- 加载动态库：21 个

### 进程详情

#### 5729:krfcommd -> 1744:krfcommd
- 内存使用：0 B -> 0 B (变化：0 B)

#### 127:watchdogd -> 86:watchdogd
- 内存使用：0 B -> 0 B (变化：0 B)

#### 716723:/usr/sbin/sshd -> 766:/usr/sbin/sshd
- 内存使用：1.363968 MB -> 2.036736 MB (变化：672.768 KB)
- 可执行文件大小变化：-218.552 KB
- 打开文件数变化：-3
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/liblz4.so.1.8.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libsystemd.so.0.33.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_deepin_pw_check.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_gnome_keyring.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_limits.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libz.so.1.2.11
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_mail.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpam.so.0.84.2
  - 🟢移除库 /usr/lib/libdeepin_pw_check.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_loginuid.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libutil-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcrack.so.2.9.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libm-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_selinux.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_env.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libiniparser.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_nologin.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcom_err.so.2.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_unix.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpam_misc.so.0.82.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkeyutils.so.1.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libwrap.so.0.7.6
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_permit.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_deepin_authentication.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_deny.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_motd.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgpg-error.so.0.26.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-c.so.3.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/liblzma.so.5.2.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_systemd.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/security/pam_keyinit.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcrypt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcrypt.so.20.2.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 22:migration/1 -> 24:migration/1
- 内存使用：0 B -> 0 B (变化：0 B)

#### 682361:/usr/lib/xorg/Xorg -> 270516:/usr/lib/xorg/Xorg
- 内存使用：37.97504 MB -> 70.406144 MB (变化：32.431104 MB)
- 可执行文件大小变化：242.624 KB
- 打开文件数变化：+58
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunwind.so.8.0.1：12.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGL.so.1.7.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 库大小变化 /usr/lib/xorg/modules/input/libinput_drv.so：8.416 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libOpenGL.so.0.0.0
  - 库大小变化 /usr/lib/xorg/modules/extensions/libglx.so：12.288 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsensors.so.5.0.0：-4.112 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgbm.so.1.0.0：-33.36 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmtdev.so.1.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0：-16.456 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libinput.so.10.13.0：73.808 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/xorg/modules/libglamoregl.so：4.224 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libEGL.so.1.1.0：4.096 KB
  - 🟢移除库 /usr/lib/xorg/modules/libfb.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libepoxy.so.0.0.0：-11.952 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libwacom.so.2.6.1：12.216 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/dri/iris_dri.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdrm_nouveau.so.2.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLdispatch.so.0.0.0：8.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgio-2.0.so.0.5800.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgmodule-2.0.so.0.5800.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/xorg/modules/drivers/modesetting_drv.so：8.064 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libicui18n.so.70.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libvulkan.so.1.2.162
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfont2.so.2.0.0：-4.16 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libEGL_mesa.so.0.0.0：110.976 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xfixes.so.0.0.0：-32 B
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gbm/dri_gbm.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcvt.so.0.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgallium-24.3.0-1deepin1.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/dri/libdril_dri.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdrm_intel.so.1.123.0

#### 1:/usr/lib/systemd/systemd -> 270812:/usr/lib/systemd/systemd
- 内存使用：5.02784 MB -> 4.793344 MB (变化：-234.496 KB)
- 可执行文件大小变化：-1.74908 MB
- 打开文件数变化：+4
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libip4tc.so.0.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/systemd/libsystemd-core-255.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 712711:/usr/bin/dbus-daemon -> 607:/usr/bin/dbus-daemon
- 内存使用：534.528 KB -> 4.709376 MB (变化：4.174848 MB)
- 可执行文件大小变化：7.848 KB
- 打开文件数变化：+89
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5

#### 11:rcu_tasks_kthread -> 13:rcu_tasks_kthread
- 内存使用：0 B -> 0 B (变化：0 B)

#### 116:kcompactd0 -> 72:kcompactd0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 4992:/usr/lib/upower/upowerd -> 820:/usr/libexec/upowerd
- 内存使用：2.13504 MB -> 3.938304 MB (变化：1.803264 MB)
- 可执行文件大小变化：-94.064 KB
- 打开文件数变化：+1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcrypt.so.20.2.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libusb-1.0.so.0.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libtasn1.so.6.5.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libidn2.so.0.3.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgnutls.so.30.23.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libhogweed.so.4.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgpg-error.so.0.26.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgmp.so.10.3.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnettle.so.6.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssl.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libzstd.so.1.5.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6

#### 39:idle_inject/4 -> 41:idle_inject/4
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713415:/usr/lib/polkit-1-dde/dde-polkit-agent -> 271219:/usr/lib/polkit-1-dde/dde-polkit-agent
- 内存使用：7.399424 MB -> 9.694208 MB (变化：2.294784 MB)
- 可执行文件大小变化：396.256 KB
- 打开文件数变化：+1
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xkb.so.1.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-keysyms.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdframeworkdbus.so.2.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Concurrent.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5XdgIconLoader.so.3.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libatk-1.0.so.0.23009.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgomp.so.1.0.0：131.592 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-icccm.so.4.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-damage.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk-x11-2.0.so.0.2400.32
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-util.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0：4.176 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libSM.so.6.0.1：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5X11Extras.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render-util.so.0.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 🟢移除库 /usr/lib/polkit-1-dde/plugins/libdpa-ext-gnomekeyring.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-c.so.3.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libmtdev.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xfixes.so.0.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-image.so.0.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinerama.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Xdg.so.3.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libICE.so.6.3.0：-4.232 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLX.so.0.0.0：4.08 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpolkit-gobject-1.so.0.0.0：-2.04 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmd4c.so.0.4.8：-4.096 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon-x11.so.0.0.0：-8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-composite.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-qt5-gui-1.so.1.112.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpolkit-agent-1.so.0.0.0：-1.664 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLdispatch.so.0.0.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libGL.so.1.7.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinput.so.0.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libEGL.so.1.1.0：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shape.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgtk-x11-2.0.so.0.2400.32
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Svg.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXinerama.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libsecret-1.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0.1.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgsettings-qt.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0.4.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6WaylandClient.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlWorkerScript.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy.so.0.5.9
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdde-shell.so.1.99.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libOpenGL.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libduktape.so.207
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnettle.so.8.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-cursor.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-cursor.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libb2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcurl-gnutls.so.4.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/librtmp.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libspdlog.so.1.12.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy/libpxbackend-1.0.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6Quick.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfmt.so.10.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libhogweed.so.6.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlMeta.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2_crypto_gnutls.so.8.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdtk6log.so.0.0.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/polkit-1-dde/plugins/libdpa-deepin-keyring-whitebox.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssh2.so.1.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnghttp3.so.9.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgmp.so.10.5.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-ewmh.so.2.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2.so.16.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libXtst.so.6.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6Qml.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libpsl.so.5.3.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libFcitx5Qt6DBusAddons.so.5.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgnutls.so.30.34.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libidn2.so.0.3.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlModels.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-client.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtasn1.so.6.6.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblber-2.5.so.0.1.8

#### 6626:/usr/sbin/smbd -> 881:/usr/sbin/smbd
- 内存使用：7.088128 MB -> 6.158336 MB (变化：-929.792 KB)
- 可执行文件大小变化：-28.456 KB
- 打开文件数变化：-4
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-spoolss.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libavahi-client.so.3.2.9：3.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libprinting-migrate.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcups.so.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-conn.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdcerpc-binding.so.0.0.1：20.64 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnetapi.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-base.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libads.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-base-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libREG-FULL-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libevents-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-spoolss-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtirpc.so.3.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0

#### 6438:/usr/lib/deepin-authenticate/deepin-authentication -> 1489:/usr/lib/deepin-authenticate/deepin-authentication
- 内存使用：24.225792 MB -> 17.107968 MB (变化：-7.117824 MB)
- 可执行文件大小变化：-703.624 KB
- 打开文件数变化：-8
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0：-8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libepoxy.so.0.0.0：-11.952 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libjpeg.so.62.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libzstd.so.1.5.6

#### 4220:/usr/lib/accountsservice/accounts-daemon -> 601:/usr/libexec/accounts-daemon
- 内存使用：2.81088 MB -> 1.314816 MB (变化：-1.496064 MB)
- 可执行文件大小变化：33.112 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libproxy.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libexpat.so.1.6.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.9.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlidec.so.1.0.7
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.25
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libatk-1.0.so.0.23009.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-glib-1.0.so.0.400.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfreetype.so.6.18.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgtk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXinerama.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libharfbuzz.so.0.20301.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlicommon.so.1.0.7
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpolkit-gobject-1.so.0.0.0：-2.04 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-c.so.3.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpng16.so.16.36.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 4228:/usr/bin/dbus-daemon -> 270842:/usr/bin/dbus-daemon
- 内存使用：2.745344 MB -> 3.033088 MB (变化：287.744 KB)
- 可执行文件大小变化：7.848 KB
- 打开文件数变化：+80
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7

#### 58:migration/7 -> 60:migration/7
- 内存使用：0 B -> 0 B (变化：0 B)

#### 21:idle_inject/1 -> 23:idle_inject/1
- 内存使用：0 B -> 0 B (变化：0 B)

#### 8:kworker/0:1H-events_highpri -> 166:kworker/0:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 112:kauditd -> 66:kauditd
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713350:/usr/lib/gvfs/gvfs-mtp-volume-monitor -> 271367:/usr/libexec/gvfs-mtp-volume-monitor
- 内存使用：899.072 KB -> 907.264 KB (变化：8.192 KB)
- 可执行文件大小变化：16.632 KB
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6

#### 4229:/usr/sbin/NetworkManager -> 656:/usr/sbin/NetworkManager
- 内存使用：9.58976 MB -> 8.799232 MB (变化：-790.528 KB)
- 可执行文件大小变化：958.768 KB
- 打开文件数变化：+6
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libssh2.so.1.0.1：106.68 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_dns-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.9.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_myhostname.so.2
  - 库大小变化 /usr/lib/x86_64-linux-gnu/librtmp.so.1：-4.12 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libwbclient.so.0.14
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libproxy.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libteamdctl.so.0.1.5：8.176 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_wins.so.2
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.25
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbrotlidec.so.1.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2_crypto_gnutls.so.8.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbrotlicommon.so.1.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2.so.16.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3

#### 713527:/usr/bin/dde-clipboard -> 271215:/usr/bin/dde-clipboard
- 内存使用：7.463936 MB -> 14.308352 MB (变化：6.844416 MB)
- 可执行文件大小变化：258.944 KB
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libICE.so.6.3.0：-4.232 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-composite.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgsettings-qt.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0：4.176 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libSM.so.6.0.1：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0.1.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgomp.so.1.0.0：131.592 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5XdgIconLoader.so.3.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLdispatch.so.0.0.0：8.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5X11Extras.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libglibmm-2.4.so.1.3.0：53.48 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xkb.so.1.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon-x11.so.0.0.0：-8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xfixes.so.0.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Xdg.so.3.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsigc-2.0.so.0.0.0：72 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Svg.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shape.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-util.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render-util.so.0.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-damage.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libEGL.so.1.1.0：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libGL.so.1.7.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmd4c.so.0.4.8：-4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Concurrent.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgiomm-2.4.so.1.3.0：209.056 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-image.so.0.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-keysyms.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinerama.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libmtdev.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdframeworkdbus.so.2.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLX.so.0.0.0：4.08 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinput.so.0.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-icccm.so.4.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssh2.so.1.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblber-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfmt.so.10.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-cursor.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6Quick.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlWorkerScript.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libFcitx5Qt6DBusAddons.so.5.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlMeta.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy.so.0.5.9
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgnutls.so.30.34.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libspdlog.so.1.12.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libb2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/librtmp.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libduktape.so.207
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6WaylandClient.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtasn1.so.6.6.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdde-shell.so.1.99.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2.so.16.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnettle.so.8.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy/libpxbackend-1.0.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-ewmh.so.2.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2_crypto_gnutls.so.8.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgmp.so.10.5.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-cursor.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6Qml.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0.4.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libOpenGL.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libhogweed.so.6.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlModels.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-client.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libpsl.so.5.3.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libXtst.so.6.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnghttp3.so.9.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcurl-gnutls.so.4.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdtk6log.so.0.0.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libidn2.so.0.3.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0

#### 113:khungtaskd -> 68:khungtaskd
- 内存使用：0 B -> 0 B (变化：0 B)

#### 312:kworker/4:1H-events_highpri -> 88:kworker/4:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 131:ecryptfs-kthread -> 90:ecryptfs-kthread
- 内存使用：0 B -> 0 B (变化：0 B)

#### 17:idle_inject/0 -> 20:idle_inject/0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 15:rcu_preempt -> 17:rcu_preempt
- 内存使用：0 B -> 0 B (变化：0 B)

#### 38:cpuhp/4 -> 40:cpuhp/4
- 内存使用：0 B -> 0 B (变化：0 B)

#### 7:kworker/0:0H-events_highpri -> 9:kworker/0:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 4237:/usr/sbin/smartd -> 618:/usr/sbin/smartd
- 内存使用：1.48992 MB -> 1.83296 MB (变化：343.04 KB)
- 可执行文件大小变化：172.888 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 712651:/usr/bin/sudo -> 272507:/usr/bin/sudo
- 内存使用：1.3312 MB -> 995.328 KB (变化：-335.872 KB)
- 可执行文件大小变化：149.296 KB
- 打开文件数变化：+5
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_unix.so：-8.968 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_deny.so：72 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_permit.so：40 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_systemd.so：49.408 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_deepin_authentication.so：-4.216 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_deepin_pw_check.so：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libutil-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_dns-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpam_misc.so.0.82.1：-144 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnss_myhostname.so.2：12.368 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/libdeepin_pw_check.so.1.1：4.096 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_limits.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_mdns4_minimal.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libapparmor.so.1.8.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_umask.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libz.so.1.3.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2

#### 6522:/usr/sbin/nmbd -> 864:/usr/sbin/nmbd
- 内存使用：17.78688 MB -> 4.130816 MB (变化：-13.656064 MB)
- 可执行文件大小变化：4.336 KB
- 打开文件数变化：-1
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkeyutils.so.1.10
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0

#### 4243:/usr/sbin/avahi-daemon -> 616:/usr/sbin/avahi-daemon
- 内存使用：362.496 KB -> 375.808 KB (变化：13.312 KB)
- 可执行文件大小变化：22.616 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdaemon.so.0.5.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7

#### 713464:/usr/bin/dde-lock -> 271228:/usr/bin/dde-lock
- 内存使用：82.67776 MB -> 39.764992 MB (变化：-42.912768 MB)
- 可执行文件大小变化：1.007224 MB
- 打开文件数变化：-8
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgomp.so.1.0.0：131.592 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/iconengines/libdtkbuiltin.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmng.so.1.1.0.10：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libGLX_mesa.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinput.so.0.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/dri/iris_dri.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-composite.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinerama.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdrm_nouveau.so.2.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/bearer/libqconnmanbearer.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-util.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xfixes.so.0.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/bearer/libqgenericbearer.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXtst.so.6.1.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdrm_amdgpu.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Concurrent.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdframeworkdbus.so.2.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/iconengines/libdsvgicon.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcroco-0.6.so.3.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5XdgIconLoader.so.3.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/imageformats/libxraw.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLX.so.0.0.0：4.08 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-keysyms.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0：4.176 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xkb.so.1.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Xdg.so.3.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5WaylandClient.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libEGL.so.1.1.0：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0.1.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLdispatch.so.0.0.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libGL.so.1.7.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/xcbglintegrations/libqxcb-glx-integration.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shape.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnm.so.0.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-damage.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libICE.so.6.3.0：-4.232 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_dns-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libelf-0.183.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/bearer/libqnmbearer.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libvulkan.so.1.2.162
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon-x11.so.0.0.0：-8 B
  - 🟢移除库 /usr/lib/dde-session-shell/modules/libone-key-login.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libSM.so.6.0.1：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-dri2.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-glx.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libLLVM-13.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-ewmh.so.2.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-icccm.so.4.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5X11Extras.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libtinfo.so.6.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdrm_radeon.so.1.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXxf86vm.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libraw.so.19.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgsettings-qt.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libsensors.so.5.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render-util.so.0.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-image.so.0.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmd4c.so.0.4.8：-4.096 KB
  - 库大小变化 /usr/lib/dde-session-shell/modules/libdss-network-plugin.so：1.097416 MB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libmtdev.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libedit.so.2.0.59
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdrm.so.2.4.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-cursor.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssh2.so.1.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6WaylandClient.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsharpyuv.so.0.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcairo-gobject.so.2.11800.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnghttp3.so.9.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/iconengines/libqsvgicon.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/librtmp.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcurl-gnutls.so.4.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libb2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdeflate.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2.so.16.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfmt.so.10.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy.so.0.5.9
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblber-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-cursor.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdtk6log.so.0.0.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2_crypto_gnutls.so.8.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libFcitx5Qt6DBusAddons.so.5.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libLerc.so.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libpsl.so.5.3.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libspdlog.so.1.12.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy/libpxbackend-1.0.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libOpenGL.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libduktape.so.207
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/imageformats/libdci.so

#### 716735:/tmp/memory-analysis-1-1745493999/memory-analysis -> 272508:/tmp/memory-analysis-2-1745493999/memory-analysis
- 内存使用：6.496256 MB -> 9.842688 MB (变化：3.346432 MB)

#### 4794:/usr/lib/policykit-1/polkitd -> 615:/usr/lib/polkit-1/polkitd
- 内存使用：4.215808 MB -> 3.377152 MB (变化：-838.656 KB)
- 可执行文件大小变化：98.624 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libproxy.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpolkit-gobject-1.so.0.0.0：-2.04 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXinerama.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlidec.so.1.0.7
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-backend-1.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.25
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlicommon.so.1.0.7
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpng16.so.16.36.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libatk-1.0.so.0.23009.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-c.so.3.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libharfbuzz.so.0.20301.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-glib-1.0.so.0.400.4
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfreetype.so.6.18.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgtk-x11-2.0.so.0.2400.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libduktape.so.207
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 4239:/usr/lib/systemd/systemd-logind -> 620:/usr/lib/systemd/systemd-logind
- 内存使用：2.335744 MB -> 2.542592 MB (变化：206.848 KB)
- 可执行文件大小变化：16.544 KB
- 打开文件数变化：+6
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libip4tc.so.0.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 4226:/usr/sbin/cron -> 606:/usr/sbin/cron
- 内存使用：346.112 KB -> 322.56 KB (变化：-23.552 KB)
- 可执行文件大小变化：-4.016 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 682356:/usr/sbin/lightdm -> 270670:/usr/sbin/lightdm
- 内存使用：1.62304 MB -> 2.989056 MB (变化：1.366016 MB)
- 可执行文件大小变化：-12.208 KB
- 打开文件数变化：+2
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libproxy.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.25
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_succeed_if.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_systemd.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_deepin_authentication.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrack.so.2.9.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_deny.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_nologin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_loginuid.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblzma.so.5.4.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblz4.so.1.9.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_deepin_keyring.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsystemd.so.0.38.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libzstd.so.1.5.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libiniparser.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libpam_misc.so.0.82.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_env.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_gnome_keyring.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_selinux.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libjson-c.so.5.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_permit.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_unix.so
  - 🔴新增库 /usr/lib/libdeepin_pw_check.so.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_umask.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypt.so.1.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_deepin_pw_check.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_limits.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 25:kworker/1:0H-events_highpri -> 27:kworker/1:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 44:cpuhp/5 -> 46:cpuhp/5
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713339:/usr/lib/gvfs/gvfs-gphoto2-volume-monitor -> 271392:/usr/libexec/gvfs-gphoto2-volume-monitor
- 内存使用：1.368064 MB -> 1.299456 MB (变化：-68.608 KB)
- 可执行文件大小变化：16.632 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgphoto2.so.6.1.0：160 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgphoto2_port.so.12.0.0：176 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 130:kswapd0 -> 89:kswapd0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 12:rcu_tasks_rude_kthread -> 14:rcu_tasks_rude_kthread
- 内存使用：0 B -> 0 B (变化：0 B)

#### 4916:/usr/lib/deepin-daemon/dde-system-daemon -> 935:/usr/lib/deepin-daemon/dde-system-daemon
- 内存使用：29.356032 MB -> 22.895616 MB (变化：-6.460416 MB)
- 可执行文件大小变化：-1.081936 MB
- 打开文件数变化：-54
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libwacom.so.2.6.1：12.216 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libepoxy.so.0.0.0：-11.952 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmtdev.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0：-8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libinput.so.10.13.0：73.808 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libjpeg.so.62.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5

#### 117:ksmd -> 73:ksmd
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713357:/usr/lib/gvfs/gvfs-afc-volume-monitor -> 271383:/usr/libexec/gvfs-afc-volume-monitor
- 内存使用：1.570816 MB -> 1.451008 MB (变化：-119.808 KB)
- 可执行文件大小变化：16.632 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnettle.so.6.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgnutls.so.30.23.2
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libidn2.so.0.3.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgmp.so.10.3.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcrypt.so.20.2.4
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libtasn1.so.6.5.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgpg-error.so.0.26.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libhogweed.so.4.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libzstd.so.1.5.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssl.so.3

#### 716734:/usr/bin/sudo -> 272505:/usr/bin/sudo
- 内存使用：1.344512 MB -> 2.0736 MB (变化：729.088 KB)
- 可执行文件大小变化：149.296 KB
- 打开文件数变化：+6
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_deepin_authentication.so：-4.216 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_deny.so：72 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_systemd.so：49.408 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_deepin_pw_check.so：-24 B
  - 库大小变化 /usr/lib/libdeepin_pw_check.so.1.1：4.096 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_unix.so：-8.968 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_dns-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/security/pam_permit.so：40 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libutil-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnss_myhostname.so.2：12.368 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpam_misc.so.0.82.1：-144 B
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_limits.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/security/pam_umask.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_mdns4_minimal.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libz.so.1.3.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libapparmor.so.1.8.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 55:kworker/6:0H-events_highpri -> 57:kworker/6:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 40:migration/4 -> 42:migration/4
- 内存使用：0 B -> 0 B (变化：0 B)

#### 43:kworker/4:0H-events_highpri -> 45:kworker/4:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713389:/usr/bin/gnome-keyring-daemon -> 270835:/usr/bin/gnome-keyring-daemon
- 内存使用：2.804736 MB -> 3.401728 MB (变化：596.992 KB)
- 可执行文件大小变化：28.92 KB
- 打开文件数变化：-1
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgck-1.so.0.0.0：-72 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcr-base-3.so.1.0.0：12.688 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libzstd.so.1.5.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsystemd.so.0.38.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblzma.so.5.4.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblz4.so.1.9.3

#### 29:ksoftirqd/2 -> 31:ksoftirqd/2
- 内存使用：0 B -> 0 B (变化：0 B)

#### 163:kworker/5:1H-events_highpri -> 184:kworker/5:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 49:kworker/5:0H-events_highpri -> 51:kworker/5:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 23:ksoftirqd/1 -> 25:ksoftirqd/1
- 内存使用：0 B -> 0 B (变化：0 B)

#### 6103:jbd2/nvme0n1p5-8 -> 276:jbd2/nvme0n1p5-8
- 内存使用：0 B -> 0 B (变化：0 B)

#### 46:migration/5 -> 48:migration/5
- 内存使用：0 B -> 0 B (变化：0 B)

#### 52:migration/6 -> 54:migration/6
- 内存使用：0 B -> 0 B (变化：0 B)

#### 716799:unknown -> 272489:unknown
- 内存使用：0 B -> 0 B (变化：0 B)

#### 20:cpuhp/1 -> 22:cpuhp/1
- 内存使用：0 B -> 0 B (变化：0 B)

#### 5142:/usr/bin/dde-dconfig-daemon -> 718:/usr/bin/dde-dconfig-daemon
- 内存使用：2.495488 MB -> 6.660096 MB (变化：4.164608 MB)
- 可执行文件大小变化：26.168 KB
- 打开文件数变化：+1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgsettings-qt.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libffi.so.6.0.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgmodule-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libselinux.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgobject-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgio-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgomp.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfmt.so.10.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libspdlog.so.1.12.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdtk6log.so.0.0.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libb2.so.1.0.4

#### 6635:/usr/sbin/winbindd -> 871:/usr/sbin/winbindd
- 内存使用：1.480704 MB -> 3.101696 MB (变化：1.620992 MB)
- 可执行文件大小变化：-368.432 KB
- 打开文件数变化：+1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libidmap.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba4.so.0：37.016 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauth.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libads.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdcerpc-binding.so.0.0.1：20.64 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnss-info.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/samba/libndr-samba4.so.0：577.688 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnss-info-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-pkt-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libads-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkeyutils.so.1.10
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libRPC-SERVER-LOOP-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libevents-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdcerpc-server-core.so.0.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libidmap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2

#### 26:cpuhp/2 -> 28:cpuhp/2
- 内存使用：0 B -> 0 B (变化：0 B)

#### 6297:/usr/sbin/ipwatchd -> 218229:/usr/sbin/ipwatchd
- 内存使用：2.848768 MB -> 2.70848 MB (变化：-140.288 KB)
- 可执行文件大小变化：-8.296 KB
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnet.so.1.7.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 34:migration/3 -> 36:migration/3
- 内存使用：0 B -> 0 B (变化：0 B)

#### 317:card0-crtc2 -> 195:card0-crtc2
- 内存使用：0 B -> 0 B (变化：0 B)

#### 712855:/usr/bin/kwin_x11 -> 271132:/usr/bin/kwin_x11
- 内存使用：81.647616 MB -> 82.673664 MB (变化：1.026048 MB)
- 可执行文件大小变化：1.955984 MB
- 打开文件数变化：+5
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-present.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5SonnetUi.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5KIOWidgets.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libEGL.so.1.1.0：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Sql.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Plasma.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-composite.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xfixes.so.0.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5DBusAddons.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Auth.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libasyncns.so.0.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libOpenGL.so.0.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/org.kde.kwin.platforms/KWinX11Platform.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xinerama.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Activities.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-glx.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Attica.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libSM.so.6.0.1：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/iconengines/libxdgicon.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdrm_nouveau.so.2.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Concurrent.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon-x11.so.0.0.0：-8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpulse.so.0.23.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdbusmenu-qt5.so.2.6.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/kwin/effects/plugins/libblur.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Script.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libglapi.so.0.0.0：-16.456 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shape.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-xinput.so.0.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5XdgIconLoader.so.3.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librsvg-2.so.2.44.10
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmd4c.so.0.4.8：-4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5TextWidgets.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/kwin/effects/plugins/libblack-screen.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Xdg.so.3.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libICE.so.6.3.0：-4.232 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-keysyms.so.1.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkdecorations2private.so.5.14.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkdeinit5_kwin_x11.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXxf86vm.so.1.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-xkb.so.1.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0：4.176 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-damage.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXtst.so.6.1.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcroco-0.6.so.3.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5SonnetCore.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render-util.so.0.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Completion.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgomp.so.1.0.0：131.592 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libepoxy.so.0.0.0：-11.952 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libvorbisenc.so.2.0.11
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0.1.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/kwin/effects/plugins/libscissor-window.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/iconengines/libdsvgicon.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/platforms/libdde-kwin-xcb.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGL.so.1.7.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/imageformats/libqwbmp.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsensors.so.5.0.0：-4.112 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkwinxrenderutils.so.5.15.5
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so：8.344 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5WaylandClient.so.5.54.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmng.so.1.1.0.10：4.096 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/org.kde.kdecoration2/libdeepin-chameleon.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpulse-mainloop-glib.so.0.0.6
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxshmfence.so.1.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Declarative.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5X11Extras.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libinput.so.10.13.0：73.808 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5QuickAddons.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5GlobalAccelPrivate.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/dri/iris_dri.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Sensors.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libwacom.so.2.6.1：12.216 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/iconengines/libdtkbuiltin.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLX_mesa.so.0.0.0：-37.184 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/org.kde.kwin.scenes/KWinSceneOpenGL.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLX.so.0.0.0：4.08 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLdispatch.so.0.0.0：8.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libraw.so.19.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5TextToSpeech.so.5.15.8
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkwin4_effect_builtins.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfam.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libsndfile.so.1.0.28
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libFLAC.so.8.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/qt5/plugins/imageformats/libxraw.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/pulseaudio/libpulsecommon-14.2.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libwrap.so.0.7.6
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkwin-xcb.so.0.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libphonon4qt5.so.4.10.2
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-image.so.0.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libvulkan.so.1.2.162
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libkdecorations2.so.5.14.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5WaylandServer.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-cursor.so.0.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-icccm.so.4.0.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmtdev.so.1.0.0：-24 B
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libpsl.so.5.3.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgallium-24.3.0-1deepin1.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6IconWidgets.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/imageformats/libdci.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgnutls.so.30.34.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/librtmp.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libltdl.so.7.3.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libusbmuxd-2.0.so.6.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgmp.so.10.5.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlWorkerScript.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6KIOGui.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgbm.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnettle.so.8.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libvorbisfile.so.3.3.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkwindecorations2private.so.5.27.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libqaccessibilityclient-qt6.so.0.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/kwin/plugins/kwin5_plugin_buttonrebinds.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/kwin/plugins/kwin5_plugin_nightcolor.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnghttp3.so.9.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6BreezeIcons.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdeflate.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssh2.so.1.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfmt.so.10.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKGlobalAccelD.so.6.2.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libplist-2.0.so.3.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libhogweed.so.6.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6ColorScheme.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsharpyuv.so.0.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy/libpxbackend-1.0.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcanberra.so.0.2.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libFcitx5Qt6DBusAddons.so.5.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/iconengines/libdicon.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdrm_intel.so.1.123.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libimobiledevice-1.0.so.6.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKWaylandClient.so.6.2.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy.so.0.5.9
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtasn1.so.6.6.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/iconengines/libqsvgicon.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-xtest.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6WaylandClient.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libLerc.so.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/org.kde.kdecoration2/com.deepin.chameleon.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdtk6log.so.0.0.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/kwin/plugins/kwin5_plugin_colord.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libpciaccess.so.0.11.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6QmlMeta.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libduktape.so.207
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0.4.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2_crypto_gnutls.so.8.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtdb.so.1.4.10
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libssl.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libspdlog.so.1.12.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6AuthCore.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-res.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6Svg.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkwindecorations2.so.5.27.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libKF6Solid.so.6.6.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcurl-gnutls.so.4.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcvt.so.0.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/liblber-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libb2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2.so.16.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libidn2.so.0.3.7

#### 712761:/usr/lib/gvfs/gvfsd -> 270872:/usr/libexec/gvfsd
- 内存使用：1.312768 MB -> 1.306624 MB (变化：-6.144 KB)
- 可执行文件大小变化：312 B
- 打开文件数变化：+1
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsecret-1.so.0.0.0：53.248 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcr-base-3.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfsdaemon.so：-12.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcr-4.so.4.2.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6

#### 713565:/usr/lib/deepin-daemon/dde-session-daemon -> 271036:/usr/lib/deepin-daemon/dde-session-daemon
- 内存使用：66.004992 MB -> 39.574528 MB (变化：-26.430464 MB)
- 可执行文件大小变化：-8.619696 MB
- 打开文件数变化：-7
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libepoxy.so.0.0.0：-11.952 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0：-8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libSM.so.6.0.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libatk-bridge-2.0.so.0.0.0：32.864 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXtst.so.6.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnl-genl-3.so.200.26.0：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnl-3.so.200.26.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libFLAC.so.8.3.0：-172.032 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libasyncns.so.0.3.1：-4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libwrap.so.0.7.6
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libICE.so.6.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0：-4.128 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libatspi.so.0.0.1：20.56 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0：24.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libasound.so.2.0.0：82.216 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf_xlib-2.0.so.0.3800.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libopus.so.0.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libjpeg.so.62.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcloudproviders.so.0.3.6

#### 4235:/usr/lib/bluetooth/bluetoothd -> 605:/usr/libexec/bluetooth/bluetoothd
- 内存使用：2.260992 MB -> 2.583552 MB (变化：322.56 KB)
- 可执行文件大小变化：388.288 KB
- 打开文件数变化：+5
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libasound.so.2.0.0：82.216 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 6637:/usr/sbin/smbd -> 894:/usr/sbin/smbd
- 内存使用：1.562624 MB -> 1.403904 MB (变化：-158.72 KB)
- 可执行文件大小变化：-28.456 KB
- 打开文件数变化：-9
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcups.so.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-conn.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-base.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnetapi.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libprinting-migrate.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-spoolss.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libavahi-client.so.3.2.9：3.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libads.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdcerpc-binding.so.0.0.1：20.64 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libREG-FULL-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtirpc.so.3.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-base-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-spoolss-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libevents-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0

#### 53:ksoftirqd/6 -> 55:ksoftirqd/6
- 内存使用：0 B -> 0 B (变化：0 B)

#### 193:kworker/6:1H-events_highpri -> 181:kworker/6:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 4219:/usr/sbin/ModemManager -> 693:/usr/sbin/ModemManager
- 内存使用：5.91872 MB -> 6.358016 MB (变化：439.296 KB)
- 可执行文件大小变化：648.368 KB
- 打开文件数变化：+1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlidec.so.1.0.7
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-option-hso.so：-9.224 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-altair-lte.so：-848 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-nokia-icera.so：-50.544 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-novatel.so：-21.376 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-haier.so：-200 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-ublox.so：19.336 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgtk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-c.so.3.0.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-linktop.so：-4.336 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-huawei.so：-1.864 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-telit.so：-37.88 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-simtech.so：36.472 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlicommon.so.1.0.7
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libharfbuzz.so.0.20301.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libexpat.so.1.6.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfreetype.so.6.18.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-x22x.so：3.64 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-longcheer.so：-392 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXinerama.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libatk-1.0.so.0.23009.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-quectel.so：28.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-ericsson-mbm.so：-5.16 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-sierra-legacy.so：-79.432 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpolkit-gobject-1.so.0.0.0：-2.04 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-motorola.so：-80 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-pantech.so：-264 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-via.so：-312 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-novatel-lte.so：-4.744 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-mtk.so：-4.472 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-glib-1.0.so.0.400.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.9.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-samsung.so：-50.624 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpng16.so.16.36.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-sierra.so：-232 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-generic.so：-224 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-iridium.so：3.656 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-cinterion.so：64.448 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-thuraya.so：-336 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-nokia.so：-4.448 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-fibocom.so：-13.176 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-dell.so：-157.736 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-wavecom.so：-496 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-option.so：-17.136 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-anydata.so：-4.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-zte.so：-42.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-icera.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-foxconn.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-telit.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gconv/UTF-16.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-broadmobi.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-gosuncn.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-tplink.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-fibocom.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-qcom-soc.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-dlink.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-sierra.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-xmm.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-foxconn.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gconv/IBM850.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-option.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libqrtr-glib.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-shared-novatel.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ModemManager/libmm-plugin-intel.so

#### 16:migration/0 -> 19:migration/0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 14:ksoftirqd/0 -> 16:ksoftirqd/0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 61:kworker/7:0H-events_highpri -> 63:kworker/7:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713327:/usr/lib/gvfs/gvfs-udisks2-volume-monitor -> 271326:/usr/libexec/gvfs-udisks2-volume-monitor
- 内存使用：3.234816 MB -> 2.77504 MB (变化：-459.776 KB)
- 可执行文件大小变化：16.624 KB
- 打开文件数变化：+3
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libicui18n.so.70.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libudisks2.so.0.0.0：327.68 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsecret-1.so.0.0.0：53.248 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libudfread.so.0.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbz2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 678531:kworker/7:1-events -> 270361:kworker/7:1-events
- 内存使用：0 B -> 0 B (变化：0 B)

#### 50:cpuhp/6 -> 52:cpuhp/6
- 内存使用：0 B -> 0 B (变化：0 B)

#### 244891:/usr/sbin/cupsd -> 753:/usr/sbin/cupsd
- 内存使用：2.325504 MB -> 2.555904 MB (变化：230.4 KB)
- 可执行文件大小变化：8.28 KB
- 打开文件数变化：-1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcups.so.2：12.288 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libavahi-client.so.3.2.9：3.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpaper.so.1.1.2：-32 B
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 41:ksoftirqd/4 -> 43:ksoftirqd/4
- 内存使用：0 B -> 0 B (变化：0 B)

#### 13:rcu_tasks_trace_kthread -> 15:rcu_tasks_trace_kthread
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713344:/usr/lib/gvfs/gvfs-goa-volume-monitor -> 271358:/usr/libexec/gvfs-goa-volume-monitor
- 内存使用：1.114112 MB -> 1.071104 MB (变化：-43.008 KB)
- 可执行文件大小变化：16.632 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgoa-1.0.so.0.0.0：-12.288 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6

#### 716798:unknown -> 272573:unknown
- 内存使用：0 B -> 0 B (变化：0 B)

#### 57:idle_inject/7 -> 59:idle_inject/7
- 内存使用：0 B -> 0 B (变化：0 B)

#### 56:cpuhp/7 -> 58:cpuhp/7
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713220:/usr/lib/dconf/dconf-service -> 271088:/usr/libexec/dconf-service
- 内存使用：1.591296 MB -> 710.656 KB (变化：-880.64 KB)
- 可执行文件大小变化：80 B
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 6192:/usr/bin/dde-file-manager-daemon -> 271216:/usr/bin/dde-file-manager-daemon
- 内存使用：27.843584 MB -> 14.372864 MB (变化：-13.47072 MB)
- 可执行文件大小变化：-384.392 KB
- 打开文件数变化：+9
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libatk-1.0.so.0.23009.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgiomm-2.4.so.1.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5XdgIconLoader.so.3.3.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmd4c.so.0.4.8：-4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-agent-1.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libGL.so.1.7.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgsettings-qt.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libglibmm-2.4.so.1.3.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-util.so.1.0.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdeepin-anything-server-lib.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnssutil3.so：6.992 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_atomic.so.1.67.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnss3.so：69.08 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libargon2.so.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0：4.176 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0：24.512 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsmime3.so：13.648 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgtk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/libdisomaster.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/librtmp.so.1：-4.12 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsqlite3.so.0.8.6：359.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_chrono.so.1.67.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-c.so.3.0.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmms.so.0.0.2：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_thread.so.1.67.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0：4.064 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-glib-1.0.so.0.400.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcryptsetup.so.12.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnspr4.so：1.528 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdde-file-manager.so.1.8.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-qt5-gui-1.so.1.112.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnl-genl-3.so.200.26.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libplds4.so：-48 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0：-32 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0：8 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/liblucene++-contrib.so.3.0.7
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLdispatch.so.0.0.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsecret-1.so.0.0.0：53.248 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdevmapper.so.1.02.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_iostreams.so.1.67.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5X11Extras.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_system.so.1.67.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0：-16 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libX11-xcb.so.1.0.0：-184 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Svg.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnl-3.so.200.26.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libudisks2-qt5.so.0.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Concurrent.so.5.15.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdfm-extension.so.1.8.2
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgomp.so.1.0.0：131.592 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0：-32 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdframeworkdbus.so.2.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmediainfo.so.0.0.0：1.480544 MB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjemalloc.so.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libudev.so.1.7.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_date_time.so.1.67.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/liblucene++.so.3.0.7
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0：24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libGLX.so.0.0.0：4.08 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-gobject-1.so.0.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libplc4.so：-176 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdocparser.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1：-12.312 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libanything.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2：4.072 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/deepin-anything-server-lib/plugins/handlers/libdde-anythingmonitor.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_regex.so.1.67.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libboost_filesystem.so.1.67.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-qt5-agent-1.so.1.112.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libKF5Codecs.so.5.54.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgio-qt.so.0.0.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpolkit-qt5-core-1.so.1.112.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/deepin-anything-server-lib/plugins/handlers/libupdate-lft.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libssh2.so.1.0.1：106.68 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXinerama.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libQt5Xdg.so.3.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libsigc-2.0.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-xkb.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6WaylandClient.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdfm6-io.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-cursor.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdeflate.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfmt.so.10.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/platforminputcontexts/libfcitx5platforminputcontextplugin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-client.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/dde-file-manager/plugins/filemanager-core/libdfmdaemon-core-plugin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libspdlog.so.1.12.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdfm6-burn.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-xfixes.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxkbcommon.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-randr.so.0.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libICE.so.6.3.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/dde-file-manager/plugins/filemanager-core/libdfmdaemon-tag-plugin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/platforms/libdxcb.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-render-util.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/platformthemes/libqdeepin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libEGL.so.1.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-keysyms.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/qt6/plugins/styles/libchameleon.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxkbcommon-x11.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6OpenGL.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libb2.so.1.0.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-icccm.so.4.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libSM.so.6.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-sync.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/dde-file-manager/plugins/filemanager-core/libdfmdaemon-filemanager1-plugin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libLerc.so.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdtk6log.so.0.0.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdfm6-framework.so.6.5.45
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdfm6-base.so.6.5.45
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-shape.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-damage.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libduktape.so.207
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdfm6-mount.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-composite.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy.so.0.5.9
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/dde-file-manager/plugins/filemanager-core/libdfmdaemon-recent-plugin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2_crypto_gnutls.so.8.1.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libwayland-cursor.so.0.23.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libxcb-image.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libOpenGL.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/dde-file-manager/plugins/filemanager-core/libdfmdaemon-vault-plugin.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libQt6XcbQpa.so.6.8.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libngtcp2.so.16.2.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libudisks2.so.0.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libFcitx5Qt6DBusAddons.so.5.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libproxy/libpxbackend-1.0.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libsharpyuv.so.0.0.1

#### 110:kdevtmpfs -> 64:kdevtmpfs
- 内存使用：0 B -> 0 B (变化：0 B)

#### 45:idle_inject/5 -> 47:idle_inject/5
- 内存使用：0 B -> 0 B (变化：0 B)

#### 713470:/usr/lib/deepin-deepinid-daemon/deepin-deepinid-daemon -> 271587:/usr/lib/deepin-deepinid-daemon/deepin-deepinid-daemon
- 内存使用：15.755264 MB -> 18.70336 MB (变化：2.948096 MB)
- 可执行文件大小变化：910.864 KB
- 打开文件数变化：+6
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_mdns4_minimal.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 4241:/usr/sbin/wpa_supplicant -> 663:/usr/sbin/wpa_supplicant
- 内存使用：4.378624 MB -> 5.925888 MB (变化：1.547264 MB)
- 可执行文件大小变化：508.24 KB
- 打开文件数变化：-1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnl-route-3.so.200.26.0：4.064 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnl-genl-3.so.200.26.0：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnl-3.so.200.26.0：-32 B
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ossl-modules/legacy.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66

#### 457:/usr/bin/udevadm -> 422:/usr/bin/udevadm
- 内存使用：5.322752 MB -> 4.768768 MB (变化：-553.984 KB)
- 可执行文件大小变化：246.2 KB
- 打开文件数变化：+1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libzstd.so.1.5.6

#### 28:migration/2 -> 30:migration/2
- 内存使用：0 B -> 0 B (变化：0 B)

#### 315:card0-crtc0 -> 193:card0-crtc0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 712778:/usr/bin/dash -> 659:/usr/bin/dash
- 内存使用：144.384 KB -> 186.368 KB (变化：41.984 KB)
- 可执行文件大小变化：8.272 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 31:kworker/2:0H-events_highpri -> 33:kworker/2:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 35:ksoftirqd/3 -> 37:ksoftirqd/3
- 内存使用：0 B -> 0 B (变化：0 B)

#### 710423:/usr/lib/systemd/systemd -> 1:/usr/lib/systemd/systemd
- 内存使用：3.288064 MB -> 6.703104 MB (变化：3.41504 MB)
- 可执行文件大小变化：-1.74908 MB
- 打开文件数变化：+203
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libip4tc.so.0.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/systemd/libsystemd-core-255.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 6562:/usr/sbin/winbindd -> 877:/usr/sbin/winbindd
- 内存使用：2.716672 MB -> 4.496384 MB (变化：1.779712 MB)
- 可执行文件大小变化：-368.432 KB
- 打开文件数变化：-2
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba4.so.0：37.016 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnss-info.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libads.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdcerpc-binding.so.0.0.1：20.64 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 库大小变化 /usr/lib/x86_64-linux-gnu/samba/libndr-samba4.so.0：577.688 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauth.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libidmap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libads-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gconv/UTF-16.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/gconv/IBM850.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/gensec/krb5.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnss-info-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libevents-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdcerpc-server-core.so.0.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libidmap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkeyutils.so.1.10
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-pkt-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libRPC-SERVER-LOOP-samba4.so.0

#### 713792:/usr/lib/bluetooth/obexd -> 271067:/usr/libexec/bluetooth/obexd
- 内存使用：2.407424 MB -> 1.683456 MB (变化：-723.968 KB)
- 可执行文件大小变化：139.392 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdb-5.3.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgthread-2.0.so.0.5800.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1：41.008 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libicalss.so.3.0.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 6122:jbd2/nvme0n1p4-8 -> 246:jbd2/nvme0n1p4-8
- 内存使用：0 B -> 0 B (变化：0 B)

#### 413:kworker/3:1H-events_highpri -> 158:kworker/3:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 37:kworker/3:0H-events_highpri -> 39:kworker/3:0H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 2:kthreadd -> 2:kthreadd
- 内存使用：0 B -> 0 B (变化：0 B)

#### 114:oom_reaper -> 69:oom_reaper
- 内存使用：0 B -> 0 B (变化：0 B)

#### 444:/usr/lib/systemd/systemd-journald -> 386:/usr/lib/systemd/systemd-journald
- 内存使用：38.478848 MB -> 49.174528 MB (变化：10.69568 MB)
- 可执行文件大小变化：27.4 KB
- 打开文件数变化：+19
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libip4tc.so.0.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 348:kworker/1:1H-events_highpri -> 183:kworker/1:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 710449:/usr/lib/gvfs/gvfsd-fuse -> 270882:/usr/libexec/gvfsd-fuse
- 内存使用：1.3568 MB -> 1.303552 MB (变化：-53.248 KB)
- 可执行文件大小变化：128 B
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libproxy.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so：8.344 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.25
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 4223:/usr/sbin/avahi-daemon -> 604:/usr/sbin/avahi-daemon
- 内存使用：1.152 MB -> 1.22368 MB (变化：71.68 KB)
- 可执行文件大小变化：22.616 KB
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdaemon.so.0.5.0：-24 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5

#### 716709:kworker/4:0-mm_percpu_wq -> 268940:kworker/4:0-mm_percpu_wq
- 内存使用：0 B -> 0 B (变化：0 B)

#### 309:kworker/7:1H-events_highpri -> 104:kworker/7:1H-events_highpri
- 内存使用：0 B -> 0 B (变化：0 B)

#### 6636:/usr/sbin/winbindd -> 898:/usr/sbin/winbindd
- 内存使用：2.115584 MB -> 1.71008 MB (变化：-405.504 KB)
- 可执行文件大小变化：-368.432 KB
- 打开文件数变化：-2
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdcerpc-binding.so.0.0.1：20.64 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnss-info.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libidmap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/samba/libndr-samba4.so.0：577.688 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba4.so.0：37.016 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libads.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libidmap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libdcerpc-server-core.so.0.0.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-pkt-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libm.so.6
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libevents-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkeyutils.so.1.10
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libRPC-SERVER-LOOP-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnss-info-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libads-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0

#### 316:card0-crtc1 -> 194:card0-crtc1
- 内存使用：0 B -> 0 B (变化：0 B)

#### 712746:/usr/bin/dbus-daemon -> 271080:/usr/bin/dbus-daemon
- 内存使用：1.625088 MB -> 787.456 KB (变化：-837.632 KB)
- 可执行文件大小变化：7.848 KB
- 打开文件数变化：-42
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.11.7
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_winbind.so.2

#### 118:khugepaged -> 75:khugepaged
- 内存使用：0 B -> 0 B (变化：0 B)

#### 4240:/usr/lib/udisks2/udisksd -> 621:/usr/libexec/udisks2/udisksd
- 内存使用：5.743616 MB -> 5.262336 MB (变化：-481.28 KB)
- 可执行文件大小变化：72.936 KB
- 打开文件数变化：+3
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so：8.352 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libparted.so.2.0.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcomposite.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbd_part_err.so.2.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXfixes.so.3.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpixman-1.so.0.36.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfreetype.so.6.18.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnssutil3.so：6.992 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libatk-1.0.so.0.23009.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsmime3.so：13.648 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libudisks2.so.0.0.0：327.68 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdevmapper.so.1.02.1：16.576 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.25
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXau.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libplds4.so：-48 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdmcp.so.6.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libparted-fs-resize.so.0.0.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libplc4.so：-176 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libjson-glib-1.0.so.0.400.4
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgraphite2.so.3.2.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlicommon.so.1.0.7
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgtk-x11-2.0.so.0.2400.32
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libargon2.so.1
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libatasmart.so.4.0.5：-24 B
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so：4.096 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbrotlidec.so.1.0.7
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libproxy.so.1.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libexpat.so.1.6.8
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcairo.so.2.11600.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so.0.3800.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXcursor.so.1.0.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnspr4.so：1.528 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnss3.so：69.08 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libthai.so.0.3.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXi.so.6.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXinerama.so.1.0.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libuuid.so.1.3.0：4.176 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrender.so.1.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdatrie.so.1.3.5
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libpolkit-gobject-1.so.0.0.0：-2.04 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-shm.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpangocairo-1.0.so.0.4200.3
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfontconfig.so.1.12.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXext.so.6.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb-render.so.0.0.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libbsd.so.0.9.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libX11.so.6.3.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libharfbuzz.so.0.20301.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libxcb.so.1.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXrandr.so.2.2.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libXdamage.so.1.1.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libfribidi.so.0.4.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpango-1.0.so.0.4200.3
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpng16.so.16.36.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so：8.344 KB
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libkeyutils.so.1.10
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmpfr.so.6.2.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgmp.so.10.5.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libext2fs.so.2.4
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbytesize.so.1.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbd_nvme.so.3.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcom_err.so.2.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libfdisk.so.1.1.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnvme.so.1.9.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libe2p.so.2.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcap.so.2.66
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libbd_mdraid.so.3.0.0

#### 6633:/usr/sbin/smbd -> 893:/usr/sbin/smbd
- 内存使用：1.143808 MB -> 1.435648 MB (变化：291.84 KB)
- 可执行文件大小变化：-28.456 KB
- 打开文件数变化：+1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-conn.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-hostconfig.so.0.0.1：14.408 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgensec.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-spoolss.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-role.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libasn1util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnetapi.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libavahi-client.so.3.2.9：3.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libprinting-migrate.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libcups.so.2
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2.2：28.936 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-cmdline.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-nbt.so.0.0.1：16.544 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-standard.so.0.0.1：401.568 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnsl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2.10.10
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamba-util.so.0.0.1：-24.4 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsamdb.so.0.0.1：8.496 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaesni-intel.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-base.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5.so.3.3：-25.2 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libndr-krb5pac.so.0.0.1：69.792 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtrusts-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libunistring.so.2.1.0：8.192 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libaddns.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libpopt-samba3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libdcerpc-binding.so.0.0.1：20.64 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3.1：-20.552 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libsasl2.so.2.0.25：-4.16 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libtevent-util.so.0.0.1：152 B
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgse.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libcliauth.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libgenrand.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libads.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0.1：-4.128 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-role-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libasn1util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgensec-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libkrb5samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgenrand-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-netlogon3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-ldap-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libCHARSET3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-setid-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauthkrb5-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libgse-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcluster-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-cldap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libevents-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libflag-mapping-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsecrets3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicuuc.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibcli-lsa3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libiov-buf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libldap-2.5.so.0.1.8
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libtirpc.so.3.0.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicui18n.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-shim-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdcerpc-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libREG-FULL-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libaddns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-spoolss-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-cluster-support-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-security-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-sockets-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbd-base-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcommon-auth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libstdc++.so.6.0.32
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-reg-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmb-transport-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcmdline-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libldbsamba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libstable-sort-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libclidns-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-SEND-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libdbwrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcliauth-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-debug-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsmbldaphelper-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libinterfaces-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libcrypto.so.3
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamdb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsrpc3-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-dgm-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtdb-wrap-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libnpa-tstream-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libutil-tdb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba-modules-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/liblibsmb-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-nbt-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libserver-id-db-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsamba3-util-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtime-basic-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmsghdr-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsocket-blocking-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libtalloc-report-printf-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libcli-smb-common-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libsys-rw-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libndr-samba-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libreplace-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libicudata.so.74.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libMESSAGING-samba4.so.0
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/samba/libmessages-util-samba4.so.0

#### 59:ksoftirqd/7 -> 61:ksoftirqd/7
- 内存使用：0 B -> 0 B (变化：0 B)

#### 27:idle_inject/2 -> 29:idle_inject/2
- 内存使用：0 B -> 0 B (变化：0 B)

#### 19:cpuhp/0 -> 21:cpuhp/0
- 内存使用：0 B -> 0 B (变化：0 B)

#### 1658:/usr/lib/systemd/systemd-timesyncd -> 596:/usr/lib/systemd/systemd-timesyncd
- 内存使用：1.585152 MB -> 1.892352 MB (变化：307.2 KB)
- 可执行文件大小变化：4.256 KB
- 打开文件数变化：+1
- 动态库变化：
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnss_wins.so.2：53.568 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libnss_myhostname.so.2：12.368 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_dns-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libresolv-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libip4tc.so.0.1.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libblkid.so.1.1.0：32.8 KB
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libaudit.so.1.0.0：-12.28 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libwinbind-client.so.0
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libattr.so.1.1.2448
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libpthread-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libmount.so.1.1.0：65.608 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/samba/libreplace.so.0
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libcap-ng.so.0.0.0：3.656 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/librt-2.28.so
  - 库大小变化 /usr/lib/x86_64-linux-gnu/libselinux.so.1：39.536 KB
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libwbclient.so.0.14
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libmd.so.0.0.5
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/libnss_mdns4_minimal.so.2
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 33:idle_inject/3 -> 35:idle_inject/3
- 内存使用：0 B -> 0 B (变化：0 B)

#### 47:ksoftirqd/5 -> 49:ksoftirqd/5
- 内存使用：0 B -> 0 B (变化：0 B)

#### 710459:/usr/bin/bash -> 272502:/usr/bin/bash
- 内存使用：3.00544 MB -> 1.257472 MB (变化：-1.747968 MB)
- 可执行文件大小变化：158.264 KB
- 打开文件数变化：-1
- 动态库变化：
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libdl-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/libnss_files-2.28.so
  - 🟢移除库 /usr/lib/x86_64-linux-gnu/ld-2.28.so
  - 🔴新增库 /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2

#### 32:cpuhp/3 -> 34:cpuhp/3
- 内存使用：0 B -> 0 B (变化：0 B)

#### 51:idle_inject/6 -> 53:idle_inject/6
- 内存使用：0 B -> 0 B (变化：0 B)

