use super::error::NewsApiError;
use super::{request, NewsApi};
use crate::{ArticleCollection, Result};

#[cfg(feature = "net_async")]
use super::error::NewsApiErrorResponse;

impl NewsApi {
    #[cfg(feature = "net_async")]
    pub async fn request_async(self) -> Result<ArticleCollection> {
        let client = reqwest::Client::new();
        let request = request::build(&self, client.get(self.url()))?.build()?;

        #[cfg(feature = "cache")]
        let (url, mut cache) = {
            let url = request.url().to_string();
            let (cache, value) = self.try_from_cache(&url);
            if value.is_ok() {
                return value;
            } else {
                (url, cache)
            }
        };

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

    #[cfg(feature = "net_block")]
    pub fn request(self) -> Result<ArticleCollection> {
        let request = request::build(&self, ureq::get(&self.url()))?;

        #[cfg(feature = "cache")]
        let (url, mut cache) = {
            let url = request.url().to_string();
            let (cache, value) = self.try_from_cache(&url);
            if value.is_ok() {
                return value;
            } else {
                (url, cache)
            }
        };

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
