//! Argument parser and high-level workflow handlers for the command line.

use crate::config::AppConfig;
use anyhow::{Error, anyhow};
use bpaf::*;
use std::{path::PathBuf, str::FromStr};

pub mod add;
pub mod grep;
pub mod lint;
pub mod remove;
pub mod sync;

#[derive(Debug, Clone)]
pub enum EntityKind {
    Track,
    Release,
    Artist,
}

impl FromStr for EntityKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "track" => Ok(Self::Track),
            "release" => Ok(Self::Release),
            "artist" => Ok(Self::Artist),
            _ => Err(anyhow!("expected one of: track, release, artist")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Options {
    Add {
        no_write: bool,
        no_auto_tag: bool,
        auto_rename: bool,
        file_action: add::AddFileAction,
        strategy: add::AddStrategy,
        path: PathBuf,
    },
    Remove {
        kind: EntityKind,
        pattern: String,
    },
    Grep {
        kind: EntityKind,
        format: String,
        pattern: String,
    },
    Lint {
        full: bool,
        verbose: bool,
    },
    Sync {
        kind: EntityKind,
        strategy: sync::SyncStrategy,
    },
}

pub fn options(cfg: &AppConfig) -> OptionParser<Options> {
    let help = long("help").short('h').help("Prints help information");
    let version = long("version")
        .short('v')
        .help("Prints version information");

    // Subcommands
    let add = add::parser(cfg.commands.add.clone())
        .to_options()
        .descr("Add file(s) to library")
        .command("add");

    let remove = remove::parser()
        .to_options()
        .descr("Remove matching items from library")
        .command("remove")
        .long("rm");

    let grep = grep::parser()
        .to_options()
        .descr("Search library for pattern")
        .command("grep");

    let lint = lint::parser()
        .to_options()
        .descr("Check library files for issues")
        .command("lint");

    let sync = sync::parser(cfg.commands.sync.clone())
        .to_options()
        .descr("Sync library metadata with files")
        .command("sync");

    construct!([add, remove, grep, lint, sync])
        .to_options()
        .descr("beets if it slayed")
        .version("0.0.1")
        .help_parser(help)
        .version_parser(version)
}

pub fn dispatch(opts: Options) -> anyhow::Result<()> {
    match opts {
        Options::Add { .. } => add::run(opts)?,
        Options::Remove { .. } => remove::run(opts)?,
        Options::Grep { .. } => grep::run(opts)?,
        Options::Lint { .. } => lint::run(opts)?,
        Options::Sync { .. } => sync::run(opts)?,
    }

    Ok(())
}
