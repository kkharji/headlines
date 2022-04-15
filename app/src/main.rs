#![feature(derive_default_enum, default_free_fn, concat_idents)]

mod fonts;
mod macros;
mod mode;
mod navbar;
mod pages;
mod state;
mod store;
mod style;

use crate::macros::*;
use crate::pages::Page::{self, *};
use crate::state::State;
use eframe::egui::{CentralPanel as EguiCentralPanel, Context};
use eframe::epaint::Vec2;
use eframe::epi::{App as EpiApp, Frame as EpiFrame, Storage as EpiStorage};
use eframe::{run_native, NativeOptions};

use store::Store;

#[derive(Default)]
pub struct App {
    pub page: Page,
    pub state: State,
    pub store: Store,
}

impl App {
    pub fn render_pages(&mut self, ctx: &Context, _frame: &EpiFrame) {
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

    pub fn render_footer(&mut self, ctx: &Context) {
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

impl EpiApp for App {
    fn name(&self) -> &str {
        "Headlines"
    }

    fn setup(&mut self, ctx: &Context, _frame: &EpiFrame, _storage: Option<&dyn EpiStorage>) {
        self.state.load();
        self.configure_fonts(ctx);
        self.configure_styles(ctx, false);
        self.state.current_query_key = "Covid News".into();
        self.state.queries.push(
            headlines::client::request()
                .set_name(&self.state.current_query_key)
                .query(["covid-19"]),
        );
        self.store.request_articles(self.state.as_ref(), false);
    }

    fn update(&mut self, ctx: &Context, frame: &EpiFrame) {
        self.render_navbar(ctx, frame);
        self.render_pages(ctx, frame);
        self.render_footer(ctx);
    }

    fn on_exit(&mut self) {
        self.state.persist();
    }
}

fn main() {
    let app: Box<App> = Default::default();
    let mut options: NativeOptions = Default::default();
    options.initial_window_size = Some(Vec2::new(740., 960.));

    tracing_subscriber::fmt().init();

    run_native(app, options);
}
