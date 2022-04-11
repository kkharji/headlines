//! Main Headline page

use crate::app::App;
use crate::style::RED;
use eframe::egui::{ScrollArea, Spinner, Ui};

impl App {
    pub fn render_headlines_page(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| match &self.articles(ui.ctx()) {
            Some(Ok(articles)) => {
                articles
                    .iter()
                    .take(10)
                    .for_each(|a| self.render_article_card(ui, a));
            }
            Some(Err(error)) => {
                ui.colored_label(RED, format!("Something wrong happend {error}"));
            }
            None => {
                ui.heading("Loading");
                ui.add(Spinner::new());
            }
        });
    }
}
