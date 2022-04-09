use strum::{AsRefStr, EnumString};

#[derive(Clone, AsRefStr, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ArticleSearchScope {
    Title,
    Description,
    Content,
}

impl Into<String> for ArticleSearchScope {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for ArticleSearchScope {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for ArticleSearchScope {
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
        let title: String = ArticleSearchScope::Title.into();
        assert_eq!("title".to_string(), title)
    }

    #[test]
    fn converts_with_to_string() {
        assert_eq!("title".to_string(), ArticleSearchScope::Title.to_string())
    }

    #[test]
    fn converts_from_string() {
        let scope = ArticleSearchScope::try_from("title").unwrap();
        assert!(matches!(scope, ArticleSearchScope::Title))
    }
}
