mod builder;
pub mod category;
mod endpoint;
mod error;
mod search_scope;

pub use builder::*;
pub use category::*;
pub use endpoint::*;
pub use error::*;

use crate::{ArticleCollection, NewsApiCache};
use eyre::{Result, WrapErr};
use search_scope::NewsApiSearchScope;
use ureq::Request;

lazy_static::lazy_static! {
    static ref CACHE: NewsApiCache = NewsApiCache::default();
}

pub struct NewsApi<'cache> {
    request: Request,
    cache: Option<&'cache mut NewsApiCache>,
}

impl<'cache> NewsApi<'cache> {
    pub fn new(cache: &mut NewsApiCache) -> NewsApiBuilder {
        NewsApiBuilder::default().cache(cache)
    }

    pub fn default() -> NewsApiBuilder<'cache> {
        NewsApiBuilder::default()
    }

    pub fn request(self) -> Result<ArticleCollection> {
        let url = self.request.url().to_string();
        if let Some(ref cache) = self.cache {
            if let Some(result) = cache.get(&url) {
                println!("From CACHE ....");
                return Ok(result.clone());
            };
        };

        let response = self.request.call().map_err(ApiError::from)?;
        let string = response.into_string()?;
        let result: ArticleCollection = serde_json::from_str(&string)
            .context(format!("NewsApi Response Serialization: {}", string))?;

        if let Some(cache) = self.cache {
            cache.insert(url.to_string(), result.clone());
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    #[test]
    fn request_without_query() {
        let articles = NewsApi::default()
            .domains(&["techcrunch.com", "thenextweb.com"])
            .request()
            .unwrap();

        assert!(articles.iter().any(|a| {
            let name = a.source.name.as_str().to_lowercase();
            name == "techcrunch" || name == "thenextweb"
        }))
    }

    #[test]
    fn request_with_single_query() {
        let articles = NewsApi::default().query(&["api"]).request().unwrap();
        assert!(articles.iter().count() > 0);
    }

    #[test]
    fn request_with_multiple_queries() {
        let articles = NewsApi::default()
            .query(&["api", "rust", "mac", "requestothing"])
            .request()
            .unwrap();
        assert!(articles.iter().count() == 0);
    }

    #[test]
    fn request_with_dates() {
        let articles = NewsApi::default()
            .query(&["news"])
            .between(
                Utc::now().date().naive_utc() - chrono::Duration::weeks(4),
                Utc::now().date().naive_utc(),
            )
            .request()
            .unwrap();
        assert_ne!(articles.iter().count(), 0);
    }

    #[test]
    fn request_with_old_dates() {
        let response = NewsApi::default()
            .query(&["news"])
            .between(
                NaiveDate::from_ymd(2022, 01, 20),
                Utc::now().date().naive_utc(),
            )
            .request();

        assert!(response.is_err());
        let err = match response {
            Ok(_) => panic!("Should have failed"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("429"))
    }
}
