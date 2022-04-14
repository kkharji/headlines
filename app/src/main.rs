#![feature(derive_default_enum, default_free_fn, concat_idents)]

use crate::config::Config;
use crate::macros::*;
use crate::pages::Page::{self, *};
use eframe::egui::{CentralPanel as EguiCentralPanel, Context};
use eframe::epaint::Vec2;
use eframe::epi::{App as EpiApp, Frame as EpiFrame, Storage as EpiStorage};
use eframe::{run_native, NativeOptions};
use headlines::{Articles, Result};
use poll_promise::Promise;
use std::default::default;

mod config;
mod fonts;
mod macros;
mod mode;
mod navbar;
mod pages;
mod state;
mod style;

#[derive(Default)]
pub struct App {
    pub page: Page,
    pub config: Config,
    pub articles: Option<Promise<Result<Articles>>>,
}

impl EpiApp for App {
    fn name(&self) -> &str {
        "Headlines"
    }

    fn setup(&mut self, ctx: &Context, _frame: &EpiFrame, _storage: Option<&dyn EpiStorage>) {
        self.config.load();
        self.articles_mut(ctx); // TODO: remove
        self.configure_fonts(ctx);
        self.configure_styles(ctx, false);
    }

    fn update(&mut self, ctx: &Context, frame: &EpiFrame) {
        self.render_navbar(ctx, frame);
        self.render_pages(ctx, frame);

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

    fn on_exit(&mut self) {
        self.config.store();
    }
}

fn main() {
    let app: Box<App> = default();
    let mut options: NativeOptions = default();
    options.initial_window_size = Some(Vec2::new(740., 960.));

    tracing_subscriber::fmt().init();
    run_native(app, options);
}

impl App {
    pub fn render_pages(&mut self, ctx: &Context, _frame: &EpiFrame) {
        EguiCentralPanel::default()
            .frame(self.get_default_frame())
            .show(ctx, |ui| {
                match self.page {
                    Headlines => self.render_headlines_page(ui),
                    Search => {}
                    Settings => self.render_settings_page(ui),
                    Favored => {}
                };
            });
    }
}
