use crate::api::{get_api_key, Everything, SearchScope, BASEURL};
use crate::ArticleCollection;
use chrono::NaiveDateTime;
use color_eyre::Result;

pub struct EverythingBuilder {
    query: Vec<String>,
    searchin: Option<Vec<SearchScope>>,
    sources: Option<Vec<String>>,
    domains: Option<Vec<String>>,
    exclude_domains: Option<Vec<String>>,
    from: Option<NaiveDateTime>,
    to: Option<NaiveDateTime>,
    language: Option<String>,
}

impl EverythingBuilder {
    pub fn new(query: &[&str]) -> Self {
        Self {
            query: query
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
            searchin: None,
            sources: None,
            domains: None,
            exclude_domains: None,
            from: None,
            to: None,
            language: None,
        }
    }

    // Everyting builder and wrapper over Everything::request
    pub fn request(self) -> Result<ArticleCollection> {
        let mut request = ureq::get(BASEURL);
        request = request.query("apiKey", get_api_key()?);
        request = request.query("q", self.query.join(" ").as_str());

        if self.searchin.is_some() {
            request = request.query("searchin", &force_join(self.searchin, ","))
        }
        if self.sources.is_some() {
            // use BASEURL/sources endpoint to check whether the given sources is correct
            request = request.query("sources", &force_join(self.sources, ","))
        }
        if self.domains.is_some() {
            request = request.query("domains", &force_join(self.domains, ","))
        }
        if self.exclude_domains.is_some() {
            request = request.query("excludeDomains", &force_join(self.exclude_domains, ","))
        }
        if self.from.is_some() {
            request = request.query("from", self.from.unwrap().to_string().as_str())
        }
        if self.to.is_some() {
            request = request.query("to", self.to.unwrap().to_string().as_str())
        }
        if self.language.is_some() {
            request = request.query("language", self.language.unwrap().to_string().as_str())
        }

        (Everything { request }).request()
    }

    /// Set the everything builder's searchin.
    pub fn searchin(mut self, searchin: &[SearchScope]) -> Self {
        self.searchin = Some(searchin.to_vec());
        self
    }

    /// Set the everything builder's sources.
    pub fn sources(mut self, sources: &[&str]) -> Self {
        self.sources = Some(
            sources
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set the everything builder's domains.
    pub fn domains(mut self, domains: &[&str]) -> Self {
        self.domains = Some(
            domains
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set the everything builder's exclude domains.
    pub fn exclude_domains(mut self, exclude_domains: &[&str]) -> Self {
        self.exclude_domains = Some(
            exclude_domains
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        self
    }

    /// Set the everything builder's from.
    pub fn from(mut self, from: NaiveDateTime) -> Self {
        self.from = Some(from);
        self
    }

    /// Set the everything builder's to.
    pub fn to(mut self, to: NaiveDateTime) -> Self {
        self.to = Some(to);
        self
    }

    /// Set the everything builder's language.
    pub fn language(mut self, language: &str) -> Self {
        self.language = Some(language.into());
        self
    }
}

fn force_join<T: Into<String>>(vec: Option<Vec<T>>, sep: &str) -> String {
    vec.unwrap()
        .into_iter()
        .map(Into::into)
        .collect::<Vec<String>>()
        .join(sep)
}
