use strum::{AsRefStr, EnumString};
#[derive(AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum NewsApiEndpoint {
    Everything,
    #[strum(serialize = "top-headlines")]
    TopHeadlines,
}

impl NewsApiEndpoint {
    pub fn inject_url(&self, url: &str) -> String {
        format!("{url}/{}", self.as_ref())
    }

    /// Returns `true` if the news api endpoint is [`Everything`].
    ///
    /// [`Everything`]: NewsApiEndpoint::Everything
    pub fn is_everything(&self) -> bool {
        matches!(self, Self::Everything)
    }

    /// Returns `true` if the news api endpoint is [`TopHeadlins`].
    ///
    /// [`TopHeadings`]: NewsApiEndpoint::TopHeadings
    pub fn is_top_headlines(&self) -> bool {
        matches!(self, Self::TopHeadlines)
    }
}

impl Into<String> for NewsApiEndpoint {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for NewsApiEndpoint {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for NewsApiEndpoint {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use eyre::Context;
        use std::str::FromStr;
        Self::from_str(&value).context("Parse failure")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn converts_with_into_string() {
        let everything: String = NewsApiEndpoint::Everything.into();
        assert_eq!("everything".to_string(), everything);
        let top_headlines: String = NewsApiEndpoint::TopHeadlines.into();
        assert_eq!("top-headlines".to_string(), top_headlines);
    }

    #[test]
    fn converts_with_to_string() {
        assert_eq!(
            "top-headlines".to_string(),
            NewsApiEndpoint::TopHeadlines.to_string()
        )
    }

    #[test]
    fn converts_from_string() {
        let scope = NewsApiEndpoint::try_from("top-headlines").unwrap();
        assert!(matches!(scope, NewsApiEndpoint::TopHeadlines))
    }
}
