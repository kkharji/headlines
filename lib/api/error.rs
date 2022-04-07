use eyre::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsAPIErrorResponse {
    status: String,
    code: String,
    message: String,
}

impl TryFrom<String> for NewsAPIErrorResponse {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value)?)
    }
}
