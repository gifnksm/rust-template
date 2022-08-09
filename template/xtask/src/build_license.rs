use std::fs;

use clap::Parser;
use color_eyre::eyre::Result;

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

        let license_dir = util::create_or_cleanup_xtask_package_directory("share/license")?;

        for src in util::collect_licenses(root_package)? {
            let dest = license_dir.join(src.file_name().unwrap());
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
