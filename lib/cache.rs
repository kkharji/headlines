//! Simple cache so we don't consume all the request allowed for free accounts.
//! Requires `cache` feature
use crate::{ArticleCollection, NewsApi};
use eyre::{bail, Result};
use redis::Commands;
use std::collections::HashMap;

pub struct NewsApiCache {
    cache: HashMap<String, ArticleCollection>,
    client: redis::Client,
}

impl Default for NewsApiCache {
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

impl NewsApiCache {
    const KEY: &'static str = "newsapi_cache";
    pub fn load(&mut self) -> Result<()> {
        let mut conn = self.client.get_connection()?;
        let result: String = match conn.get(Self::KEY) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{e}");
                return Ok(());
            }
        };

        self.cache = serde_json::from_str(&result)?;

        Ok(())
    }
    pub fn get(&self, url: &str) -> Result<ArticleCollection> {
        if let Some(result) = self.cache.get(url) {
            println!("From CACHE ....");
            return Ok(result.clone());
        }
        bail!("No cache for {url}")
    }

    pub fn all(&self) -> ArticleCollection {
        let mut articles = vec![];
        for a in self.cache.values() {
            articles.append(&mut a.articles.clone())
        }
        ArticleCollection { articles }
    }

    pub fn update(&mut self, url: String, data: ArticleCollection) -> Result<()> {
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

impl NewsApi {
    #[cfg(feature = "net_async")]
    pub async fn request_from_cache_async(&self) -> Result<ArticleCollection> {
        Ok(NewsApiCache::default().all())
    }

    pub(crate) fn try_from_cache(&self, url: &str) -> (NewsApiCache, Result<ArticleCollection>) {
        let cache = NewsApiCache::default();
        if let Ok(value) = cache.get(&url) {
            return (cache, Ok(value));
        }
        (cache, Err(eyre::eyre!("No Results from cache")))
    }

    #[cfg(feature = "net_block")]
    pub fn request_from_cache(self) -> Result<ArticleCollection> {
        Ok(NewsApiCache::default().all())
    }

}
