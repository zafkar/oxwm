use std::{process::Command, time::Duration};

use crate::{bar::blocks::Block, errors::BlockError};

pub struct ButtonBlock {
    text: String,
    color: u32,
    command: String,
}

impl ButtonBlock {
    pub fn new(text: &str, color: u32, command: &str) -> Self {
        Self {
            text: text.to_string(),
            color,
            command: command.to_string(),
        }
    }
}

impl Block for ButtonBlock {
    fn content(&mut self) -> Result<String, BlockError> {
        Ok(self.text.clone())
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(u64::MAX)
    }

    fn color(&self) -> u32 {
        self.color
    }

    fn on_click(&mut self, click_x: i16) {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "({})&",
                &self.command.replace("{click_x}", &click_x.to_string())
            ))
            .spawn();
    }
}
