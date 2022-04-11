use crate::app::App;
use crate::macros::*;
use eframe::egui::Context;

impl App {
    pub fn render_footer(&self, ctx: &Context) {
        TopBottomPanel!(ctx, |ui| {
            VerticalCentered!(ui, |ui| {
                Space!(10.0, ui);
                Label!("NewsApp®", Monospace, ui);
                Hyperlink!("Powered by NewsApi", "https://newsapi.org", ui);
                Hyperlink!("Made with egui", "https://github.com/emilk/egui", ui);
                Space!(10.0, ui);
            });
        });
    }
}
