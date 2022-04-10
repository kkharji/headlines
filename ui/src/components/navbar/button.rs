use crate::macros::Button;
use crate::state::{Page, State};
use eframe::egui::TextStyle::Body;
use eframe::egui::{RichText, Ui};
use eframe::epi::Frame;

/// Generic navbar page button
pub fn page(page: Page, state: &mut State, ui: &mut Ui) {
    let mut label = RichText::new(page.as_ref()).heading();
    if page == state.current_page {
        label = label.strong();
    }
    if ui.button(label).clicked() {
        state.set_current_page(page);
    };
}

/// toggle between dark and light theme
pub fn toggle_theme(ui: &mut Ui, state: &mut State) {
    let isdark = state.config.dark_mode;
    let text = if isdark { "🌞" } else { "🌙" };
    if Button!(text: text, style: Body, ui).clicked() {
        state.config.dark_mode = !state.config.dark_mode;
    };
}

/// refresh current page data.
pub fn refresh(ui: &mut Ui, _state: &mut State) {
    if Button!(text: "🔄", style: Body, ui).clicked() {};
}

/// Close application
pub fn close(ui: &mut Ui, frame: &Frame, _state: &mut State) {
    if Button!(text: "❌", style: Body, ui).clicked() {
        if !cfg!(target_arch = "wasm32") {
            frame.quit();
        }
    }
}
