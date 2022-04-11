use super::{parser, Request};
use crate::{Articles, HLError, Result};

#[cfg(feature = "net_async")]
use crate::util::RemoteError;

#[cfg(feature = "cli")]
pub fn from_cli_args() -> Request {
    use clap::StructOpt;
    Request::parse()
}

impl Request {
    #[cfg(feature = "net_block")]
    pub fn run(self) -> Result<Articles> {
        let request = parser::parse(&self, ureq::get(&self.url()))?;

        #[cfg(feature = "cache")]
        let (url, mut cache) = {
            let url = request.url().to_string();
            println!("{url}");
            let (cache, value) = self.try_from_cache(&url);
            if value.is_ok() {
                return value;
            } else {
                (url, cache)
            }
        };

        let response = request.call().map_err(HLError::from)?;
        let string = response.into_string()?;
        let result: Articles = eyre::Context::context(
            serde_json::from_str(&string),
            format!("NewsApi Response Serialization: {}", string),
        )?;

        #[cfg(feature = "cache")]
        cache.update(url, result.clone())?;

        Ok(result)
    }

    #[cfg(feature = "net_async")]
    pub async fn run_async(self) -> Result<Articles> {
        let client = reqwest::Client::new();
        let request = parser::parse(&self, client.get(self.url()))?.build()?;

        #[cfg(feature = "cache")]
        let (url, mut cache) = {
            let url = request.url().to_string();
            let (cache, value) = self.try_from_cache(&url);
            if value.is_ok() {
                return value;
            } else {
                (url, cache)
            }
        };

        let response = client.execute(request).await?;
        let result: Articles = if response.status() != 200 {
            return response
                .json::<RemoteError>()
                .await
                .map(|v| Err(HLError::Remote(v).into()))?;
        } else {
            response.json().await?
        };

        #[cfg(feature = "cache")]
        cache.update(url, result.clone())?;

        Ok(result)
    }

    #[cfg(feature = "egui")]
    pub fn run_promise(
        self,
        ctx: &eframe::egui::Context,
    ) -> poll_promise::Promise<Result<Articles>> {
        let ctx = ctx.clone();
        poll_promise::Promise::spawn_thread("newsapi", move || {
            let articles = self.run()?;
            ctx.request_repaint();
            Ok(articles)
        })
    }
}
