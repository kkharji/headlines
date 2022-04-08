mod builder;
pub mod category;
mod endpoint;
mod error;
mod search_scope;

pub use builder::*;
pub use category::*;
pub use endpoint::*;
pub use error::*;

use crate::ArticleCollection;
use eyre::{Result, WrapErr};
use search_scope::NewsApiSearchScope;
use ureq::Request;

pub struct NewsApi {
    request: Request,
}

impl NewsApi {
    pub fn new() -> NewsApiBuilder {
        NewsApiBuilder::default()
    }

    pub fn request(self) -> Result<ArticleCollection> {
        dbg!(&self.request);
        let response = self.request.call().map_err(ApiError::from)?;
        let string = response.into_string()?;
        serde_json::from_str(&string).context(format!("NewsApi Response Serialization: {}", string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};
    #[test]
    fn request_without_query() {
        let articles = NewsApi::new()
            .domains(&["techcrunch.com", "thenextweb.com"])
            .request()
            .unwrap();
        let name = (*articles)
            .first()
            .unwrap()
            .source
            .name
            .as_str()
            .to_lowercase();

        assert!(name == "techcrunch" || name == "thenextweb");
    }

    #[test]
    fn request_with_single_query() {
        let articles = NewsApi::new().query(&["api"]).request().unwrap();
        assert!(articles.iter().count() > 0);
    }

    #[test]
    fn request_with_multiple_queries() {
        let articles = NewsApi::new()
            .query(&["api", "rust", "mac", "requestothing"])
            .request()
            .unwrap();
        assert!(articles.iter().count() == 0);
    }

    #[test]
    fn request_with_dates() {
        let articles = NewsApi::new()
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
        let response = NewsApi::new()
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
