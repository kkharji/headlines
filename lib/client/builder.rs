use super::endpoint::{self, EndPoint};
use super::Request;
use crate::article::{ArticleLanguage, ArticleQueryScope};
use chrono::NaiveDate;

static BASEURL: &str = "https://newsapi.org/v2";

// TODO: create methods for last 24, last 3 days, last week
// TODO: support setting country
// TODO: support sortBy only for everything

/// Builder
impl Request {
    /// Get Request url
    pub(super) fn url(&self) -> String {
        self.endpoint.inject_url(BASEURL)
    }

    /// Get Request url
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    /// Set the scope in which to search for with [`Request.query`]
    pub fn scope<T: AsRef<[ArticleQueryScope]> + Into<Vec<ArticleQueryScope>>>(
        mut self,
        searchin: T,
    ) -> Self {
        self.scope = searchin.into();
        self
    }

    /// Set request's sources.

    pub fn sources<T>(mut self, sources: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        self.sources = Some(
            sources
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set request's domains.
    pub fn domains<T>(mut self, domains: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        self.domains = Some(
            domains
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set request's domains to exclude.
    pub fn exclude_domains<T>(mut self, exclude_domains: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        let value = exclude_domains
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        self.exclude_domains = Some(value);
        self
    }

    /// Set min date to return articles.
    pub fn from(mut self, from: NaiveDate) -> Self {
        self.from = Some(from);
        self
    }

    /// Set max date to return articles.
    pub fn upto(mut self, to: NaiveDate) -> Self {
        self.to = Some(to);
        self
    }

    /// Set articles language.
    pub fn language(mut self, language: ArticleLanguage) -> Self {
        self.language = language.into();
        self
    }

    /// Set the request's page.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set a limit to the number of results returned.
    pub fn limit(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Set the request's query.
    pub fn query<T>(mut self, query: T) -> Self
    where
        T: IntoIterator,
        T::Item: std::fmt::Display,
    {
        let value = query
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        self.query = value;
        self
    }

    pub fn between(self, from: NaiveDate, to: NaiveDate) -> Self {
        self.from(from).upto(to)
    }

    /// Set the request's endpoint.
    pub fn endpoint(mut self, endpoint: EndPoint) -> Self {
        self.endpoint = endpoint;
        self
    }

    /// Set the request's endpoint to top-headlines
    pub fn headlines(mut self) -> Self {
        self.endpoint = endpoint::top_headlines();
        self
    }

    /// Set the request's endpoint to everything
    pub fn everything(mut self) -> Self {
        self.endpoint = endpoint::everything();
        self
    }
}
