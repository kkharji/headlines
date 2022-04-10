use crate::route::Route;
use newsapp::ArticleCollection;
use poll_promise::Promise;

pub struct Config {
    pub dark_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

#[derive(Default)]
pub struct State {
    pub current_page: Route,
    articles: Option<Promise<ArticleCollection>>,
    pub config: Config,
}

impl State {
    /// Get a reference to the state's articles.
    pub fn articles(&self) -> Option<&Promise<ArticleCollection>> {
        self.articles.as_ref()
    }

    /// Get a mutable reference to the state's articles.
    pub fn articles_mut(&mut self) -> &mut Option<Promise<ArticleCollection>> {
        &mut self.articles
    }

    /// Set the state's current page.
    pub fn set_current_page(&mut self, current_page: Route) {
        self.current_page = current_page;
    }
}
