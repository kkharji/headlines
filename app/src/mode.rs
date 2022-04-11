use crate::macros::Button;
use eframe::egui::TextStyle::Body;
use eframe::egui::{Ui, Visuals};
use strum::AsRefStr;

#[derive(AsRefStr, Default, Clone)]
pub enum Mode {
    #[default]
    Dark,
    Light,
}

impl Into<Visuals> for Mode {
    fn into(self) -> Visuals {
        match self {
            Self::Dark => Visuals::dark(),
            Self::Light => Visuals::light(),
        }
    }
}

impl Mode {
    /// Update Mode to alternative mode
    pub fn update(&mut self, ui: &Ui) {
        *self = self.alter();
        ui.ctx().set_visuals(self.clone().into());
    }

    /// Get UI button
    pub fn render_button(&mut self, ui: &mut Ui) {
        let alter = self.alter();
        let (icon, hover) = (alter.icon(), alter.hover_text());
        if Button!(icon, Body, ui).on_hover_text(hover).clicked() {
            self.update(ui);
        }
    }

    /// Get mode hover text.
    pub fn hover_text(&self) -> String {
        format!("Toggle {} mode", self.as_ref())
    }

    /// Get Alternative ModeIcon.
    pub fn icon(&self) -> &str {
        match self {
            Self::Light => "🌞",
            Self::Dark => "🌙",
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
