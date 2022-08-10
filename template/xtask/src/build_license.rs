use std::fs;

use clap::Parser;
use color_eyre::eyre::{ensure, Result};

use crate::{metadata, util};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "build-license", skip_all, err)]
    pub(crate) fn run(&self) -> Result<()> {
        tracing::info!("Building license files...");

        let Args {} = self;

        let metadata = metadata::get();
        let root_package = metadata.root_package().unwrap();

        let src_dir = util::package_root_directory(root_package);
        let dest_dir = util::create_or_cleanup_xtask_package_directory("share/license")?;

        for name in ["LICENSE-MIT", "LICENSE-APACHE"] {
            let src = src_dir.join(name);
            let dest = dest_dir.join(name);
            ensure!(src.is_file(), "{name} is not a file: {src}");
            tracing::info!(
                "  {} -> {}",
                util::to_relative(&src),
                util::to_relative(&dest)
            );
            fs::copy(src, &dest)?;
        }

        Ok(())
    }
}
