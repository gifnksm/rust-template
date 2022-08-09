use std::{
    fs::{self, File},
    io::BufReader,
    iter,
    path::Path,
    process::{Command, Stdio},
};

use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Artifact, Message, Package,
};
use clap_mangen::Man;
use color_eyre::eyre::{ensure, Error, Result};
use flate2::{write::GzEncoder, Compression};
use time::OffsetDateTime;

use crate::{iter::TryIterator, metadata};

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

fn package_root(package: &Package) -> &Utf8Path {
    package.manifest_path.parent().unwrap()
}

pub(crate) fn collect_readme(package: &Package) -> Result<Vec<Utf8PathBuf>> {
    let path = package_root(package).join("README.md");
    ensure!(path.is_file(), "README.md is not a file: {path}");
    Ok(vec![path])
}

pub(crate) fn collect_licenses(package: &Package) -> Result<Vec<Utf8PathBuf>> {
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

pub(crate) fn cargo_build_release_exe(
    package: &Package,
    use_cross: bool,
    target: Option<&str>,
) -> Result<impl Iterator<Item = Result<Utf8PathBuf>>> {
    let metadata = metadata::get();
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

    tracing::info!("  $ {} {}", cmd_name, args.join(" "));
    let mut cmd = cmd.stdout(Stdio::piped()).spawn()?;

    let reader = BufReader::new(cmd.stdout.take().unwrap());
    let it = Message::parse_stream(reader)
        .map_err(Error::from)
        .filter_map_ok(|msg| match msg {
            Message::CompilerArtifact(Artifact { executable, .. }) => executable,
            _ => None,
        })
        .and_then(move |mut exe| {
            if use_cross {
                let relative_path = exe.strip_prefix("/target").unwrap();
                exe = metadata.target_directory.join(relative_path)
            }
            ensure!(exe.is_file(), "Artifact is not a file: {exe}");
            Ok(exe)
        });
    Ok(it)
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

pub(crate) fn build_man_pages<'a>(
    man_dir: &Utf8Path,
    mut cmd: clap::Command<'a>,
    package_name: &str,
    section: impl Into<String>,
) -> Result<impl Iterator<Item = Result<Utf8PathBuf>> + 'a> {
    cmd._build_all(); // https://github.com/clap-rs/clap/discussions/3603

    let capitalized_name = {
        let mut cs = package_name.chars();
        match cs.next() {
            Some(c) => c.to_uppercase().collect::<String>() + cs.as_str(),
            None => String::new(),
        }
    };
    let section = section.into();
    let out_dir = man_dir.join(format!("man{section}"));
    fs::create_dir_all(&out_dir)?;

    let now = OffsetDateTime::now_utc();
    let manual_name = format!("{capitalized_name} Command Manual");
    let date = format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day()
    );
    let source = format!(
        "{} {}",
        cmd.get_name(),
        cmd.get_version()
            .or_else(|| cmd.get_long_version())
            .unwrap_or_default()
    );

    let it = iter_submodules(cmd).map(move |cmd| {
        let command_name = cmd.get_name().to_string();
        let filename = format!("{command_name}.{}", section);
        let path = out_dir.join(&filename);
        let mut file = File::create(&path)?;
        let man = Man::new(cmd.clone())
            .title(command_name.to_uppercase())
            .section(&section)
            .date(&date)
            .source(&source)
            .manual(&manual_name);
        man.render(&mut file)?;
        Ok(path)
    });

    Ok(it)
}

fn iter_submodules<'a>(cmd: clap::Command<'a>) -> Box<dyn Iterator<Item = clap::Command<'a>> + 'a> {
    #[allow(clippy::needless_collect)]
    let subcommands = cmd.get_subcommands().cloned().collect::<Vec<_>>();
    let command_name = cmd.get_name().to_string();
    let command_version = cmd.get_version();
    let command_long_version = cmd.get_long_version();
    let command_author = cmd.get_author();

    let it = iter::once(cmd.clone()).chain(
        subcommands
            .into_iter()
            .map(move |mut subcommand| {
                let name = format!("{command_name}-{}", subcommand.get_name());
                subcommand = subcommand.name(name);
                if subcommand.get_version().is_none() {
                    if let Some(version) = command_version {
                        subcommand = subcommand.version(version);
                    }
                }
                if subcommand.get_long_version().is_none() {
                    if let Some(long_version) = command_long_version {
                        subcommand = subcommand.long_version(long_version);
                    }
                }
                if subcommand.get_author().is_none() {
                    if let Some(author) = command_author {
                        subcommand = subcommand.author(author);
                    }
                }
                subcommand
            })
            .flat_map(iter_submodules),
    );
    Box::new(it)
}
