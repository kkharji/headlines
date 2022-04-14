use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug, Default, Serialize, Deserialize, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ArticleLanguage {
    #[default]
    En,
}

/// Inline [`ArticleLanguage::En`]
#[inline(always)]
pub fn en() -> ArticleLanguage {
    ArticleLanguage::En
}

impl Into<String> for ArticleLanguage {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for ArticleLanguage {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for ArticleLanguage {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use eyre::Context;
        use std::str::FromStr;
        Self::from_str(&value).context("Parse failure")
    }
}
