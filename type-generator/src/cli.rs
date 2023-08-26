use clap::Parser;

/// Open Api Type Generator CLI for Liferay DXP
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File Path or Url from which to load the openapi spec.
    #[arg(short, long)]
    spec: String,
}
