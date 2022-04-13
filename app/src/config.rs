use crate::mode::Mode;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub mode: Mode,
    pub api_key: String,
}
impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        &self
    }
}

impl Config {
    pub fn load(&mut self) {
        match confy::load("headlines") {
            Ok(config) => {
                *self = config;
                tracing::trace!("Configuration loaded.");
            }
            Err(e) => {
                tracing::error!("Unable to load config from files {e}");
            }
        }
    }

    pub fn store(&self) {
        if let Err(e) = confy::store("headlines", self) {
            return tracing::error!("Unable to save configuration {e}!");
        }
        tracing::trace!("Configuration saved.")
    }

    pub fn has_api_key(&self) -> bool {
        !self.api_key.is_empty()
    }
}
