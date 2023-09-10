use anyhow::Result;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use ureq::{serde::de::DeserializeOwned, Request};
use url::Url;

static AUTHORIZATION: &str = "Authorization";

pub struct LiferayBasicAuthClient {
    base_url: Url,
    auth_header: String,
}

impl LiferayBasicAuthClient {
    pub fn new(base_url: Url, username: String, password: String) -> Self {
        Self {
            base_url,
            auth_header: LiferayBasicAuthClient::user_pass_to_basic_auth(username, password),
        }
    }

    fn join_username_password(username: String, password: String) -> String {
        format!("{}:{}", username, password)
    }

    fn base64_encode(userpass: String) -> String {
        general_purpose::STANDARD.encode(userpass)
    }

    fn format_basic_auth_header(base64: String) -> String {
        format!("Basic {base64}")
    }

    fn user_pass_to_basic_auth(username: String, password: String) -> String {
        let userpass = LiferayBasicAuthClient::join_username_password(username, password);
        let base64 = LiferayBasicAuthClient::base64_encode(userpass);
        LiferayBasicAuthClient::format_basic_auth_header(base64)
    }
}

impl LiferayClient for LiferayBasicAuthClient {
    fn get_base_url(&self) -> Url {
        self.base_url.clone()
    }

    fn handle_auth(&self, req: Request) -> Request {
        req.set(AUTHORIZATION, &self.auth_header)
    }
}

pub struct LiferayOAuthClient {
    base_url: Url,
    token: Option<String>,
}

impl LiferayClient for LiferayOAuthClient {
    fn get_base_url(&self) -> Url {
        self.base_url.clone()
    }

    fn handle_auth(&self, req: Request) -> Request {
        todo!()
    }
}

pub trait LiferayClient {
    fn handle_auth(&self, req: Request) -> Request;
    fn get_base_url(&self) -> Url;
    fn get<T>(&self, endpoint: String) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut base = self.get_base_url();
        base.set_path(&endpoint);
        let req = ureq::get(base.as_ref());
        let req = self.handle_auth(req);
        Ok(req.call()?.into_json()?)
    }
}
