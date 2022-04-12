use crate::mode::Mode;
use eframe::egui::Context;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub mode: Mode,
    pub api_key: String,
}

impl Config {
    pub fn load(&mut self) {
        match confy::load("headlines") {
            Ok(config) => *self = config,
            Err(e) => {
                tracing::error!("Unable to load config from files {e}");
            }
        }
    }

    pub fn store(&self) {
        if let Err(e) = confy::store("headlines", self) {
            tracing::error!("Unable to save configuration {e}!");
        }
    }

    /// Make sure that the given state match egui state.
    pub fn ensure(&mut self, ctx: &Context) {
        // Ensure mode
        let ctx_is_dark = ctx.style().visuals.dark_mode;
        let config_is_dark = self.mode.is_dark();
        if ctx_is_dark != config_is_dark {
            let mode = &self.mode; // why?
            ctx.set_visuals(mode.into());
        }
    }
}
