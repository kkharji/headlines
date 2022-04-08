use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum NewsApiCategory {
    Entertainment,
    General,
    Health,
    Science,
    Sports,
    Technology,
}

impl Into<String> for NewsApiCategory {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

impl ToString for NewsApiCategory {
    fn to_string(&self) -> String {
        self.as_ref().into()
    }
}

impl TryFrom<String> for NewsApiCategory {
    type Error = eyre::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use eyre::Context;
        use std::str::FromStr;
        Self::from_str(&value).context("Parse failure")
    }
}

#[cfg(test)]
mod tests {

    use crate::api::NewsApiCategory;

    #[test]
    fn to_string() {
        assert_eq!(NewsApiCategory::Entertainment.as_ref(), "entertainment");
        assert_eq!(NewsApiCategory::Sports.as_ref(), "sports");
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        assert!(matches!(
            NewsApiCategory::from_str("entertainment").unwrap(),
            NewsApiCategory::Entertainment
        ));

        assert!(matches!(
            NewsApiCategory::from_str("sports").unwrap(),
            NewsApiCategory::Sports
        ));

        let cat: NewsApiCategory = "sports".try_into().unwrap();
        assert!(matches!(cat, NewsApiCategory::Sports));
    }
}
