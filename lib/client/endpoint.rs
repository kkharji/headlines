use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString};

#[derive(
    AsRefStr, EnumString, Default, Debug, Serialize, Deserialize, EnumIter, PartialEq, Clone, Copy,
)]
#[strum(serialize_all = "snake_case")]
pub enum EndPoint {
    #[strum(serialize = "top-headlines")]
    TopHeadlines,
    #[default]
    Everything,
}

impl EndPoint {
    pub(crate) fn inject_url(&self, url: &str) -> String {
        if self.is_top_headlines() {
            format!("{url}/top-headlines")
        } else {
            format!("{url}/everything")
        }
    }

    pub(crate) fn is_top_headlines(&self) -> bool {
        matches!(self, Self::TopHeadlines)
    }
}

#[inline(always)]
pub fn top_headlines() -> EndPoint {
    EndPoint::TopHeadlines
}

#[inline(always)]
pub fn everything() -> EndPoint {
    EndPoint::Everything
}

impl Into<String> for EndPoint {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for EndPoint {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for EndPoint {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use eyre::Context;
        use std::str::FromStr;
        Self::from_str(&value).context("Parse failure")
    }
}
