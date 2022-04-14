use eyre::Context;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug, Serialize, Deserialize, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ArticleCategory {
    Entertainment,
    General,
    Health,
    Science,
    Sports,
    Technology,
}

/// Inline [`ArticleCategory::Entertainment`]
#[inline(always)]
pub fn entertainment() -> ArticleCategory {
    ArticleCategory::Entertainment
}

/// Inline  [`ArticleCategory::General`]
#[inline(always)]
pub fn general() -> ArticleCategory {
    ArticleCategory::General
}

/// Inline [`ArticleCategory::Health`]
#[inline(always)]
pub fn health() -> ArticleCategory {
    ArticleCategory::Health
}

/// Inline [`ArticleCategory::Science`]
// #[inline(always)]
pub const fn science() -> ArticleCategory {
    ArticleCategory::Science
}

/// Inline [`ArticleCategory::Sports`]
#[inline(always)]
pub fn sports() -> ArticleCategory {
    ArticleCategory::Sports
}

/// Inline [`ArticleCategory::Technology`]
#[inline(always)]
pub fn technology() -> ArticleCategory {
    ArticleCategory::Technology
}

impl Into<String> for ArticleCategory {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for ArticleCategory {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for ArticleCategory {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value).context("Parse failure")
    }
}
