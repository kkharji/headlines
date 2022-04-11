#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[derive(Default)]
pub struct Request {
    /// NewsApi Search Query:
    ///
    /// Must appear (Eg: +bitcoin).
    /// Must not appear (Eg: -bitcoin).
    /// AND / OR / NOT keywords: (Eg: crypto, AND, (ethereum, OR, litecoin), NOT, bitcoin)
    pub(super) query: Vec<String>,

    /// Type of query: everything, top-headings
    #[cfg_attr(feature = "cli", clap(short, long, default_value_t = endpoint::everything()))]
    pub(super) endpoint: endpoint::EndPoint,

    /// Scope to match query in. Valid Options: title, description, content,
    #[cfg_attr(feature = "cli", clap(short, long = "scope"))]
    pub(super) scope: Vec<crate::article::scope::ArticleQueryScope>,

    /// Article language. Only valid for everything endpoint
    #[cfg_attr(feature = "cli", clap(short = 'L', long, default_value_t = crate::article::lang::en()))]
    pub(super) language: ArticleLanguage,

    /// Limit number of results to return
    #[cfg_attr(feature = "cli", clap(short = 'l', long = "limit"))]
    pub(super) page_size: Option<u32>,

    /// Page through results
    #[cfg_attr(feature = "cli", clap(short = 'P', long))]
    pub(super) page: Option<u32>,

    /// Source to search in. Max 20 sources.
    #[cfg_attr(feature = "cli", clap(short = 'i', long = "in"))]
    pub(super) sources: Option<Vec<String>>,

    /// Doamins to search in
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub(super) domains: Option<Vec<String>>,

    /// Doamins to exclude
    #[cfg_attr(feature = "cli", clap(short = 'E', long))]
    pub(super) exclude_domains: Option<Vec<String>>,

    /// Date range start
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub(super) from: Option<chrono::NaiveDate>,

    /// Date range end
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub(super) to: Option<chrono::NaiveDate>,

    /// Article category
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub(super) category: Option<ArticleCategory>,

    /// Source country
    #[cfg_attr(feature = "cli", clap(short = 'C', long))]
    pub(super) country: Option<String>,
}

/// Return inline default of [`Request`]
#[inline(always)]
pub fn request() -> Request {
    Default::default()
}

use crate::article::category::ArticleCategory;
use crate::article::ArticleLanguage;

#[cfg(test)]
mod tests;

pub mod endpoint;

pub(crate) mod builder;
pub(crate) mod macros;
pub(crate) mod parser;
pub mod request;
