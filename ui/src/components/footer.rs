use crate::app::App;
use crate::macros::hyperlink;
use eframe::egui::{Context, Hyperlink, RichText, TopBottomPanel};

impl App {
    pub fn render_footer(&self, ctx: &Context) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("NewsApp®").monospace());

                hyperlink!(ui, "Powered by NewsApi", "https://newsapi.org");
                hyperlink!(ui, "Made with egui", "https://github.com/emilk/egui");

                ui.add_space(10.);
            });
        });
    }
}
