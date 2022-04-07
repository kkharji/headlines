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
