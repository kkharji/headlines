#![feature(derive_default_enum, default_free_fn)]

use crate::config::Config;
use crate::macros::*;
use crate::pages::Page;
use crate::style::{title_text, PADDING};
use eframe::egui::{Context, Ui};
use eframe::epaint::Vec2;
use eframe::epi::{self, Frame};
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

impl App {
    pub fn render_header(&self, ui: &mut Ui, title: &str) {
        VerticalCentered!(ui, |ui| ui.label(title_text(title)));
        Space!(PADDING, ui);
        Separator!(20., ui);
    }

    /// Render main page
    pub fn render_page(&mut self, ui: &mut Ui) {
        match self.page {
            Page::Headlines => self.render_headlines_page(ui),
            Page::Search => {
                ui.centered_and_justified(|ui| {
                    ui.heading("Someday");
                });
            }
            Page::Settings | Page::Favored => {}
        };
    }

    pub fn render_footer(&self, ctx: &Context) {
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

impl epi::App for App {
    fn name(&self) -> &str {
        "Headlines"
    }

    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn epi::Storage>) {
        fonts::configure(ctx);
        let _ = self.articles_mut(ctx);

        self.config.load();
        self.config.ensure(ctx);
    }

    fn update(&mut self, ctx: &Context, frame: &Frame) {
        self.render_navbar(ctx, frame);
        CentralPanel!(ctx, |ui| {
            self.update_style(ui);
            self.render_page(ui);
            self.render_footer(ctx);
        });
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    #[allow(unused_mut)]
    let mut options = NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(740., 960.));

    let app: Box<App> = default();

    run_native(app, options);
}
