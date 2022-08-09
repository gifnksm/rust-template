use std::fs;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::{build_doc, build_exe, build_license, build_man, metadata, util};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {
    #[clap(flatten)]
    build_doc_args: build_doc::Args,
    #[clap(flatten)]
    build_license_args: build_license::Args,
    #[clap(flatten)]
    build_man_args: build_man::Args,
    #[clap(flatten)]
    build_exe_args: build_exe::Args,
}

impl Args {
    #[tracing::instrument(name = "dist", skip_all, err)]
    pub(crate) fn run(&self) -> Result<()> {
        let Args {
            build_doc_args,
            build_license_args,
            build_man_args,
            build_exe_args,
        } = self;

        let metadata = metadata::get();
        let root_package = metadata.root_package().unwrap();

        let package_dir = util::create_or_cleanup_xtask_package_directory("")?;

        build_doc_args.run()?;
        build_license_args.run()?;
        build_man_args.run()?;
        build_exe_args.run()?;

        let dist_dir = metadata.target_directory.join("dist");
        fs::create_dir_all(&dist_dir)?;

        let target = build_exe_args
            .target
            .as_deref()
            .unwrap_or(env!("DEFAULT_TARGET"));
        let archive_name = format!(
            "{}-v{}-{target}.tar.gz",
            root_package.name, root_package.version
        );

        let archive_path = dist_dir.join(archive_name);

        tracing::info!("Creating archive: {archive_path}");

        util::compress_files_tar_gz(&archive_path, package_dir)?;

        Ok(())
    }
}
