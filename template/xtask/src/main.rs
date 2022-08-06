use std::{
    env::{self, consts::EXE_EXTENSION},
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

use clap::Parser;
use color_eyre::eyre::{ensure, Result};
use flate2::{write::GzEncoder, Compression};

const PACKAGE_NAME: &str = "{{ project-name }}";
const EXECUTABLE_NAMES: &[&str] = &["{{ project-name }}"];

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Cmd {
    /// Package the {{ package-name }} and produce a set of distributable artifacts
    Dist(DistArgs),
}

#[derive(Debug, Parser)]
struct DistArgs {
    /// Use cross tool to build
    #[clap(long)]
    use_cross: bool,
    /// Target triple for the build
    #[clap(long)]
    target: Option<String>,
    /// Release name for the build
    #[clap(long)]
    tag: Option<String>,
}

fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();
    color_eyre::install()?;

    let cmd = Cmd::parse();
    match &cmd {
        Cmd::Dist(args) => dist(args)?,
    }

    Ok(())
}

fn dist(args: &DistArgs) -> Result<()> {
    let DistArgs {
        use_cross,
        target,
        tag,
    } = args;

    let use_cross = *use_cross;
    let target = target.as_deref();
    let tag = tag.as_deref();

    let mut artifacts_path = vec![];
    for exe_name in EXECUTABLE_NAMES {
        let path = build(exe_name, use_cross, target)?;
        artifacts_path.push(path);
    }

    create_archive(tag, target, &artifacts_path)?;

    Ok(())
}

fn build(exe_name: &str, use_cross: bool, target: Option<&str>) -> Result<PathBuf> {
    let mut target_dir = PathBuf::from("target");
    ensure!(
        target_dir.is_dir(),
        "output directory does not exist: {}",
        target_dir.display()
    );
    if let Some(target) = target {
        target_dir.push(target);
    }
    ensure!(
        target_dir.is_dir(),
        "output directory does not exist: {}",
        target_dir.display()
    );

    tracing::debug!(%use_cross, ?target, target_dir = %target_dir.display());

    let cmd = if use_cross { "cross" } else { "cargo" };
    let args = ["build", "--release", "--package", exe_name];

    if let Some(target) = target {
        tracing::info!("Running {} {} --target {}", cmd, args.join(" "), target);
        Command::new(cmd)
            .args(&args)
            .arg("--target")
            .arg(target)
            .status()?;
    } else {
        tracing::info!("Running {} {}", cmd, args.join(" "));
        Command::new(cmd).args(&args).status()?;
    }

    let mut output = target_dir.join("release").join(exe_name);
    output.set_extension(EXE_EXTENSION);
    ensure!(
        output.is_file(),
        "output file does not exist: {}",
        output.display()
    );
    Ok(output)
}

fn create_archive(tag: Option<&str>, target: Option<&str>, artifacts: &[PathBuf]) -> Result<()> {
    let mut name_parts = vec![PACKAGE_NAME];
    if let Some(tag) = tag {
        name_parts.push(tag);
    }
    if let Some(target) = target {
        name_parts.push(target);
    }

    let archive_name = format!("{}.tar.gz", name_parts.join("-"));
    let dist_dir = PathBuf::from("target/dist");
    fs::create_dir_all(&dist_dir)?;

    let archive_path = dist_dir.join(archive_name);

    tracing::info!("Creating archive: {}", archive_path.display());
    let archive = File::create(&archive_path)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for src_path in artifacts {
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
