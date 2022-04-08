use super::NewsApiCategory;
use crate::api::{NewsApi, NewsApiEndpoint, NewsApiSearchScope};
use crate::{ArticleCollection, NewsApiCache};
use chrono::NaiveDate;
use color_eyre::Result;
use eyre::{bail, ContextCompat};

static BASEURL: &str = "https://newsapi.org/v2";
static APIKEY: Option<&str> = option_env!("NEWSAPI_APIKEY");

pub fn get_api_key() -> Result<&'static str> {
    let errmsg = "NEWSAPI_APIKEY must be set in order for this to work! visit https://newsapi.org";
    APIKEY.wrap_err(errmsg)
}

pub struct NewsApiBuilder<'cache> {
    endpoint: NewsApiEndpoint,
    query: Option<Vec<String>>,
    searchin: Option<Vec<NewsApiSearchScope>>,
    sources: Option<Vec<String>>,
    domains: Option<Vec<String>>,
    exclude_domains: Option<Vec<String>>,
    from: Option<NaiveDate>,
    upto: Option<NaiveDate>,
    language: String,
    page_size: u32,
    page: u32,
    category: Option<NewsApiCategory>,
    country: Option<String>,
    cache: Option<&'cache mut NewsApiCache>,
}

impl<'cache> Default for NewsApiBuilder<'cache> {
    fn default() -> Self {
        Self {
            query: None,
            searchin: None,
            sources: None,
            domains: None,
            exclude_domains: None,
            from: None,
            upto: None,
            language: "en".into(),
            endpoint: NewsApiEndpoint::Everything,
            page_size: 100,
            page: 1,
            category: None,
            country: None,
            cache: None,
        }
    }
}

impl<'cache> NewsApiBuilder<'cache> {
    /// Wrapper over NewsApi::request
    pub fn request(self) -> Result<ArticleCollection> {
        let url = self.endpoint.inject_url(BASEURL);

        let mut request = ureq::get(&url)
            .query("apiKey", get_api_key()?)
            .query("pageSize", &self.page_size.to_string())
            .query("page", &self.page.to_string());

        if self.query.is_some() {
            request = request.query("q", self.query.unwrap().join(" ").as_str());
        }

        if self.endpoint.is_top_headings()
            && self.sources.as_ref().is_some()
            && (self.category.as_ref().is_some() || self.country.as_ref().is_some())
        {
            bail!("catgory and sources or category and country can't exist at the same time");
        }

        if self.endpoint.is_everything() {
            request = request.query("language", &self.language);

            if self.searchin.is_some() {
                request = request.query("searchin", &force_join(self.searchin, ","))
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
            if self.upto.is_some() {
                request = request.query("to", self.upto.unwrap().to_string().as_str())
            }
        } else {
            if self.category.is_some() {
                request = request.query("category", self.category.unwrap().as_ref())
            }
            if self.country.is_some() {
                request = request.query("country", self.country.unwrap().as_ref())
            }
        }

        if self.sources.is_some() {
            // use BASEURL/sources endpoint to check whether the given sources is correct
            request = request.query("sources", &force_join(self.sources, ","))
        }

        Ok(NewsApi {
            request,
            cache: self.cache,
        }
        .request()?)
    }

    /// Set the NewsApi builder's searchin.
    pub fn searchin(mut self, searchin: &[NewsApiSearchScope]) -> Self {
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
    pub fn language(mut self, language: &str) -> Self {
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

    pub fn cache(mut self, cache: &'cache mut NewsApiCache) -> Self {
        self.cache = Some(cache);
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
