mod error;
pub mod query;
pub use error::*;
use eyre::ContextCompat;
pub use query::*;

static BASEURL: &str = "https://newsapi.org/v2/everything";
static APIKEY: Option<&str> = option_env!("NEWSAPI_APIKEY");

#[derive(Clone)]
pub enum SearchScope {
    Title,
    Description,
    Content,
}

impl Into<String> for SearchScope {
    fn into(self) -> String {
        match self {
            SearchScope::Title => "title".into(),
            SearchScope::Description => "description".into(),
            SearchScope::Content => "content".into(),
        }
    }
}

pub fn query_everything(query: &[&str]) -> EverythingBuilder {
    Everything::new(query)
}

pub fn get_api_key() -> eyre::Result<&'static str> {
    APIKEY
        .wrap_err("NEWSAPI_APIKEY must be set in order for this to work! visit https://newsapi.org")
}
