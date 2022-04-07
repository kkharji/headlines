mod error;
pub mod query;
mod search_scope;
pub use error::*;
use eyre::ContextCompat;
pub use query::*;
use search_scope::SearchScope;

static BASEURL: &str = "https://newsapi.org/v2/everything";
static APIKEY: Option<&str> = option_env!("NEWSAPI_APIKEY");

pub fn query_everything(query: &[&str]) -> EverythingBuilder {
    Everything::new(query)
}

pub fn get_api_key() -> eyre::Result<&'static str> {
    APIKEY
        .wrap_err("NEWSAPI_APIKEY must be set in order for this to work! visit https://newsapi.org")
}
