use crossbeam_channel::{Receiver, Sender};
use headlines::{Articles, Result};
use std::{collections::HashMap, default::default, thread};

use crate::state::State;
type Message = (String, Result<Articles>);

pub struct Store {
    articles: HashMap<String, Articles>,
    articles_tx: Sender<Message>,
    articles_rx: Receiver<Message>,
    articles_tx_count: u32,
}

impl Default for Store {
    fn default() -> Self {
        let (articles, articles_tx_count) = (default(), default());
        let (articles_tx, articles_rx) = crossbeam_channel::unbounded();
        Self {
            articles,
            articles_tx,
            articles_rx,
            articles_tx_count,
        }
    }
}

impl Store {
    /// Get an article set by key.
    pub fn try_get_articles(&mut self, key: &str, refresh: bool) -> Option<Result<&Articles>> {
        // TODO: handle case when the user hit refresh
        if !refresh && self.articles.contains_key(key) {
            return Some(Ok(&self.articles[key]));
        }

        if let Ok((ref key, result)) = self.articles_rx.try_recv() {
            return match result {
                Ok(articles) => {
                    self.articles.insert(key.into(), articles);
                    Some(Ok(&self.articles[key]))
                }
                Err(e) => Some(Err(e)),
            };
        }

        None
    }

    pub fn processing(&self) -> bool {
        self.articles_tx_count > 0
    }

    /// Get a reference to the store's articles.
    pub fn all_articles(&self) -> &HashMap<String, Articles> {
        &self.articles
    }

    /// Try to request articles for given key
    pub fn request_articles(&mut self, state: &State, force: bool) {
        if state.current_query_key.is_empty() {
            return;
        }

        if !force && self.articles.get(&state.current_query_key).is_some() {
            return;
        }
        let key = state.current_query_key.clone();

        let request = match state.get_user_query(&key) {
            Some(r) => r.clone(),
            None => {
                // TODO: present A UI Error
                tracing::error!(
                    "Unexpected behavior, '{}' doesn't have a value in app.config.queries",
                    state.current_query_key
                );
                return;
            }
        };

        tracing::debug!("Requesting {:?}", request);

        self.articles_tx_count += 1;
        let s = self.articles_tx.clone();

        thread::spawn(move || s.send((key.into(), request.run())));
    }
}
