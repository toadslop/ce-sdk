use std::ops::Deref;

use ureq::{Agent, Error, MiddlewareNext, Request, Response};
use url::Url;

pub struct LiferayBlockingClient {
    base_url: String,
    client: Agent,
}

impl LiferayBlockingClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Agent::new(),
        }
    }

    pub fn get(endpoint: impl Into<String>) {
        let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    }

    fn ad_base_url(&self, req: Request, next: MiddlewareNext) -> Result<Response, Error> {
        let url = req.url();
        let mut parsed = Url::parse(url)?;
        parsed.set_host(Some(&self.base_url))?;
        let url = req.request_url()?;
        Request { url };
        next.handle(req)
    }
}

impl Deref for LiferayBlockingClient {
    type Target = Agent;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
