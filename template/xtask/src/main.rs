use std::env;

use clap::Parser;
use color_eyre::eyre::Result;

mod build_doc;
mod build_exe;
mod build_license;
mod build_man;
mod dist;
mod iter;
mod metadata;
mod util;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Args {
    /// Package the executables and produce a set of distributable artifacts
    Dist(dist::Args),
    /// Build the man page
    BuildMan(build_man::Args),
    /// Build the executables
    BuildExe(build_exe::Args),
    /// Build the license files
    BuildLicense(build_license::Args),
}

impl Args {
    #[tracing::instrument(name = "xtask", skip_all, err)]
    fn run(&self) -> Result<()> {
        match self {
            Args::Dist(args) => args.run(),
            Args::BuildMan(args) => args.run(),
            Args::BuildExe(args) => args.run(),
            Args::BuildLicense(args) => args.run(),
        }
    }
}

fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();
    color_eyre::install()?;

    Args::parse().run()
}
