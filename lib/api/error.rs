use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum ApiError {
    ResponseError(String),
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ResponseError(msg) => f.write_str(msg),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsAPIErrorResponse {
    status: String,
    code: String,
    message: String,
}

impl From<ureq::Error> for ApiError {
    fn from(error: ureq::Error) -> Self {
        match error.into_response() {
            None => Self::ResponseError("NewsApi: Request failed for unknown reason".into()),
            Some(response) => {
                let status = response.status();
                let status_text = response.status_text().to_owned();
                let base = format!("NewsApi ({}: {})", status, status_text);
                match response.into_string() {
                    Ok(res) => match serde_json::from_str::<NewsAPIErrorResponse>(&res) {
                        Ok(err) => Self::ResponseError(format!("{base}: {:#?}", err)),
                        Err(err) => Self::ResponseError(format!("{base}: {:#?}", err)),
                    },
                    Err(err) => Self::ResponseError(format!("{base} {:#?}", err)),
                }
            }
        }
    }
}
