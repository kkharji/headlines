use super::components::*;
use super::state::State;
use crate::route::Route;
use eframe::egui::{CentralPanel, Context, Visuals};
use eframe::epi::{self, Frame};
use newsapp::{ArticleCollection, NewsApi};
use poll_promise::Promise;

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
            Route::update(ui, &mut self.state);
            footer(ctx);
        });
    }

    // Lifecycle method. Called one time. best for preloading or configuration of the app.
    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn epi::Storage>) {
        crate::fonts::configure(ctx);

        // TODO: figure out a way to make real requests and make triggered by action.
        self.state.articles_mut().get_or_insert_with(|| {
            let ctx = ctx.clone();
            Promise::spawn_async(async move {
                match NewsApi::default().request_from_cache().await {
                    Ok(articles) => {
                        ctx.request_repaint();
                        articles
                    }
                    Err(err) => {
                        tracing::error!("Fail to fetch articles {err}");
                        ArticleCollection::default()
                    }
                }
            })
        });
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
