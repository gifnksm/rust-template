use std::env;

use clap::Parser;
use color_eyre::eyre::Result;

mod dist;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Args {
    /// Package the executables of root package and produce a set of distributable artifacts
    Dist(dist::Args),
}

fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    color_eyre::install()?;

    let args = Args::parse();
    match &args {
        Args::Dist(args) => dist::run(args)?,
    }

    Ok(())
}
