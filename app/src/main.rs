#![feature(derive_default_enum, default_free_fn, concat_idents)]

mod fonts;
mod macros;
mod mode;
mod navbar;
mod pages;
mod state;
mod store;
mod style;

use std::default::default;

use crate::macros::*;
use crate::pages::Page::{self, *};
use crate::state::State;
use eframe::egui::{CentralPanel as EguiCentralPanel, Context as EguiContext};
use eframe::epaint::Vec2;
use eframe::epi::{App as EpiApp, Frame as EpiFrame};
use eframe::CreationContext;
use eframe::{glow::Context as GlowContext, NativeOptions};

use store::Store;

#[derive(Default)]
pub struct App {
    pub page: Page,
    pub state: State,
    pub store: Store,
}

impl EpiApp for App {
    fn update(&mut self, ctx: &EguiContext, frame: &mut EpiFrame) {
        self.render_navbar(ctx, frame);
        self.render_pages(ctx, frame);
        self.render_footer(ctx);
    }

    fn on_exit(&mut self, _gl: &GlowContext) {
        self.state.persist();
    }
}

impl App {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut app = App::default();
        app.state.load();
        let ctx = &cc.egui_ctx;
        app.configure_fonts(&ctx);
        app.configure_styles(&ctx, false);
        app.state.current_query_key = "Covid News".into();
        app.state.queries.push(
            headlines::client::request()
                .set_name(&app.state.current_query_key)
                .set_query(["covid-19"]),
        );
        app.store.request_articles(app.state.as_ref(), false);
        app
    }

    pub fn render_pages(&mut self, ctx: &EguiContext, _frame: &EpiFrame) {
        EguiCentralPanel::default()
            .frame(self.get_default_frame())
            .show(ctx, |ui| {
                if self.state.current_query_key.is_empty() {
                    ui.add_space(50.);
                    ui.vertical_centered_justified(|ui| {
                        ui.colored_label(
                            self.white(),
                            style::text("No Queries configured or selected!").size(22.),
                        );
                        ui.heading("Navigate to settings and create a custom query.");
                    });
                    return;
                }

                match self.page {
                    Headlines => self.render_headlines_page(ui),
                    Settings => self.render_settings_page(ui),
                    Search => {}
                    Favored => {}
                };
            });
    }

    pub fn render_footer(&mut self, ctx: &EguiContext) {
        TopBottomPanel!(ctx, |ui| {
            VerticalCentered!(ui, |ui| {
                Space!(10.0, ui);
                Label!("NewsApp®", Monospace, ui);
                Hyperlink!("Powered by NewsApi", "https://newsapi.org", ui);
                Hyperlink!("Made with egui", "https://github.com/emilk/egui", ui);
                Space!(10.0, ui);
            });
        });
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    eframe::run_native(
        "Headlines",
        NativeOptions {
            initial_window_size: Some(Vec2::new(740., 960.)),
            ..default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
