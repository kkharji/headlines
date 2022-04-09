use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum ArticleLanguage {
    En,
}

impl Default for ArticleLanguage {
    fn default() -> Self {
        ArticleLanguage::En
    }
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

#[cfg(test)]
mod tests {

    use super::ArticleLanguage;

    #[test]
    fn to_string() {
        assert_eq!(ArticleLanguage::En.as_ref(), "en");
    }

    #[test]
    fn from_str() {
        use std::str::FromStr;

        assert!(matches!(
            ArticleLanguage::from_str("en").unwrap(),
            ArticleLanguage::En
        ));

        assert!(matches!(
            ArticleLanguage::from_str("en").unwrap(),
            ArticleLanguage::En
        ));

        let val: ArticleLanguage = "en".try_into().unwrap();
        assert!(matches!(val, ArticleLanguage::En));
    }
}
