pub(crate) mod builder;
pub(crate) mod endpoint;
pub(crate) mod error;
pub(crate) mod r#impl;
pub(crate) mod macros;
pub(crate) mod request;

#[cfg_attr(feature = "derive_clap", derive(clap::Parser))]
#[derive(Default)]
pub struct NewsApi {
    /// NewsApi Search Query:
    ///
    /// Must appear (Eg: +bitcoin).
    /// Must not appear (Eg: -bitcoin).
    /// AND / OR / NOT keywords: (Eg: crypto, AND, (ethereum, OR, litecoin), NOT, bitcoin)
    pub(super) query: Vec<String>,

    /// Type of query: everything, top-headings
    #[cfg_attr(feature = "derive_clap", clap(short, long, default_value_t = endpoint::NewsApiEndpoint::Everything))]
    pub(super) endpoint: endpoint::NewsApiEndpoint,

    /// Scope to match query in. Valid Options: title, description, content,
    #[cfg_attr(feature = "derive_clap", clap(short, long = "scope"))]
    pub(super) searchin: Vec<crate::ArticleSearchScope>,

    /// Article language
    #[cfg_attr(feature = "derive_clap", clap(short = 'L', long, default_value_t = crate::ArticleLanguage::En))]
    pub(super) language: crate::ArticleLanguage,

    /// Limit number of results to return
    #[cfg_attr(
        feature = "derive_clap",
        clap(short = 'l', long = "limit", default_value_t = 10)
    )]
    pub(super) page_size: u32,

    /// Page through results
    #[cfg_attr(feature = "derive_clap", clap(short = 'P', long, default_value_t = 1))]
    pub(super) page: u32,

    /// Source to search in. Max 20 sources.
    #[cfg_attr(feature = "derive_clap", clap(short = 'i', long = "in"))]
    pub(super) sources: Option<Vec<String>>,

    /// Doamins to search in
    #[cfg_attr(feature = "derive_clap", clap(short, long))]
    pub(super) domains: Option<Vec<String>>,

    /// Doamins to exclude
    #[cfg_attr(feature = "derive_clap", clap(short = 'E', long))]
    pub(super) exclude_domains: Option<Vec<String>>,

    /// Date range start
    #[cfg_attr(feature = "derive_clap", clap(short, long))]
    pub(super) from: Option<chrono::NaiveDate>,

    /// Date range end
    #[cfg_attr(feature = "derive_clap", clap(short, long))]
    pub(super) to: Option<chrono::NaiveDate>,

    /// Article category
    #[cfg_attr(feature = "derive_clap", clap(short, long))]
    pub(super) category: Option<crate::ArticleCategory>,

    /// Source country
    #[cfg_attr(feature = "derive_clap", clap(short = 'C', long))]
    pub(super) country: Option<String>,
}

#[cfg(test)]
mod tests;
