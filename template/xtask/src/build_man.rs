use clap::{CommandFactory, Parser};
use color_eyre::eyre::Result;

use crate::util;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "build-man", skip_all, err)]
    pub(crate) fn run(&self) -> Result<()> {
        tracing::info!("Building man pages...");

        let Args {} = self;

        let man_dir = util::create_or_cleanup_xtask_package_directory("share/man")?;
        let cmd = {{ crate_name }}::Args::command();
        let package_name = "{{ project-name }}";
        let section = "1";
        for path in util::build_man_pages(&man_dir, cmd, package_name, section)? {
            let path = path?;
            tracing::info!("  {}", util::to_relative(&path));
        }

        Ok(())
    }
}
