//! Configures argument parser for `cordite sync` subcommand.

use std::str::FromStr;

use bpaf::*;
use serde::Deserialize;

use crate::{
    cli::{EntityKind, Options},
    config::SyncConfig,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SyncStrategy {
    Prompt,
    PreferDb,
    PreferFiles,
}

impl FromStr for SyncStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "prompt" => Ok(Self::Prompt),
            "prefer-db" => Ok(Self::PreferDb),
            "prefer-files" => Ok(Self::PreferFiles),
            _ => Err("expected one of: prompt, prefer-db, prefer-file".into()),
        }
    }
}

pub fn parser(sync_cfg: SyncConfig) -> impl Parser<Options> {
    let kind = short('t')
        .long("type")
        .help("Type of entity to match (one of: artist, album, track)")
        .argument::<EntityKind>("TYPE")
        .fallback(EntityKind::Track);

    let strategy = short('s')
        .long("strategy")
        .help("Merge strategy (one of: prompt, prefer-db, prefer-files)")
        .argument::<SyncStrategy>("STRATEGY")
        .fallback(sync_cfg.strategy);

    construct!(Options::Sync { kind, strategy })
}

pub fn run(opts: Options) -> anyhow::Result<()> {
    println!("{:?}", opts);

    Ok(())
}
