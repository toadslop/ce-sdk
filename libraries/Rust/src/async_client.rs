use reqwest::{Client, Url};

pub struct LiferayAsyncClient {
    base_url: Url,
    client: Client,
}

impl LiferayAsyncClient {
    pub fn new(base_url: impl Into<Url>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }
}
