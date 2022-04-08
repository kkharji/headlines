use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum ArticleCategory {
    Entertainment,
    General,
    Health,
    Science,
    Sports,
    Technology,
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
        use eyre::Context;
        use std::str::FromStr;
        Self::from_str(&value).context("Parse failure")
    }
}

#[cfg(test)]
mod tests {

    use crate::ArticleCategory;

    #[test]
    fn to_string() {
        assert_eq!(ArticleCategory::Entertainment.as_ref(), "entertainment");
        assert_eq!(ArticleCategory::Sports.as_ref(), "sports");
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        assert!(matches!(
            ArticleCategory::from_str("entertainment").unwrap(),
            ArticleCategory::Entertainment
        ));

        assert!(matches!(
            ArticleCategory::from_str("sports").unwrap(),
            ArticleCategory::Sports
        ));

        let cat: ArticleCategory = "sports".try_into().unwrap();
        assert!(matches!(cat, ArticleCategory::Sports));
    }
}
