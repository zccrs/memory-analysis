use anyhow::Result;
use log::info;
use nix::unistd::{Uid, Gid, chown};
use std::path::Path;

/// 从环境变量中获取实际用户ID
pub fn get_real_user() -> Option<(u32, u32)> {
    // 如果是sudo运行，则从SUDO_UID和SUDO_GID获取实际用户信息
    if let (Ok(uid), Ok(gid)) = (std::env::var("SUDO_UID"), std::env::var("SUDO_GID")) {
        if let (Ok(uid), Ok(gid)) = (uid.parse::<u32>(), gid.parse::<u32>()) {
            return Some((uid, gid));
        }
    }

    // 如果不是sudo运行，则使用当前用户信息
    let uid = nix::unistd::getuid().as_raw();
    let gid = nix::unistd::getgid().as_raw();
    Some((uid, gid))
}

/// 修改文件所有者为实际用户
pub fn fix_file_owner<P: AsRef<Path>>(path: P) -> Result<()> {
    if let Some((uid, gid)) = get_real_user() {
        chown(path.as_ref(), Some(Uid::from_raw(uid)), Some(Gid::from_raw(gid)))?;
        info!("已修改文件 {} 的所有者为 UID={}, GID={}", path.as_ref().display(), uid, gid);
    }
    Ok(())
}
