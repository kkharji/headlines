use crate::pages::Page;
use crate::state::Config;
use eframe::egui::{CentralPanel, Context, Visuals};
use eframe::epi::{self, Frame};
use newsapp::{ArticleCollection, Result};
use poll_promise::Promise;

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
        self.render_theme(ctx);
        self.render_navbar(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            // header(ui, "Headlines");
            match self.page {
                Page::Headlines => {
                    self.render_headlines_page(ui);
                }
                Page::Search => {
                    ui.centered_and_justified(|ui| {
                        ui.heading("Someday");
                    });
                }
                Page::Home => {}
            }
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
    fn render_theme(&mut self, ctx: &Context) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }
    }
}
