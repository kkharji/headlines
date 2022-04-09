use super::*;
use crate::ArticleCollection;
use chrono::{NaiveDate, Utc};

fn with_domains_builder() -> NewsApi {
    NewsApi::default().domains(&["techcrunch.com", "thenextweb.com"])
}

fn with_single_query_builder() -> NewsApi {
    NewsApi::default().query(&["api"])
}

fn with_mutliple_queries_builder() -> NewsApi {
    NewsApi::default().query(&["api", "rust", "mac", "requestothing"])
}

fn between_dates_free_builder() -> NewsApi {
    NewsApi::default().query(&["news"]).between(
        Utc::now().date().naive_utc() - chrono::Duration::weeks(4),
        Utc::now().date().naive_utc(),
    )
}

fn between_dates_paid_builder() -> NewsApi {
    NewsApi::default().query(&["news"]).between(
        NaiveDate::from_ymd(2022, 01, 20),
        Utc::now().date().naive_utc(),
    )
}

fn assert_with_domains_builder(articles: ArticleCollection) {
    assert!(articles.iter().any(|a| {
        let name = a.source.name.as_str().to_lowercase();
        name == "techcrunch" || name == "thenextweb"
    }))
}

fn assert_has_results(articles: ArticleCollection) {
    assert!(articles.iter().count() > 0);
}

fn assert_no_results(articles: ArticleCollection) {
    assert_eq!(articles.iter().count(), 0);
}

fn assert_require_payment(response: eyre::Result<ArticleCollection>) {
    assert!(response.is_err());
    let err = match response {
        Ok(_) => panic!("Should have failed"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("upgrade"));
}

#[cfg(all(feature = "net_async", not(feature = "net_block")))]
mod net_async {
    use super::*;

    #[tokio::test]
    async fn with_domains_only() {
        assert_with_domains_builder(with_domains_builder().request().await.unwrap());
    }

    #[tokio::test]
    async fn with_single_query() {
        assert_has_results(with_single_query_builder().request().await.unwrap());
    }

    #[tokio::test]
    async fn with_multiple_queries() {
        assert_no_results(with_mutliple_queries_builder().request().await.unwrap());
    }

    #[tokio::test]
    async fn with_dates() {
        assert_has_results(between_dates_free_builder().request().await.unwrap());
    }

    #[tokio::test]
    async fn with_old_dates() {
        assert_require_payment(between_dates_paid_builder().request().await)
    }
}

#[cfg(all(feature = "net_block", not(feature = "net_async")))]
mod net_block {
    use super::*;
    use chrono::{NaiveDate, Utc};
    #[test]
    fn with_domains_only() {
        assert_with_domains_builder(with_domains_builder().request().unwrap());
    }

    #[test]
    fn with_single_query() {
        assert_has_results(with_single_query_builder().request().unwrap());
    }

    #[test]
    fn with_multiple_queries() {
        assert_no_results(with_mutliple_queries_builder().request().unwrap());
    }

    #[test]
    fn with_dates() {
        assert_has_results(between_dates_free_builder().request().unwrap());
    }

    #[test]
    fn with_old_dates() {
        assert_require_payment(between_dates_paid_builder().request())
    }
}
