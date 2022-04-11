use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleSource {
    pub id: Option<String>,
    pub name: String,
}

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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Articles {
    pub articles: Vec<Article>,
}

impl std::ops::Deref for Articles {
    type Target = Vec<Article>;

    fn deref(&self) -> &Self::Target {
        &self.articles
    }
}

impl Articles {
    #[cfg(feature = "colour")]
    pub fn render(&self) {
        let sep = "---------------------------------------------------------------------";
        use colour::*;
        grey_ln!(sep);
        white_ln!("NewsApi Result:");
        grey_ln!(sep);
        self.iter().for_each(|a| {
            dark_green_ln!("> {}", a.title);
            blue_ln!("  {}", a.url)
        })
    }
}
