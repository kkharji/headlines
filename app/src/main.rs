#![feature(derive_default_enum)]

use crate::pages::Page;
use crate::state::Config;
use crate::style::update_style;
use eframe::egui::{CentralPanel, Context, Ui};
use eframe::epaint::Vec2;
use eframe::epi::{self, Frame};
use eframe::{run_native, NativeOptions};
use headlines::{ArticleCollection, Result};
use poll_promise::Promise;

mod components;
mod fonts;
mod macros;
mod mode;
mod pages;
mod state;
mod style;

#[derive(Default)]
pub struct App {
    pub page: Page,
    pub config: Config,
    pub articles: Option<Promise<Result<ArticleCollection>>>,
}

impl epi::App for App {
    fn name(&self) -> &str {
        "NewsApp"
    }

    // Lifecycle method. Called a LOT
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        // self.render_theme(ctx);
        self.render_navbar(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            update_style(ui);
            self.render_header(ui, "Headlines");
            self.render_page(ui);
            self.render_footer(ctx);
        });
    }

    // Lifecycle method. Called one time. best for preloading or configuration of the app.
    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn epi::Storage>) {
        crate::fonts::configure(ctx);
        // TODO: figure out a way to make real requests and make triggered by action.
        let _ = self.articles_mut(ctx);
    }
}

impl App {
    /// Render main page
    pub fn render_page(&mut self, ui: &mut Ui) {
        match self.page {
            Page::Headlines => {
                self.render_headlines_page(ui);
            }
            Page::Search => {
                ui.centered_and_justified(|ui| {
                    ui.heading("Someday");
                });
            }
            Page::Settings => {}
            Page::Favored => {}
        }
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    #[allow(unused_mut)]
    let mut options = NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(740., 960.));

    run_native(Box::new(App::default()), options);
}
