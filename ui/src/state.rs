use crate::app::App;
use eframe::egui::Context;
use newsapp::{ArticleCollection, NewsApi, Result};
use poll_promise::Promise;

pub struct Config {
    pub dark_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

impl App {
    /// Get a reference to the state's articles.
    pub fn articles(&self, _ctx: &Context) -> Option<&Result<ArticleCollection>> {
        self.articles.as_ref().unwrap().ready()
    }

    /// Get a reference to the state's articles.
    pub fn articles_mut(&mut self, ctx: &Context) -> &mut Promise<Result<ArticleCollection>> {
        self.articles
            .get_or_insert_with(|| NewsApi::default().request_from_cache_promise(ctx))
    }
}
