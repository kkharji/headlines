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
        match confy::load("headlines", None) {
            Ok(config) => {
                *self = config;
                tracing::trace!("Configuration loaded!");
            }
            Err(e) => {
                tracing::error!("[Fail load config from files]: {e}");
            }
        }
    }

    pub fn persist(&self) {
        if let Err(e) = confy::store("headlines", None, self) {
            tracing::error!("[Fail to save configuration]: {:#?}", e);
        } else {
            tracing::trace!("Configuration saved!.");
        }
    }

    pub fn has_api_key(&self) -> bool {
        !self.api_key.is_empty()
    }
}
