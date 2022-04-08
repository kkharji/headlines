use strum::{AsRefStr, EnumString};
#[derive(AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum NewsApiEndpoint {
    Everything,
    TopHeadings,
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

    /// Returns `true` if the news api endpoint is [`TopHeadings`].
    ///
    /// [`TopHeadings`]: NewsApiEndpoint::TopHeadings
    pub fn is_top_headings(&self) -> bool {
        matches!(self, Self::TopHeadings)
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
        let top_headings: String = NewsApiEndpoint::TopHeadings.into();
        assert_eq!("top_headings".to_string(), top_headings);
    }

    #[test]
    fn converts_with_to_string() {
        assert_eq!(
            "top_headings".to_string(),
            NewsApiEndpoint::TopHeadings.to_string()
        )
    }

    #[test]
    fn converts_from_string() {
        let scope = NewsApiEndpoint::try_from("top_headings").unwrap();
        assert!(matches!(scope, NewsApiEndpoint::TopHeadings))
    }
}
