use crate::mode::Mode;
use headlines::client::Request;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub mode: Mode,
    pub api_key: String,
    pub queries: Vec<Request>,
    pub current_query_key: String,
}

impl AsRef<State> for State {
    fn as_ref(&self) -> &State {
        &self
    }
}

impl State {
    pub fn get_user_query(&self, key: &str) -> Option<&Request> {
        self.queries.iter().find(|req| req.name.as_str() == key)
    }

    pub fn load(&mut self) {
        match confy::load("headlines", None) {
            Ok(config) => {
                *self = config;
                tracing::trace!("Configuration loaded!");
            }
            Err(e) => {
                tracing::error!("[Fail load config from files]: {e}");
            }
        }
    }

    pub fn persist(&self) {
        if let Err(e) = confy::store("headlines", None, self) {
            tracing::error!("[Fail to save configuration]: {:#?}", e);
        } else {
            tracing::trace!("Configuration saved!.");
        }
    }

    pub fn has_api_key(&self) -> bool {
        !self.api_key.is_empty()
    }
}
