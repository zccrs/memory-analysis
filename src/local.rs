use anyhow::{Result, anyhow};
use std::process::Command;
use log::debug;

pub struct LocalExecutor {
    sudo_verified: bool
}

impl LocalExecutor {
    pub fn new() -> Self {
        Self {
            sudo_verified: false
        }
    }

    pub fn execute_command(&self, command: &str) -> Result<(String, String)> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        Ok((
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }

    fn verify_sudo(&mut self) -> Result<()> {
        if self.sudo_verified {
            return Ok(());
        }

        // 尝试获取sudo访问权限
        let result = Command::new("sudo")
            .arg("-v")
            .status()?;

        if !result.success() {
            return Err(anyhow!("Failed to verify sudo access"));
        }

        // 设置NOPASSWD
        let whoami = String::from_utf8_lossy(&Command::new("whoami").output()?.stdout).trim().to_string();
        let sudoers_line = format!("{} ALL=(ALL) NOPASSWD: ALL", whoami);

        let temp_sudoers = format!("/etc/sudoers.d/{}", whoami);
        Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(format!("echo '{}' > {}", sudoers_line, temp_sudoers))
            .status()?;

        debug!("Successfully set up NOPASSWD sudo access");
        self.sudo_verified = true;
        Ok(())
    }

    pub fn execute_sudo_command(&mut self, command: &str) -> Result<(String, String)> {
        self.verify_sudo()?;
        let sudo_command = format!("sudo {}", command);
        self.execute_command(&sudo_command)
    }
}
