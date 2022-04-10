mod button;
use crate::macros::*;
use crate::state::Page::{Headlines, Search};
use crate::state::State;
use button::*;
use eframe::egui::TextStyle::Body;
use eframe::egui::{menu, Context, Layout, RichText, TopBottomPanel};
use eframe::epi::Frame;

pub fn navbar(ctx: &Context, frame: &Frame, state: &mut State) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        Space!(ui);
        menu::bar(ui, |ui| {
            Layout!(ui, left_to_right, |ui| {
                Label!(text: "📓", style: Body, ui)
            });
            Layout!(ui, left_to_right, |ui| {
                ui.add_space(5.);
                page(Headlines, state, ui);
                page(Search, state, ui);
            });
            Layout!(ui, right_to_left, |ui| {
                close(ui, frame, state);
                refresh(ui, state);
                toggle_theme(ui, state);
            });
        });
        Space!(ui);
    });
}
