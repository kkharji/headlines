use crate::macros::Button;
use eframe::egui::TextStyle::Body;
use eframe::egui::{Ui, Visuals};

#[derive(Default)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

impl From<&mut Mode> for Visuals {
    fn from(mode: &mut Mode) -> Self {
        match mode {
            Mode::Dark => Self::dark(),
            Mode::Light => Self::light(),
        }
    }
}

impl Mode {
    /// Update Mode to alternative mode
    pub fn update(&mut self, ui: &Ui) {
        *self = self.alter();
        ui.ctx().set_visuals(self.into());
    }

    /// Get UI button
    pub fn render_button(&mut self, ui: &mut Ui) {
        let alter = self.alter();
        let (icon, hover) = alter.assets();
        if Button!(icon, Body, ui).on_hover_text(hover).clicked() {
            self.update(ui);
        }
    }

    /// Get mode hover text.
    #[inline]
    pub fn assets(&self) -> (&'static str, &'static str) {
        match self {
            Self::Dark => ("🌙", "Toggle Dark mode"),
            Self::Light => ("🌞", "Toggle Light Mode"),
        }
    }

    /// Returns `true` if the mode is [`Dark`].
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Returns `true` if the mode is [`Light`].
    pub fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }

    /// Get alternative mode
    pub fn alter(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}
