use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use color_eyre::eyre::Result;

use crate::{metadata, util};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "build-completion", skip_all, err)]
    pub(crate) fn run(&self) -> Result<()> {
        tracing::info!("Building shell compiletion files...");

        let Args {} = self;

        let metadata = metadata::get();
        let root_package = metadata.root_package().unwrap();

        let bash_dir = util::create_or_cleanup_xtask_package_directory("share/bash-completion")?;
        let fish_dir = util::create_or_cleanup_xtask_package_directory("share/fish/completions")?;
        let zsh_dir = util::create_or_cleanup_xtask_package_directory("share/zsh/site-functions")?;

        let mut cmd = {{ crate_name }}::Args::command();
        let bin_name = root_package.name.as_str();

        clap_complete::generate_to(Shell::Bash, &mut cmd, bin_name, &bash_dir)?;
        clap_complete::generate_to(Shell::Fish, &mut cmd, bin_name, &fish_dir)?;
        clap_complete::generate_to(Shell::Zsh, &mut cmd, bin_name, &zsh_dir)?;

        Ok(())
    }
}
