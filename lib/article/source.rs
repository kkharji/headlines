#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ArticleSource {
    pub id: Option<String>,
    pub name: String,
}
