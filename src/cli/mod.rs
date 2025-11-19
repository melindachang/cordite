//! Argument parser and high-level workflow handlers for the command line.

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
    Album,
    Artist,
}

impl FromStr for EntityKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "track" => Ok(Self::Track),
            "album" => Ok(Self::Album),
            "artist" => Ok(Self::Artist),
            _ => Err("expected one of: track, album, artist".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Options {
    Add {
        strategy: add::AddStrategy,
        path: PathBuf,
    },
    Remove {
        r#type: EntityKind,
        pattern: String,
    },
    Grep {
        r#type: EntityKind,
        format: String,
        pattern: String,
    },
    Lint {
        full: bool,
        verbose: bool,
    },
    Sync {
        r#type: EntityKind,
        strategy: sync::SyncStrategy,
    },
}

pub fn options() -> OptionParser<Options> {
    let help = long("help").short('h').help("Prints help information");
    let version = long("version")
        .short('v')
        .help("Prints version information");

    // Subcommands
    let add = add::parser()
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
    let sync = sync::parser()
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
