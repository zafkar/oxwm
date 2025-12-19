use super::Block;
use crate::errors::BlockError;
use std::process::Command;
use std::time::{Duration, Instant};

pub struct ShellBlock {
    format: String,
    command: String,
    onclick_command: Option<String>,
    interval: Duration,
    color: u32,
    cached_output: Option<String>,
    last_run: Option<Instant>,
}

impl ShellBlock {
    pub fn new(
        format: &str,
        command: &str,
        onclick_command: Option<&String>,
        interval_secs: u64,
        color: u32,
    ) -> Self {
        Self {
            format: format.to_string(),
            command: command.to_string(),
            onclick_command: onclick_command.cloned(),
            interval: Duration::from_secs(interval_secs),
            color,
            cached_output: None,
            last_run: None,
        }
    }

    fn execute(&mut self) -> Result<String, BlockError> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output()
            .map_err(|e| BlockError::CommandFailed(format!("Failed to execute command: {}", e)))?;

        if !output.status.success() {
            return Err(BlockError::CommandFailed(format!(
                "Command exited with status: {}",
                output.status
            )));
        }

        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let formatted = self.format.replace("{}", &result);

        self.cached_output = Some(formatted.clone());
        self.last_run = Some(Instant::now());

        Ok(formatted)
    }

    fn onclick_execute(&mut self, click_x: i16) -> Result<(), BlockError> {
        if let Some(command) = &self.onclick_command {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command.replace("{click_x}", &click_x.to_string()))
                .output()
                .map_err(|e| {
                    BlockError::CommandFailed(format!("Failed to execute command: {}", e))
                })?;

            if !output.status.success() {
                return Err(BlockError::CommandFailed(format!(
                    "Command exited with status: {}",
                    output.status
                )));
            }

            let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let formatted = self.format.replace("{}", &result);

            self.cached_output = Some(formatted.clone());
            self.last_run = Some(Instant::now());
        }
        Ok(())
    }
}

impl Block for ShellBlock {
    fn content(&mut self) -> Result<String, BlockError> {
        let should_refresh = match self.last_run {
            None => true,
            Some(last) => last.elapsed() >= self.interval,
        };

        if should_refresh {
            return self.execute();
        }

        self.cached_output
            .clone()
            .ok_or_else(|| BlockError::CommandFailed("No cached output".to_string()))
    }

    fn interval(&self) -> Duration {
        self.interval
    }

    fn color(&self) -> u32 {
        self.color
    }

    fn on_click(&mut self, click_x: i16) {
        let _ = self.onclick_execute(click_x);
    }
}
