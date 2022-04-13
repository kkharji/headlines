//! Main Headline page

use crate::macros::*;
use crate::style::padding;
use crate::App;
use eframe::egui::{Hyperlink, RichText, ScrollArea, Spinner, Ui};

impl App {
    pub fn render_headlines_page(&mut self, ui: &mut Ui) {
        self.render_header(ui, "Headlines");

        let heading_color = self.foreground_light();
        let read_more = RichText::new("Read More >").size(16.).monospace();

        ScrollArea::vertical().show(ui, |ui| match &self.articles(ui.ctx()) {
            Some(Ok(articles)) => {
                articles.iter().take(30).for_each(|a| {
                    Space!(padding(), ui);
                    Label!(format!("▶ {}", a.title), heading_color, heading, ui);
                    Space!(padding(), ui);
                    Label!(&a.description, ui);
                    Space!(padding(), ui);
                    UiWithLayout!(ui, left_to_right, 24.0, |ui| {
                        ui.add(Hyperlink::from_label_and_url(read_more.clone(), &a.url))
                    });
                    Separator!(ui);
                });
            }
            Some(Err(error)) => {
                ui.colored_label(self.red(), format!("Something wrong happend {error}"));
            }
            None => {
                ui.heading("Loading");
                ui.add(Spinner::new());
            }
        });
    }
}
