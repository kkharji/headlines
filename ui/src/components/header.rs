use super::PADDING;
use crate::app::App;
use eframe::egui::{RichText, Separator, Ui};

impl App {
    pub fn render_header(&self, ui: &mut Ui, title: &str) {
        ui.vertical_centered(|ui| ui.label(RichText::new(title).size(33.)));
        ui.add_space(PADDING);
        ui.add(Separator::default().spacing(20.));
    }
}
