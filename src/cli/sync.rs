//! Configures argument parser for `cordite sync` subcommand.

use std::str::FromStr;

use bpaf::*;

use crate::cli::{EntityKind, Options};

#[derive(Debug, Clone)]
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

pub fn parser() -> impl Parser<Options> {
    let r#type = short('t')
        .long("type")
        .help("Type of entity to match (one of: artist, album, track)")
        .argument::<EntityKind>("TYPE");
    let strategy = short('s')
        .long("strategy")
        .help("Merge strategy (one of: prompt, prefer-db, prefer-files)")
        .argument::<SyncStrategy>("STRATEGY")
        .fallback(SyncStrategy::Prompt);

    construct!(Options::Sync { r#type, strategy })
}
