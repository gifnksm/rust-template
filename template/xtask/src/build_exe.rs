use std::fs;

use clap::Parser;

use color_eyre::eyre::Result;

use crate::{metadata, util};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {
    /// Target triple for the build
    #[clap(long)]
    pub(crate) target: Option<String>,
    /// Use cross tool to build
    #[clap(long)]
    use_cross: bool,
    /// Use cross if target is different from default target
    #[clap(long)]
    use_cross_if_needed: bool,
}

impl Args {
    #[tracing::instrument(name = "build-exe", skip_all, err)]
    pub(crate) fn run(&self) -> Result<()> {
        tracing::info!("Building executables...");

        let Args {
            target,
            use_cross,
            use_cross_if_needed,
        } = self;

        let target = target.as_deref();
        let use_cross = *use_cross
            || (*use_cross_if_needed
                && target.map(|t| t != env!("DEFAULT_TARGET")).unwrap_or(false));

        let metadata = metadata::get();
        let root_package = metadata.root_package().unwrap();

        let exe_dir = util::create_or_cleanup_xtask_package_directory("bin")?;
        for src in util::cargo_build_release_exe(root_package, use_cross, target)? {
            let src = src?;
            let dest = exe_dir.join(src.file_name().unwrap());
            tracing::info!(
                "  {} -> {}",
                util::to_relative(&src),
                util::to_relative(&dest)
            );
            fs::copy(&src, &dest)?;
        }

        Ok(())
    }
}
