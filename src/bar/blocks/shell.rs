use super::Block;
use crate::errors::BlockError;
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct ShellBlock {
    format: String,
    command: String,
    onclick_command: Option<String>,
    interval: Duration,
    color: u32,
    value: Option<String>,
    last_time_run: Instant,
}

impl ShellBlock {
    pub fn new(
        format: &str,
        command: &str,
        onclick_command: Option<impl ToString>,
        interval_secs: u64,
        color: u32,
    ) -> Self {
        Self {
            format: format.to_string(),
            command: command.to_string(),
            onclick_command: onclick_command.map(|s| s.to_string()),
            interval: Duration::from_secs(interval_secs),
            color,
            value: None,
            last_time_run: Instant::now(),
        }
    }
}

impl ShellBlock {
    fn run(&mut self, cmd: &str) -> Result<String, BlockError> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| BlockError::CommandFailed(format!("Failed to execute command: {}", e)))?;

        if !output.status.success() {
            return Err(BlockError::CommandFailed(format!(
                "Command exited with status: {}",
                output.status
            )));
        }

        self.last_time_run = Instant::now();

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn run_cmd(&mut self) -> Result<(), BlockError> {
        let cmd = self.command.clone();
        self.value = Some(self.run(&cmd)?);

        Ok(())
    }

    fn run_onclick(&mut self, click_x: i16) -> Result<(), BlockError> {
        if let Some(onclick_command) = &self.onclick_command {
            self.value =
                Some(self.run(&onclick_command.replace("{click_x}", &click_x.to_string()))?);
        }
        Ok(())
    }
}

impl Block for ShellBlock {
    fn content(&mut self) -> Result<String, BlockError> {
        if self.value.is_none() || self.last_time_run + self.interval > Instant::now() {
            self.run_cmd()?;
        }

        //Error here should never happens since value should have been updated or return an error just before
        self.value
            .as_ref()
            .map(|value| self.format.replace("{}", value))
            .ok_or(BlockError::CommandFailed(
                "Previous command failed leaving this block in an invalid state".to_string(),
            ))
    }

    fn interval(&self) -> Duration {
        if self.last_time_run + self.interval > Instant::now() {
            Duration::from_secs(0)
        } else {
            Duration::MAX
        }
    }

    fn color(&self) -> u32 {
        self.color
    }

    fn on_click(&mut self, click_x: i16) {
        let _ = self.run_onclick(click_x);
    }
}
