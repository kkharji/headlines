use super::endpoint::NewsApiEndpoint;
use super::NewsApi;
use crate::article::{ArticleLanguage, ArticleSearchScope};
use chrono::NaiveDate;

static BASEURL: &str = "https://newsapi.org/v2";

/// Builder
impl NewsApi {
    /// Get Request url
    pub(super) fn url(&self) -> String {
        self.endpoint.inject_url(BASEURL)
    }

    /// Set the NewsApi builder's searchin.
    pub fn searchin(mut self, searchin: &[ArticleSearchScope]) -> Self {
        self.searchin = searchin.to_vec();
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
        self.to = Some(to);
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
        self.query = value;
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
