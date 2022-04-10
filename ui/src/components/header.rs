use super::PADDING;
use eframe::egui::{Separator, Ui};

pub fn header(ui: &mut Ui, title: &str) {
    ui.vertical_centered(|ui| ui.heading(title));
    ui.add_space(PADDING);
    ui.add(Separator::default().spacing(20.));
}
