use super::*;
use crate::Articles;
use chrono::{NaiveDate, Utc};

fn with_domains_builder() -> Request {
    request().set_domains(&["techcrunch.com", "thenextweb.com"])
}

fn with_single_query_builder() -> Request {
    request().set_query(&["api"]).set_limit(1)
}

fn with_multiple_queries_builder() -> Request {
    request().set_query(&["api", "rust", "mac", "requestothing"])
}

fn between_dates_free_builder() -> Request {
    Request::default()
        .set_query(&["news"])
        .between(
            Utc::now().date().naive_utc() - chrono::Duration::weeks(4),
            Utc::now().date().naive_utc(),
        )
        .set_limit(1)
}

fn between_dates_paid_builder() -> Request {
    Request::default()
        .set_query(&["news"])
        .between(
            NaiveDate::from_ymd(2022, 01, 20),
            Utc::now().date().naive_utc(),
        )
        .set_limit(1)
}

fn assert_with_domains_builder(articles: Articles) {
    assert!(articles.iter().any(|a| {
        let name = a.source.name.as_str().to_lowercase();
        name == "techcrunch" || name == "thenextweb"
    }))
}

fn assert_has_results(articles: Articles) {
    assert!(articles.iter().count() > 0);
}

fn assert_no_results(articles: Articles) {
    assert_eq!(articles.iter().count(), 0);
}

fn assert_require_payment(response: eyre::Result<Articles>) {
    assert!(response.is_err());
    let err = match response {
        Ok(_) => panic!("Should have failed"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("upgrade"));
}

#[cfg(feature = "net_async")]
mod net_async {
    use super::*;

    #[tokio::test]
    async fn with_domains_only() {
        assert_with_domains_builder(with_domains_builder().run_async().await.unwrap());
    }

    #[tokio::test]
    async fn with_single_query() {
        assert_has_results(with_single_query_builder().run_async().await.unwrap());
    }

    #[tokio::test]
    async fn with_multiple_queries() {
        assert_no_results(with_multiple_queries_builder().run_async().await.unwrap());
    }

    #[tokio::test]
    async fn with_dates() {
        assert_has_results(between_dates_free_builder().run_async().await.unwrap());
    }

    #[tokio::test]
    async fn with_old_dates() {
        assert_require_payment(between_dates_paid_builder().run_async().await)
    }
}

#[cfg(feature = "net_block")]
mod net_block {
    use super::*;
    #[test]
    fn with_domains_only() {
        assert_with_domains_builder(with_domains_builder().run().unwrap());
    }

    #[test]
    fn with_single_query() {
        assert_has_results(with_single_query_builder().run().unwrap());
    }

    #[test]
    fn with_multiple_queries() {
        assert_no_results(with_multiple_queries_builder().run().unwrap());
    }

    #[test]
    fn with_dates() {
        assert_has_results(between_dates_free_builder().run().unwrap());
    }

    #[test]
    fn with_old_dates() {
        assert_require_payment(between_dates_paid_builder().run())
    }
}
