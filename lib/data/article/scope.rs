use strum::{AsRefStr, EnumString};

#[derive(Clone, AsRefStr, EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum ArticleQueryScope {
    Title,
    Description,
    Content,
}

/// Inline [`ArticleQueryScope::Title`]
#[inline(always)]
pub fn title() -> ArticleQueryScope {
    ArticleQueryScope::Title
}

/// Inline [`ArticleQueryScope::Description`]
#[inline(always)]
pub fn desc() -> ArticleQueryScope {
    ArticleQueryScope::Description
}

/// Inline [`ArticleQueryScope::Content`]
#[inline(always)]
pub fn content() -> ArticleQueryScope {
    ArticleQueryScope::Content
}

impl Into<String> for ArticleQueryScope {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for ArticleQueryScope {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for ArticleQueryScope {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use eyre::Context;
        use std::str::FromStr;

        Self::from_str(&value).context("Parse failure")
    }
}
