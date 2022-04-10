use super::PADDING;
use crate::colors::{CYAN, WHITE};
use crate::macros::*;
use eframe::egui::{Hyperlink, Layout, RichText, Separator, TextStyle, Ui};
use eframe::epaint::Vec2;
use newsapp::Article;

pub fn article_card(ui: &mut Ui, article: &Article) {
    let heading = RichText::new(format!("▶ {}", article.title))
        .heading()
        .strong();
    ui.add_space(PADDING);
    ui.colored_label(WHITE, heading);
    ui.add_space(PADDING);

    let text = RichText::new(&article.description).text_style(TextStyle::Body);

    ui.label(text);
    // TODO: move to style.rs
    ui.style_mut().visuals.hyperlink_color = CYAN;
    ui.add_space(PADDING);

    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), 24.0),
        Layout::left_to_right(),
        |ui| {
            hyperlink!(ui, "Read More >", &article.url);
        },
    );

    ui.add(Separator::default());
}
