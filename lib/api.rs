mod endpoint;
mod error;
mod macros;
mod request;
#[cfg(test)]
mod tests;

pub use endpoint::*;
pub use error::*;

pub(crate) static BASEURL: &str = "https://newsapi.org/v2";
pub(crate) static APIKEY: &str = env!("NEWSAPI_APIKEY");

use crate::cache::NewsApiCache;
use crate::{ArticleCategory, ArticleCollection, ArticleLanguage, ArticleSearchScope, Result};
use chrono::NaiveDate;

#[derive(Default)]
pub struct NewsApi {
    #[cfg(feature = "cache")]
    pub cache: NewsApiCache,
    pub endpoint: NewsApiEndpoint,
    pub language: ArticleLanguage,
    pub page_size: u32,
    pub page: u32,
    pub query: Option<Vec<String>>,
    pub searchin: Option<Vec<ArticleSearchScope>>,
    pub sources: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub from: Option<NaiveDate>,
    pub upto: Option<NaiveDate>,
    pub category: Option<ArticleCategory>,
    pub country: Option<String>,
}

impl NewsApi {
    #[cfg(all(feature = "net_async", not(feature = "net_block")))]

    pub async fn request(mut self) -> Result<ArticleCollection> {
        let client = reqwest::Client::new();
        let request = request::build(&self, client.get(self.url()))?.build()?;

        #[cfg(feature = "cache")]
        let url = request.url().to_string();

        #[cfg(feature = "cache")]
        if let Ok(value) = self.cache.get(&url) {
            return Ok(value);
        };

        let response = client.execute(request).await?;

        let result: ArticleCollection = if response.status() != 200 {
            let err = response.json::<NewsApiResponseError>().await?;
            return Err(NewsApiError::ResponseError(err).into());
        } else {
            response.json().await?
        };

        #[cfg(feature = "cache")]
        self.cache.update(url, result.clone())?;

        Ok(result)
    }

    #[cfg(all(feature = "net_block", not(feature = "net_async")))]
    pub fn request(mut self) -> Result<ArticleCollection> {
        let request = request::build(&self, ureq::get(&self.url()))?;

        #[cfg(feature = "cache")]
        let url = request.url().to_string();

        dbg!(&url);
        #[cfg(feature = "cache")]
        if let Ok(value) = self.cache.get(&url) {
            return Ok(value);
        };

        let response = request.call().map_err(NewsApiError::from)?;
        let string = response.into_string()?;
        let result: ArticleCollection = eyre::Context::context(
            serde_json::from_str(&string),
            format!("NewsApi Response Serialization: {}", string),
        )?;

        #[cfg(feature = "cache")]
        self.cache.update(url, result.clone())?;

        Ok(result)
    }

    /// Get Request url
    fn url(&self) -> String {
        self.endpoint.inject_url(BASEURL)
    }

    /// Set the NewsApi builder's searchin.
    pub fn searchin(mut self, searchin: &[ArticleSearchScope]) -> Self {
        self.searchin = Some(searchin.to_vec());
        self
    }

    /// Set the NewsApi builder's sources.
    pub fn sources(mut self, sources: &[&str]) -> Self {
        self.sources = Some(
            sources
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set the NewsApi builder's domains.
    pub fn domains(mut self, domains: &[&str]) -> Self {
        self.domains = Some(
            domains
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set the NewsApi builder's exclude domains.
    pub fn exclude_domains(mut self, exclude_domains: &[&str]) -> Self {
        let value = exclude_domains
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        self.exclude_domains = Some(value);
        self
    }

    /// Set the NewsApi builder's from.
    pub fn from(mut self, from: NaiveDate) -> Self {
        self.from = Some(from);
        self
    }

    /// Set the NewsApi builder's to.
    pub fn upto(mut self, to: NaiveDate) -> Self {
        self.upto = Some(to);
        self
    }

    /// Set the NewsApi builder's language.
    pub fn language(mut self, language: ArticleLanguage) -> Self {
        self.language = language.into();
        self
    }

    /// Set the news api builder's page.
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Set the news api builder's page size.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = page_size;
        self
    }

    /// Set the news api builder's query.
    pub fn query(mut self, query: &[&str]) -> Self {
        let value = query
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        self.query = Some(value);
        self
    }

    pub fn between(self, from: NaiveDate, to: NaiveDate) -> Self {
        self.from(from).upto(to)
    }

    /// Set the news api builder's endpoint.
    pub fn endpoint(mut self, endpoint: NewsApiEndpoint) -> Self {
        self.endpoint = endpoint;
        self
    }
}
