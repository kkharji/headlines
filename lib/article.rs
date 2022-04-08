mod collection;
mod source;

pub use collection::*;
use serde::{Deserialize, Deserializer, Serialize};
pub use source::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub source: ArticleSource,
    pub author: Option<String>,
    pub title: String,
    // #[serde(default)]
    #[serde(deserialize_with = "deserialize_description")]
    pub description: String,
    pub url: String,
    #[serde(rename(deserialize = "urlToImage"))]
    pub url_to_image: Option<String>,
}

/// Custom deserializer so it doesn't fail when facing null and doesn't require me to have make it
/// optional.
fn deserialize_description<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match String::deserialize(deserializer) {
        Ok(str) => str,
        Err(_) => "<<No Description>>".into(),
    })
}
