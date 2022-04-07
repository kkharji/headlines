use crate::api::NewsAPIErrorResponse;
use crate::{eyre, ArticleCollection, Result, WrapErr};
use ureq::Request;
mod builder;
pub use builder::EverythingBuilder;

pub struct Everything {
    request: Request,
}

impl Everything {
    pub fn new(query: &[&str]) -> EverythingBuilder {
        EverythingBuilder::new(query)
    }

    pub fn request(self) -> Result<ArticleCollection> {
        let response = self.request.call().map_err(|e| {
            let err_resp = e.into_response().unwrap();
            let status = err_resp.status();
            let status_text = err_resp.status_text().to_owned();
            let err: NewsAPIErrorResponse = err_resp.into_string().unwrap().try_into().unwrap();
            eyre!("NewsApi ({}: {}): {:#?}", status, status_text, err)
        })?;

        let reader = response.into_reader();
        let articles = serde_json::from_reader(reader).context("NewsApi Response Serialization")?;

        Ok(articles)
    }
}
