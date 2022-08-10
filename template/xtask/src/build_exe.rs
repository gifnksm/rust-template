use std::{
    fs,
    io::BufReader,
    process::{Command, Stdio},
};

use cargo_metadata::{camino::Utf8PathBuf, Artifact, Message, Package};
use clap::Parser;

use color_eyre::eyre::{ensure, Error, Result};

use crate::{iter::TryIterator, metadata, util};

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
        for src in cargo_build_release_exe(root_package, use_cross, target)? {
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

fn cargo_build_release_exe(
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
