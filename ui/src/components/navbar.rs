use crate::app::App;
use crate::fonts::inner_link;
use crate::macros::*;
use crate::pages::Page::{Headlines, Search};
use eframe::egui::{menu, Context, Layout, RichText, TextStyle, TopBottomPanel, Ui};
use eframe::epi::Frame;

impl App {
    pub fn render_navbar(&mut self, ctx: &Context, frame: &Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            Space!(ui);
            menu::bar(ui, |ui| {
                Layout!(ui, left_to_right, |ui| {
                    // Label!("📓", inner_link(), ui);
                    ui.label(RichText::new("📓").text_style(inner_link()));
                });
                Layout!(ui, left_to_right, |ui| {
                    ui.add_space(5.);
                    // self.set_current_page_button(Headlines, ui);
                    // self.set_current_page_button(Search, ui);
                });
                Layout!(ui, right_to_left, |ui| {
                    self.close_button(ui, frame);
                    self.refresh_button(ui);
                    self.toggle_theme_button(ui);
                });
            });
            Space!(ui);
        });
    }

    /// toggle between dark and light theme
    pub fn toggle_theme_button(&mut self, ui: &mut Ui) {
        let isdark = self.config.dark_mode;
        let text = if isdark { "🌞" } else { "🌙" };
        if Button!(text: text, style: TextStyle::Body, ui).clicked() {
            self.config.dark_mode = !self.config.dark_mode;
        };
    }

    /// refresh current page data.
    pub fn refresh_button(&mut self, ui: &mut Ui) {
        if Button!(text: "🔄", style: TextStyle::Body, ui).clicked() {};
    }

    /// Close application
    pub fn close_button(&mut self, ui: &mut Ui, frame: &Frame) {
        if Button!(text: "❌", style: TextStyle::Body, ui).clicked() {
            if !cfg!(target_arch = "wasm32") {
                frame.quit();
            }
        }
    }
}
