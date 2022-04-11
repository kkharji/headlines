//! Main Headline page

use crate::macros::*;
use crate::style::{is_dark_mode, BLACK, PADDING, RED, WHITE};
use crate::App;
use eframe::egui::{Hyperlink, RichText, ScrollArea, Spinner, Ui};

impl App {
    pub fn render_headlines_page(&mut self, ui: &mut Ui) {
        self.render_header(ui, "Headlines");

        let is_dark_mode = is_dark_mode(ui);
        let heading_color = if is_dark_mode { WHITE } else { BLACK };
        let read_more = RichText::new("Read More >").size(16.).monospace();

        ScrollArea::vertical().show(ui, |ui| match &self.articles(ui.ctx()) {
            Some(Ok(articles)) => {
                articles.iter().take(10).for_each(|a| {
                    Space!(PADDING, ui);
                    Label!(format!("▶ {}", a.title), heading_color, heading, ui);
                    Space!(PADDING, ui);
                    Label!(&a.description, ui);
                    Space!(PADDING, ui);
                    UiWithLayout!(ui, left_to_right, 24.0, |ui| {
                        ui.add(Hyperlink::from_label_and_url(read_more.clone(), &a.url))
                    });
                    Separator!(ui);
                });
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
