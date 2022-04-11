use super::PADDING;
use crate::app::App;
use crate::macros::*;
use crate::style::{is_dark_mode, BLACK, WHITE};
use eframe::egui::{Layout, RichText, Separator, Ui};
use eframe::epaint::Vec2;
use newsapp::Article;

impl App {
    pub fn render_article_card(&self, ui: &mut Ui, article: &Article) {
        let is_dark_mode = is_dark_mode(ui);
        let heading_color = if is_dark_mode { WHITE } else { BLACK };
        let heading_text = format!("▶ {}", article.title);

        Space!(ui, PADDING);
        ui.colored_label(
            heading_color,
            RichText::new(heading_text).heading().strong(),
        );
        Space!(ui, PADDING);
        Label!(ui, &article.description);
        Space!(ui, PADDING);
        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), 24.0),
            Layout::left_to_right(),
            |ui| {
                // Hyperlink!(ui, "Read More >", &article.url)
                let text = RichText::new("Read More >")
                    .size(16.)
                    .family(eframe::epaint::FontFamily::Monospace);
                ui.add(eframe::egui::Hyperlink::from_label_and_url(
                    text,
                    &article.url,
                ))
            },
        );

        ui.add(Separator::default());
    }
}
