use std::{
    fs::{self, File},
    path::Path,
};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Package,
};
use color_eyre::eyre::Result;
use flate2::{write::GzEncoder, Compression};

use crate::metadata;

pub(crate) fn to_relative(path: &Utf8Path) -> &Utf8Path {
    path.strip_prefix(&metadata::get().workspace_root)
        .unwrap_or(path)
}

pub(crate) fn create_or_cleanup_xtask_package_directory(
    path: impl AsRef<Utf8Path>,
) -> Result<Utf8PathBuf> {
    let dir = metadata::xtask_package_directory().join(path);
    create_or_cleanup_dir(&dir)?;
    Ok(dir)
}

pub(crate) fn create_or_cleanup_dir(dir: impl AsRef<Path>) -> Result<()> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        fs::remove_dir_all(&dir)?;
    }
    fs::create_dir_all(&dir)?;
    Ok(())
}

pub(crate) fn package_root_directory(package: &Package) -> &Utf8Path {
    package.manifest_path.parent().unwrap()
}

pub(crate) fn compress_files_tar_gz(
    archive_path: impl AsRef<Path>,
    src_dir: impl AsRef<Path>,
) -> Result<()> {
    let archive = File::create(&archive_path)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let artifact_name = entry.file_name();
        if entry.metadata()?.is_file() {
            tar.append_path_with_name(entry.path(), artifact_name)?;
        } else {
            tar.append_dir_all(artifact_name, entry.path())?;
        }
    }

    Ok(())
}
