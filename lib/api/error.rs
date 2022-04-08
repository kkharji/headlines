use eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("NewsApi: {0}")]
    ResponseError(ApiResponseError),
    #[error("Unexpected Error")]
    EyreError(#[from] eyre::Error),
}

#[derive(Serialize, Deserialize, Debug, derive_more::Display)]
#[display(fmt = "{}", message)]
pub struct ApiResponseError {
    status: String,
    code: String,
    message: String,
}

impl From<ureq::Error> for ApiError {
    fn from(error: ureq::Error) -> Self {
        let response = match error.into_response() {
            Some(response) => response,
            None => return Self::EyreError(eyre!("NewsApi: Request failed for unknown reason")),
        };

        let message = format!("({}: {})", response.status(), response.status_text());

        let resposne_string = match response.into_string() {
            Ok(res) => res,
            Err(e) => return Self::EyreError(eyre!("NewsApi {message}: ParseError {e}")),
        };

        match serde_json::from_str::<ApiResponseError>(&resposne_string) {
            Ok(mut re) => {
                re.message = format!("{message}: {}", re.message);
                Self::ResponseError(re)
            }
            Err(e) => Self::EyreError(eyre!("NewsApi {message}: ParseError {e}")),
        }
    }
}
