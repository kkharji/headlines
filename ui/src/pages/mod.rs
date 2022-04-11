mod headlines;
use crate::app::App;
use eframe::egui::{RichText, TextStyle, Ui};
use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug, PartialEq)]
pub enum Page {
    Search,
    Home,
    Headlines,
}

impl Default for Page {
    fn default() -> Self {
        Self::Headlines
    }
}

impl App {
    /// Render main page
    pub fn render_page(&mut self, ui: &mut Ui) {
        match self.page {
            Page::Headlines => {
                self.render_headlines_page(ui);
            }
            Page::Search => {
                ui.centered_and_justified(|ui| {
                    ui.heading("Someday");
                });
            }
            Page::Home => {}
        }
    }

    /// Set the state's current page.
    pub fn set_current_page_button(&mut self, page: Page, ui: &mut Ui) {
        let text_style = TextStyle::Name("NavPageLink".into());
        let mut label = RichText::new(page.as_ref()).text_style(text_style);

        if page == self.page {
            label = label.strong();
        }

        if ui.button(label).clicked() {
            self.page = page;
        };
    }
}
