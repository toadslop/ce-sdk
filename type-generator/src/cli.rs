use clap::{Args, Parser, Subcommand};

/// Open Api Type Generator CLI for Liferay DXP.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate types from the previously loaded open-api specs.
    /// You will need to load type settings using the `LoadApi` or
    /// `LoadObject` commands.
    Generate(GenerateArgs),

    /// Load type settings for a Liferay Headless API.
    LoadApi(LoadApiArgs),

    /// Load type settings for a Liferay Object.
    LoadObject(LoadObjectArgs),
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct LoadApiArgs {
    /// The base URL of the source Liferay instance.
    #[arg(short, long)]
    pub base_url: String,

    /// User with authorization to load the API spec
    #[arg(short, long)]
    pub username: String,

    /// Password for user who has authorization to laod the API spec
    #[arg(short, long)]
    pub password: String,
}

#[derive(Args, Debug)]
pub struct LoadObjectArgs {
    /// The base URL of the source Liferay instance.
    #[arg(short, long)]
    pub base_url: String,

    /// Username
    #[arg(short, long)]
    pub username: String,

    /// The base URL of the source Liferay instance.
    #[arg(short, long)]
    pub password: String,
}
