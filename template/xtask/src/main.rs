use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::Path,
    process::{Command, Stdio},
};

use cargo_metadata::{camino::Utf8PathBuf, Message, Metadata, MetadataCommand, Package};
use clap::Parser;
use color_eyre::eyre::{ensure, Result};
use flate2::{write::GzEncoder, Compression};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
enum Cmd {
    /// Package the executables of root package and produce a set of distributable artifacts
    Dist(DistArgs),
}

#[derive(Debug, Parser)]
struct DistArgs {
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
        target,
        use_cross,
        use_cross_if_needed,
    } = args;

    let target = target.as_deref();
    let use_cross = *use_cross
        || (*use_cross_if_needed && target.map(|t| t != env!("DEFAULT_TARGET")).unwrap_or(false));

    let metadata = MetadataCommand::new().exec()?;
    let root_package = &metadata.root_package().unwrap();

    let artifacts_path = build(&metadata, root_package, use_cross, target)?;
    create_archive(&metadata, root_package, target, &artifacts_path)?;

    Ok(())
}

fn build(
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
                ensure!(exe.is_file(), "Artifact is not a file");
                artifacts.push(exe);
            }
        }
    }

    Ok(artifacts)
}

fn create_archive(
    metadata: &Metadata,
    package: &Package,
    target: Option<&str>,
    artifacts: &[impl AsRef<Path>],
) -> Result<()> {
    let dist_dir = metadata.target_directory.join("dist");
    fs::create_dir_all(&dist_dir)?;

    let target = target.unwrap_or(env!("DEFAULT_TARGET"));
    let archive_name = format!("{}-v{}-{target}.tar.gz", package.name, package.version);

    let archive_path = dist_dir.join(archive_name);

    tracing::info!("Creating archive: {archive_path}");

    let archive = File::create(&archive_path)?;
    let enc = GzEncoder::new(archive, Compression::default());
    let mut tar = tar::Builder::new(enc);

    for src_path in artifacts {
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
