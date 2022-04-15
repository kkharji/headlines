use crate::fonts::inner_link;
use crate::macros::*;
use crate::pages::Page;
use crate::style::text;
use crate::App;
use eframe::egui::TextStyle::Body;
use eframe::egui::{menu, Context, Layout, RichText, TopBottomPanel, Ui};
use eframe::epi::Frame;

/// FIX: use svg icons. text sizes differs
impl App {
    pub fn render_navbar(&mut self, ctx: &Context, frame: &Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            Space!(5., ui);
            menu::bar(ui, |ui| {
                Layout!(ui, left_to_right, |ui| ui
                    .label(RichText::new("📓").text_style(inner_link())));

                if frame.is_web() {
                    VerticalCentered!(ui, |ui| ui
                        .label(text("Headlines").size(22.).color(self.white())));
                }

                Layout!(ui, right_to_left, |ui| {
                    Space!(20., ui);
                    // self.close_app(ui, frame);
                    self.toggle_settings_window(ui);
                    self.refresh_articles(ui);
                    if self.state.mode.render_button(ui).clicked() {
                        self.configure_styles(ui.ctx(), true);
                    };
                    self.page.render_button(ui, true);
                });
            });
            Space!(5., ui);
        });
    }

    pub fn render_header(&self, ui: &mut Ui, title: &str) {
        VerticalCentered!(ui, |ui| ui.label(text(title).size(35.)));
        Space!(crate::style::padding(), ui);
        Separator!(20., ui);
    }

    /// Refresh current page data.
    pub fn refresh_articles(&mut self, ui: &mut Ui) {
        if Button!("🔄", Body, ui)
            .on_hover_text("Refresh data")
            .clicked()
        {};
    }

    /// Toggle settings window
    pub fn toggle_settings_window(&mut self, ui: &mut Ui) {
        let mut settings = Page::Settings;
        if settings.render_button(ui, false).clicked() {
            self.page = settings;
        }
    }
}
