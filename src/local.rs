use anyhow::Result;
use std::process::Command;

pub struct LocalExecutor;

impl LocalExecutor {
    pub fn new() -> Self {
        Self
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

    pub fn execute_sudo_command(&self, command: &str) -> Result<(String, String)> {
        let sudo_command = format!("sudo {}", command);
        self.execute_command(&sudo_command)
    }
}
