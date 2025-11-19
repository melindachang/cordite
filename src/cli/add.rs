//! Configures argument parser for `cordite add` subcommand.

use crate::cli::Options;
use bpaf::*;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub enum AddStrategy {
    Prompt,
    Overwrite,
    Keep,
}

impl FromStr for AddStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "prompt" => Ok(Self::Prompt),
            "overwrite" => Ok(Self::Overwrite),
            "keep" => Ok(Self::Keep),
            _ => Err("expected one of: prompt, overwrite, keep".into()),
        }
    }
}

pub fn parser() -> impl Parser<Options> {
    let strategy = long("strategy")
        .short('s')
        .help("Merge strategy: prompt (default), overwrite, keep")
        .argument::<AddStrategy>("STRATEGY")
        .fallback(AddStrategy::Prompt);
    let path = positional::<PathBuf>("PATH").help("File(s) to add");

    construct!(Options::Add { strategy, path })
}
