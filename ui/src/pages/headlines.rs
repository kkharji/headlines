//! Main Headline page

use crate::colors::RED;
use crate::components::article_card;
use crate::state::State;
use eframe::egui::{ScrollArea, Spinner, Ui};

pub fn render(ui: &mut Ui, state: &mut State) {
    ScrollArea::vertical().show(ui, |ui| match &state.articles(ui.ctx()) {
        Some(Ok(articles)) => {
            articles.iter().take(10).for_each(|a| article_card(ui, a));
        }
        Some(Err(e)) => {
            ui.colored_label(RED, format!("Something wrong happend {e}"));
        }
        None => {
            ui.heading("Loading");
            ui.add(Spinner::new());
        }
    });
}
