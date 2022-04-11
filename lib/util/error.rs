use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum HLError {
    #[error("NewsApi: {0}")]
    Remote(RemoteError),
    #[error("Unexpected Error")]
    EyreError(#[from] eyre::Error),
}

#[derive(Serialize, Deserialize, Debug, derive_more::Display)]
#[display(fmt = "{}", message)]
pub struct RemoteError {
    status: String,
    code: String,
    message: String,
}

#[cfg(feature = "net_block")]
impl From<ureq::Error> for HLError {
    fn from(error: ureq::Error) -> Self {
        use eyre::eyre;
        let response = match error.into_response() {
            Some(response) => response,
            None => return Self::EyreError(eyre!("NewsApi: Request failed for unknown reason")),
        };

        let message = format!("({}: {})", response.status(), response.status_text());

        let resposne_string = match response.into_string() {
            Ok(res) => res,
            Err(e) => return Self::EyreError(eyre!("NewsApi {message}: ParseError {e}")),
        };

        match serde_json::from_str::<RemoteError>(&resposne_string) {
            Ok(mut re) => {
                re.message = format!("{message}: {}", re.message);
                Self::Remote(re)
            }
            Err(e) => Self::EyreError(eyre!("NewsApi {message}: ParseError {e}")),
        }
    }
}

#[cfg(feature = "net_async")]
impl From<(String, reqwest::Response)> for HLError {
    fn from(response: (String, reqwest::Response)) -> Self {
        use eyre::eyre;

        let message = format!("({})", response.1.status());

        let resposne_string = response.0;

        match serde_json::from_str::<RemoteError>(&resposne_string) {
            Ok(mut re) => {
                re.message = format!("{message}: {}", re.message);
                Self::Remote(re)
            }
            Err(e) => Self::EyreError(eyre!("NewsApi {message}: ParseError {e}")),
        }
    }
}
