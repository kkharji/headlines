//! Simple cache so we don't consume all the request allowed for free accounts.
//! Requires `cache` feature

use crate::client::Request;
use crate::Articles;
use eyre::{bail, Result};
use redis::Commands;
use std::collections::HashMap;

pub struct Cache {
    cache: HashMap<String, Articles>,
    client: redis::Client,
}

impl Default for Cache {
    fn default() -> Self {
        // WARN: force unwrap
        let mut cache = Self {
            cache: Default::default(),
            client: redis::Client::open("redis://127.0.0.1:6379").unwrap(),
        };
        cache.load().unwrap();
        cache
    }
}

impl Cache {
    const KEY: &'static str = "newsapi_cache";
    pub fn load(&mut self) -> Result<()> {
        let mut conn = self.client.get_connection()?;
        let result: String = match conn.get(Self::KEY) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };

        self.cache = serde_json::from_str(&result)?;

        Ok(())
    }
    pub fn get(&self, url: &str) -> Result<Articles> {
        if let Some(result) = self.cache.get(url) {
            print!("C");
            return Ok(result.clone());
        }
        bail!("No cache for {url}")
    }

    pub fn all(&self) -> Articles {
        let mut articles = vec![];
        for a in self.cache.values() {
            articles.append(&mut a.articles.clone())
        }
        Articles { articles }
    }

    pub fn update(&mut self, url: String, data: Articles) -> Result<()> {
        self.cache.insert(url, data);
        self.persist()?;
        Ok(())
    }

    pub fn persist(&self) -> Result<()> {
        let mut conn = self.client.get_connection()?;
        let _: () = conn.set(Self::KEY, serde_json::to_string(&self.cache)?)?;
        Ok(())
    }
}

impl Request {
    #[cfg(feature = "net_async")]
    pub async fn from_cache_async(&self) -> Result<Articles> {
        Ok(Cache::default().all())
    }

    pub(crate) fn try_from_cache(&self, url: &str) -> (Cache, Result<Articles>) {
        let cache = Cache::default();
        if let Ok(value) = cache.get(&url) {
            return (cache, Ok(value));
        }
        (cache, Err(eyre::eyre!("No Results from cache")))
    }

    #[cfg(feature = "net_block")]
    pub fn from_cache(self) -> Result<Articles> {
        Ok(Cache::default().all())
    }

    #[cfg(feature = "egui")]
    pub fn from_cache_promise(
        self,
        ctx: &eframe::egui::Context,
    ) -> poll_promise::Promise<Result<Articles>> {
        let ctx = ctx.clone();

        poll_promise::Promise::spawn_thread("newsapi", move || {
            let articles = self.from_cache()?;
            ctx.request_repaint();
            Ok(articles)
        })
    }
}
