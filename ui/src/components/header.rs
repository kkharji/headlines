use super::PADDING;
use crate::app::App;
use eframe::egui::{Separator, Ui};

impl App {
    pub fn render_header(&self, ui: &mut Ui, title: &str) {
        ui.vertical_centered(|ui| ui.heading(title));
        ui.add_space(PADDING);
        ui.add(Separator::default().spacing(20.));
    }
}
