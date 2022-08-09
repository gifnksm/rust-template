use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Metadata, MetadataCommand,
};
use once_cell::sync::Lazy;

static METADATA: Lazy<Metadata> = Lazy::new(|| MetadataCommand::new().exec().unwrap());
static XTASK_WORK_DIRECTORY: Lazy<Utf8PathBuf> =
    Lazy::new(|| METADATA.target_directory.join("xtask"));
static XTASK_PACKAGE_DIRECTORY: Lazy<Utf8PathBuf> = Lazy::new(|| XTASK_WORK_DIRECTORY.join("pkg"));

pub(crate) fn get() -> &'static Metadata {
    &METADATA
}

pub(crate) fn xtask_package_directory() -> &'static Utf8Path {
    &XTASK_PACKAGE_DIRECTORY
}
