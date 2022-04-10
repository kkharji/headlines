//! All Article Page

use crate::colors::RED;
use crate::components::{article_card, header};
use crate::state::State;
use eframe::egui::{ScrollArea, Spinner, Ui};

pub fn all_article(ui: &mut Ui, state: &mut State) {
    header(ui, "All");
    ScrollArea::vertical().show(ui, |ui| {
        if let Some(promise) = &state.articles() {
            if let Some(articles) = promise.ready() {
                articles.iter().take(10).for_each(|a| article_card(ui, a));
                return;
            }
            ui.heading("Loading");
            ui.add(Spinner::new());
            return;
        }

        tracing::error!("Promise isn't set yet!");
        ui.colored_label(RED, "Promise isn't set yet!".to_string());
    });
}
