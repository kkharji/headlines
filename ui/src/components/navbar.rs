use crate::macros::*;
use crate::route::Route;
use crate::state::State;
use eframe::egui::TextStyle::Body;
use eframe::egui::{menu, Context, Layout, RichText, TopBottomPanel, Ui};
use eframe::epi::Frame;

pub fn navbar(ctx: &Context, frame: &Frame, state: &mut State) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        Space!(ui);
        menu::bar(ui, |ui| {
            Layout! { ui, left_to_right,
                |ui| Label! {
                    text: "📓 NewsApp®",
                    style: Body,
                    ui
                }
            };
            Layout! { ui, left_to_right,
                |ui| Route::create_buttons(ui, state)
            };
            Layout! { ui, right_to_left,
                |ui| {
                closeapp_button(ui, frame);
                refersh_button(ui);
                theme_button(ui, state);
            }};
        });
        Space!(ui);
    });
}

fn theme_button(ui: &mut Ui, state: &mut State) {
    let isdark = state.config.dark_mode;
    let text = if isdark { "🌞" } else { "🌙" };
    if Button!(text: text, style: Body, ui).clicked() {
        state.config.dark_mode = !state.config.dark_mode;
    };
}

fn refersh_button(ui: &mut Ui) {
    if Button!(text: "🔄", style: Body, ui).clicked() {};
}

fn closeapp_button(ui: &mut Ui, frame: &Frame) {
    if Button!(text: "❌", style: Body, ui).clicked() {
        if !cfg!(target_arch = "wasm32") {
            frame.quit();
        }
    }
}
