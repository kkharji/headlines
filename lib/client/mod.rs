#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    /// Request Name. Used Internally and always set to empty if not explicitly set by the consumer
    #[cfg_attr(feature = "cli", clap(skip))]
    #[serde(default)]
    pub name: String,

    /// NewsApi Search Query:
    ///
    /// Must appear (Eg: +bitcoin).
    /// Must not appear (Eg: -bitcoin).
    /// AND / OR / NOT keywords: (Eg: crypto, AND, (ethereum, OR, litecoin), NOT, bitcoin)
    pub query: String,

    /// Type of query: everything, top-headings
    #[cfg_attr(feature = "cli", clap(short, long, default_value_t = endpoint::everything()))]
    pub endpoint: endpoint::EndPoint,

    /// Scope to match query in. Valid Options: title, description, content,
    #[cfg_attr(feature = "cli", clap(short, long = "scope"))]
    pub scope: Vec<ArticleQueryScope>,

    /// Article language. Only valid for everything endpoint
    #[cfg_attr(feature = "cli", clap(short = 'L', long, default_value_t = crate::article::lang::en()))]
    pub language: ArticleLanguage,

    /// Limit number of results to return
    #[cfg_attr(feature = "cli", clap(short = 'l', long = "limit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Page through results
    #[cfg_attr(feature = "cli", clap(short = 'P', long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Source to search in. Max 20 sources.
    #[cfg_attr(feature = "cli", clap(short = 'i', long = "in"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,

    /// Doamins to search in
    #[cfg_attr(feature = "cli", clap(short, long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,

    /// Doamins to exclude
    #[cfg_attr(feature = "cli", clap(short = 'E', long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,

    /// Date range start
    #[cfg_attr(feature = "cli", clap(short, long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<chrono::NaiveDate>,

    /// Date range end
    #[cfg_attr(feature = "cli", clap(short, long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<chrono::NaiveDate>,

    /// Article category
    #[cfg_attr(feature = "cli", clap(short, long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ArticleCategory>,

    /// Source country
    #[cfg_attr(feature = "cli", clap(short = 'C', long))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<country::Country>,
}

/// Return inline default of [`Request`]
#[inline(always)]
pub fn request() -> Request {
    Default::default()
}

use crate::article::category::ArticleCategory;
use crate::article::scope::ArticleQueryScope;
use crate::article::ArticleLanguage;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

pub mod endpoint;

pub(crate) mod builder;
pub mod country;
pub(crate) mod macros;
pub(crate) mod parser;
pub mod request;
