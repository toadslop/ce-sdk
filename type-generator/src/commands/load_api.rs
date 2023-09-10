use super::Command;
use crate::cli::LoadApiArgs;
use thiserror::Error;
use url::{ParseError, Url};

pub struct LoadSpec;

impl Command<LoadApiArgs, ()> for LoadSpec {
    fn run(self, ctx: &mut LoadApiArgs) -> anyhow::Result<()> {
        let LoadApiArgs { base_url, .. } = ctx;

        let _url = Url::parse(base_url)
            .map_err(|err| LoadSpecError::InvalidUrl(base_url.to_owned(), err))?;

        // let client = println!("{:?}", body);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum LoadSpecError {
    #[error("Could not parse the provided Liferay base url: {:?}", .0)]
    InvalidUrl(String, #[source] ParseError),
}
