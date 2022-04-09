#[cfg(feature = "cache")]
use crate::cache::NewsApiCache;

#[cfg(feature = "net_async")]
use super::error::NewsApiErrorResponse;

use super::error::NewsApiError;
use super::{request, NewsApi};
use crate::{ArticleCollection, Result};

/// Net Async Feature Only Implementation
#[cfg(all(feature = "net_async", not(feature = "net_block")))]
impl NewsApi {
    #[cfg(feature = "cache")]
    pub async fn request_from_cache(&self) -> Result<ArticleCollection> {
        Ok(NewsApiCache::default().all())
    }

    pub async fn request(self) -> Result<ArticleCollection> {
        let client = reqwest::Client::new();
        let request = request::build(&self, client.get(self.url()))?.build()?;

        #[cfg(feature = "cache")]
        let url = request.url().to_string();
        #[cfg(feature = "cache")]
        let (mut cache, value) = self.try_from_cache(&url);
        #[cfg(feature = "cache")]
        if value.is_ok() {
            return value;
        }

        let response = client.execute(request).await?;
        let result: ArticleCollection = if response.status() != 200 {
            return response
                .json::<NewsApiErrorResponse>()
                .await
                .map(|v| Err(NewsApiError::ResponseError(v).into()))?;
        } else {
            response.json().await?
        };

        #[cfg(feature = "cache")]
        cache.update(url, result.clone())?;

        Ok(result)
    }
}

/// Net Blocking Feature Only Implementation
#[cfg(all(feature = "net_block", not(feature = "net_async")))]
impl NewsApi {
    #[cfg(feature = "cache")]
    pub fn request_from_cache(mut self) -> Result<ArticleCollection> {
        Ok(NewsApiCache::default().all())
    }

    pub fn request(self) -> Result<ArticleCollection> {
        let request = request::build(&self, ureq::get(&self.url()))?;

        #[cfg(feature = "cache")]
        let url = request.url().to_string();
        #[cfg(feature = "cache")]
        let (mut cache, value) = self.try_from_cache(&url);
        #[cfg(feature = "cache")]
        if value.is_ok() {
            return value;
        }

        let response = request.call().map_err(NewsApiError::from)?;
        let string = response.into_string()?;
        let result: ArticleCollection = eyre::Context::context(
            serde_json::from_str(&string),
            format!("NewsApi Response Serialization: {}", string),
        )?;

        #[cfg(feature = "cache")]
        cache.update(url, result.clone())?;

        Ok(result)
    }
}

#[cfg(feature = "cache")]
impl NewsApi {
    fn try_from_cache(&self, url: &str) -> (NewsApiCache, Result<ArticleCollection>) {
        let cache = NewsApiCache::default();
        if let Ok(value) = cache.get(&url) {
            return (cache, Ok(value));
        }
        (cache, Err(eyre::eyre!("No Results from cache")))
    }
}
