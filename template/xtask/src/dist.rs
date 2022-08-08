use std::{fs, path::Path};

use cargo_metadata::{camino::Utf8PathBuf, Metadata, MetadataCommand, Package};
use clap::Parser;
use color_eyre::eyre::Result;

use crate::util;

#[derive(Debug, Parser)]
pub struct Args {
    /// Target triple for the build
    #[clap(long)]
    target: Option<String>,
    /// Use cross tool to build
    #[clap(long)]
    use_cross: bool,
    /// Use cross if target is different from default target
    #[clap(long)]
    use_cross_if_needed: bool,
}

pub fn run(args: &Args) -> Result<()> {
    let Args {
        target,
        use_cross,
        use_cross_if_needed,
    } = args;

    let target = target.as_deref();
    let use_cross = *use_cross
        || (*use_cross_if_needed && target.map(|t| t != env!("DEFAULT_TARGET")).unwrap_or(false));

    let metadata = MetadataCommand::new().exec()?;
    let root_package = &metadata.root_package().unwrap();

    let mut artifacts_path = vec![util::collect_readme(root_package)?];
    for artifact in util::collect_licenses(root_package)? {
        artifacts_path.push(artifact);
    }
    for artifact in util::cargo_build_release_exe(&metadata, root_package, use_cross, target)? {
        artifacts_path.push(artifact);
    }
    let archive_path = create_archive(&metadata, root_package, target, &artifacts_path)?;

    tracing::info!(%archive_path, "dist completed successfully");

    Ok(())
}

fn create_archive(
    metadata: &Metadata,
    package: &Package,
    target: Option<&str>,
    artifacts: &[impl AsRef<Path>],
) -> Result<Utf8PathBuf> {
    let dist_dir = metadata.target_directory.join("dist");
    fs::create_dir_all(&dist_dir)?;

    let target = target.unwrap_or(env!("DEFAULT_TARGET"));
    let archive_name = format!("{}-v{}-{target}.tar.gz", package.name, package.version);

    let archive_path = dist_dir.join(archive_name);

    tracing::info!("Creating archive: {archive_path}");

    util::compress_files_tar_gz(&archive_path, artifacts)?;

    Ok(archive_path)
}
