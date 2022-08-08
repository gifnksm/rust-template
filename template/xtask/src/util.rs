use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
    process::{Command, Stdio},
};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Message, Metadata, Package,
};
use color_eyre::eyre::{ensure, Result};
use flate2::{write::GzEncoder, Compression};

fn package_root(package: &Package) -> &Utf8Path {
    package.manifest_path.parent().unwrap()
}

pub fn collect_readme(package: &Package) -> Result<Utf8PathBuf> {
    let path = package_root(package).join("README.md");
    ensure!(path.is_file(), "README.md is not a file: {path}");
    Ok(path)
}

pub fn collect_licenses(package: &Package) -> Result<Vec<Utf8PathBuf>> {
    let mut artifacts = vec![];
    for ent in fs::read_dir(package_root(package))? {
        let ent = ent?;
        let path = ent.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.starts_with("LICENSE-")
            || file_name.starts_with("LICENSE.")
            || file_name == "LICENSE"
        {
            ensure!(path.is_file(), "LICENSE is not a file: {}", path.display());
            artifacts.push(path.try_into()?);
        }
    }
    Ok(artifacts)
}

pub fn cargo_build_release_exe(
    metadata: &Metadata,
    package: &Package,
    use_cross: bool,
    target: Option<&str>,
) -> Result<Vec<Utf8PathBuf>> {
    let cmd_name = if use_cross { "cross" } else { "cargo" };
    let mut args = vec![
        "build",
        "--release",
        "--package",
        package.name.as_str(),
        "--message-format=json-render-diagnostics",
    ];

    if let Some(target) = target {
        args.extend_from_slice(&["--target", target]);
    }

    let mut cmd = Command::new(cmd_name);
    cmd.args(&args);

    tracing::info!("Running {} {}", cmd_name, args.join(" "));
    let mut cmd = cmd.stdout(Stdio::piped()).spawn()?;

    let mut artifacts = vec![];
    let reader = BufReader::new(cmd.stdout.take().unwrap());
    for message in Message::parse_stream(reader) {
        if let Message::CompilerArtifact(msg) = message? {
            if let Some(exe) = &msg.executable {
                let exe = if use_cross {
                    metadata
                        .target_directory
                        .join(exe.strip_prefix("/target").unwrap())
                } else {
                    exe.clone()
                };
                tracing::info!("Found artifact: {}", exe);
                ensure!(exe.is_file(), "Artifact is not a file: {exe}");
                artifacts.push(exe);
            }
        }
    }

    Ok(artifacts)
}

pub fn compress_files_tar_gz(
    archive_path: impl AsRef<Path>,
    files: &[impl AsRef<Path>],
) -> Result<()> {
    let archive = File::create(&archive_path)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for src_path in files {
        let src_path = src_path.as_ref();
        let artifact_name = src_path.file_name().unwrap().to_str().unwrap();
        tracing::info!(
            "Adding artifact: {} as {}",
            src_path.display(),
            artifact_name
        );
        tar.append_path_with_name(src_path, artifact_name)?;
    }

    Ok(())
}
