#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ArticleSource {
    pub id: Option<String>,
    pub name: String,
}
