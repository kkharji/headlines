mod collection;
mod source;

pub use collection::*;
use serde::{Deserialize, Serialize};
pub use source::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub source: ArticleSource,
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub url: String,
}
