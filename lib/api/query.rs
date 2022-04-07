mod everything;
use crate::api::ApiError;
use crate::{ArticleCollection, Result, WrapErr};
use ureq::Request;

pub use everything::*;

pub struct Everything {
    request: Request,
}

impl Everything {
    pub fn new(query: &[&str]) -> EverythingBuilder {
        EverythingBuilder::new(query)
    }

    pub fn request(self) -> Result<ArticleCollection> {
        let response = self.request.call().map_err(ApiError::from)?;
        let reader = response.into_reader();
        serde_json::from_reader(reader).context("NewsApi Response Serialization")
    }
}
