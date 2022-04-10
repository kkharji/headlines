use crate::route::Route;
use crate::state::State;
use eframe::egui::TextStyle::Body;
use eframe::egui::{menu, Context, Layout, RichText, TopBottomPanel, Ui};
use eframe::epi::Frame;

fn logo(ui: &mut Ui) {
    ui.label(RichText::new("📓 NewsApp®").text_style(Body));
}

fn theme_button(ui: &mut Ui, state: &mut State) {
    let text = if state.config.dark_mode {
        "🌞"
    } else {
        "🌙"
    };
    if ui.button(RichText::new(text)).clicked() {
        state.config.dark_mode = !state.config.dark_mode;
    };
}

fn refersh_button(ui: &mut Ui) {
    let text = RichText::new("🔄").text_style(Body);
    if ui.button(text).clicked() {};
}

fn closeapp_button(ui: &mut Ui, frame: &Frame) {
    let text = RichText::new("❌").text_style(Body);
    if !cfg!(target_arch = "wasm32") {
        if ui.button(text).clicked() {
            frame.quit();
        };
    }
}

pub fn navbar(ctx: &Context, frame: &Frame, state: &mut State) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        let ltr = Layout::left_to_right();
        let rtl = Layout::right_to_left();

        ui.add_space(10.);
        menu::bar(ui, |ui| {
            ui.with_layout(ltr, |ui| logo(ui));
            ui.with_layout(ltr, |ui| Route::create_buttons(ui, state));
            ui.with_layout(rtl, |ui| {
                closeapp_button(ui, frame);
                refersh_button(ui);
                theme_button(ui, state);
            });
        });
        ui.add_space(10.);
    });
}
