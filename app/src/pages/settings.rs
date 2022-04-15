//! Settings page
use crate::App;
use eframe::egui::Ui;

impl App {
    pub fn render_settings_page(&mut self, ui: &mut Ui) {
        self.render_header(ui, "Settings");

        ui.horizontal(|ui| {
            ui.label("Api Key: ");

            // TODO: find a way to ensure that the key is correct
            ui.text_edit_singleline(&mut self.state.api_key);
        });
    }
}
