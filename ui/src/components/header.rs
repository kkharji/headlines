use super::PADDING;
use crate::macros::{Separator, Space, VerticalCentered};
use crate::App;
use eframe::egui::{RichText, Ui};

impl App {
    pub fn render_header(&self, ui: &mut Ui, title: &str) {
        VerticalCentered!(ui, |ui| { ui.label(RichText::new(title).size(33.)) });
        Space!(PADDING, ui);
        Separator!(20., ui);
    }
}
