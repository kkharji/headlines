use eframe::egui::Context;
use newsapp::{ArticleCollection, NewsApi, Result};
use poll_promise::Promise;
use strum::{AsRefStr, EnumString};

#[derive(Default)]
pub struct State {
    pub current_page: Page,
    articles: Option<Promise<Result<ArticleCollection>>>,
    pub config: Config,
}

impl State {
    /// Get a reference to the state's articles.
    pub fn articles(&mut self, ctx: &Context) -> Option<&mut Result<ArticleCollection>> {
        self.articles
            .get_or_insert_with(|| NewsApi::default().request_from_cache_promise(ctx))
            .ready_mut()
    }

    /// Set the state's current page.
    pub fn set_current_page(&mut self, current_page: Page) {
        self.current_page = current_page;
    }
}

pub struct Config {
    pub dark_mode: bool,
}

#[derive(EnumString, AsRefStr, Debug, PartialEq)]
pub enum Page {
    Search,
    Home,
    Headlines,
}

impl Default for Page {
    fn default() -> Self {
        Self::Headlines
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}
