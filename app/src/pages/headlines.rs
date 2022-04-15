//! Main Headline page

use crate::style::padding;
use crate::App;
use crate::{macros::UiWithLayout, style::text};
use eframe::egui::{RichText, ScrollArea, Spinner, Ui};

impl App {
    pub fn render_headlines_page(&mut self, ui: &mut Ui) {
        let heading_color = self.foreground_light();
        let current_query_key = self.state.current_query_key.to_string();
        let read_more = RichText::new("Read More >").size(16.).monospace();
        let title = |title: &str| text(&format!("▶ {title}")).heading().color(heading_color);

        match self.store.try_get_articles(&current_query_key, false) {
            Some(Ok(articles)) => {
                ScrollArea::vertical().show(ui, |ui| {
                    articles.iter().for_each(|a| {
                        ui.add_space(padding());

                        ui.label(title(&a.title));
                        ui.add_space(padding());

                        ui.label(&a.description);
                        ui.add_space(padding());

                        UiWithLayout!(ui, left_to_right, 24.0, |ui| {
                            ui.hyperlink_to(read_more.clone(), &a.url);
                        });

                        ui.separator();
                    });
                });
            }
            Some(Err(e)) => {
                ui.colored_label(self.red(), format!("Something wrong happend {e}"));
            }
            None => {
                ui.heading("Loading");
                ui.add(Spinner::new());
            }
        };
    }
}
