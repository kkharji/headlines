use super::components::*;
use super::state::State;
use crate::pages;
use crate::state::Page;
use eframe::egui::{CentralPanel, Context, Visuals};

use eframe::epi::{self, Frame};

#[derive(Default)]
pub struct App {
    state: State,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "NewsApp"
    }

    // Lifecycle method. Called a LOT
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        self.update_theme(ctx);
        navbar(ctx, frame, &mut self.state);
        CentralPanel::default().show(ctx, |ui| {
            // header(ui, "Headlines");
            match self.state.current_page {
                Page::Headlines => pages::headlines(ui, &mut self.state),
                Page::Search => {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Someday");
                    });
                }
                Page::Home => {}
            }
            footer(ctx);
        });
    }

    // Lifecycle method. Called one time. best for preloading or configuration of the app.
    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn epi::Storage>) {
        crate::fonts::configure(ctx);
        // TODO: figure out a way to make real requests and make triggered by action.
        let _ = self.state.articles(ctx);
    }
}

impl App {
    fn update_theme(&mut self, ctx: &Context) {
        if self.state.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }
    }
}
