use super::macros::query::{push_opt_to_string, push_opt_vec, push_to_string};
use crate::{NewsApi, NewsApiEndpoint, APIKEY};
use eyre::{bail, Result};

fn ensure_no_conflicts(b: &NewsApi) -> Result<()> {
    if b.endpoint.is_top_headlines()
        && b.sources.is_some()
        && (b.category.is_some() || b.country.is_some())
    {
        bail!("catgory and sources or category and country can't exist at the same time");
    }
    Ok(())
}

/// Build request to be consumed by a handler
pub(crate) fn build<T: NewsApiRequestBuilder>(b: &NewsApi, mut r: T) -> Result<T> {
    ensure_no_conflicts(b)?;

    let mut queries: Vec<(&str, String)> = Vec::new();
    let page_size = if b.page_size.eq(&0) { 1 } else { b.page_size };
    let page = if b.page.eq(&0) { 100 } else { b.page_size };

    push_to_string!("apiKey", APIKEY, queries);
    push_to_string!("pageSize", page_size, queries);
    push_to_string!("page", page, queries);

    push_opt_vec!("q", b.query, queries, " ");

    match b.endpoint {
        NewsApiEndpoint::Everything => {
            push_to_string!("language", b.language, queries);
            push_opt_vec!("searchin", b.searchin, queries);
            push_opt_vec!("domains", b.domains, queries);
            push_opt_vec!("exclude_domains", b.exclude_domains, queries);
            push_opt_to_string!("from", b.from, queries);
            push_opt_to_string!("to", b.upto, queries);
        }
        NewsApiEndpoint::TopHeadlines => {
            push_opt_to_string!("category", b.category, queries);
            push_opt_to_string!("country", b.country, queries);
        }
    }

    // use BASEURL/sources endpoint to check whether the given sources is correct
    push_opt_vec!("sources", b.sources, queries);

    if cfg!(feature = "net_async") {
        r = r.inject_query_bulk(queries);
    } else {
        for (k, v) in queries.iter() {
            r = r.inject_query(k, v.as_ref());
        }
    };

    Ok(r)
}

pub trait NewsApiRequestBuilder {
    fn inject_query<'a, T: Into<&'a str>>(self, _key: &str, _value: T) -> Self
    where
        Self: Sized,
    {
        unimplemented!();
    }
    fn inject_query_bulk(self, _value: Vec<(&str, String)>) -> Self
    where
        Self: Sized,
    {
        unimplemented!();
    }
}

#[cfg(feature = "net_async")]
impl NewsApiRequestBuilder for reqwest::RequestBuilder {
    fn inject_query_bulk(self, value: Vec<(&str, String)>) -> Self {
        self.query(&value)
    }
}

#[cfg(feature = "net_block")]
impl NewsApiRequestBuilder for ureq::Request {
    fn inject_query<'a, T: Into<&'a str>>(self, key: &str, value: T) -> Self {
        self.query(key, value.into())
    }
}
