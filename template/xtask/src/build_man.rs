use std::{
    fs::{self, File},
    iter,
};

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap::{CommandFactory, Parser};
use clap_mangen::Man;
use color_eyre::eyre::Result;
use time::OffsetDateTime;

use crate::{metadata, util};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub(crate) struct Args {}

impl Args {
    #[tracing::instrument(name = "build-man", skip_all, err)]
    pub(crate) fn run(&self) -> Result<()> {
        tracing::info!("Building man pages...");

        let Args {} = self;

        let metadata = metadata::get();
        let root_package = metadata.root_package().unwrap();

        let man_dir = util::create_or_cleanup_xtask_package_directory("share/man")?;
        let cmd = {{ crate_name}}::Args::command();
        let package_name = root_package.name.as_str();
        let section = "1";
        for path in build_man_pages(&man_dir, cmd, package_name, section)? {
            let path = path?;
            tracing::info!("  {}", util::to_relative(&path));
        }

        Ok(())
    }
}

fn build_man_pages<'a>(
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

    let it = iterate_commands(cmd).map(move |cmd| {
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

fn iterate_commands<'a>(
    cmd: clap::Command<'a>,
) -> Box<dyn Iterator<Item = clap::Command<'a>> + 'a> {
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
            .flat_map(iterate_commands),
    );
    Box::new(it)
}
