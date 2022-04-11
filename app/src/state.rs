use crate::mode::Mode;
use crate::App;
use eframe::egui::Context;
use headlines::client::Request;
use headlines::{Articles, Result};
use poll_promise::Promise;

#[derive(Default)]
pub struct Config {
    pub mode: Mode,
}

impl App {
    /// Get a reference to the state's articles.
    pub fn articles(&self, _ctx: &Context) -> Option<&Result<Articles>> {
        self.articles.as_ref().unwrap().ready()
    }

    /// Get a reference to the state's articles.
    pub fn articles_mut(&mut self, ctx: &Context) -> &mut Promise<Result<Articles>> {
        self.articles
            .get_or_insert_with(|| Request::default().from_cache_promise(ctx))
    }
}
