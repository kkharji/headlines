use super::Article;
use std::ops::Deref;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ArticleCollection {
    pub articles: Vec<Article>,
}

impl Deref for ArticleCollection {
    type Target = Vec<Article>;

    fn deref(&self) -> &Self::Target {
        &self.articles
    }
}
