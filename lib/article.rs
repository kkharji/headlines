mod category;
mod collection;
mod source;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

pub use category::*;
pub use collection::*;
pub use source::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub source: ArticleSource,
    pub author: Option<String>,
    pub title: String,
    #[serde(deserialize_with = "deserialize_string")]
    pub description: String,
    #[serde(deserialize_with = "deserialize_string")]
    pub content: String,
    pub url: String,
    pub url_to_image: Option<String>,
    pub published_at: DateTime<Utc>,
}

/// Custom deserializer so it doesn't fail when facing null and doesn't require me to have make it
/// optional.
fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match String::deserialize(deserializer) {
        Ok(str) => str,
        Err(_) => "".into(),
    })
}
