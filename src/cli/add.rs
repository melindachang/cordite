//! Configures argument parser for `cordite add` subcommand.

use crate::{cli::Options, config::AddConfig};
use anyhow::{Error, anyhow};
use bpaf::*;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddFileAction {
    Copy,
    Move,
    None,
}

impl FromStr for AddFileAction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "copy" => Ok(Self::Copy),
            "move" => Ok(Self::Move),
            "none" => Ok(Self::None),
            _ => Err(anyhow!("expected one of: copy, move, none")),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddStrategy {
    Prompt,
    Overwrite,
    Keep,
}

impl FromStr for AddStrategy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "prompt" => Ok(Self::Prompt),
            "overwrite" => Ok(Self::Overwrite),
            "keep" => Ok(Self::Keep),
            _ => Err(anyhow!("expected one of: prompt, overwrite, keep")),
        }
    }
}

pub fn parser(add_cfg: AddConfig) -> impl Parser<Options> {
    let no_write = long("no-write")
        .short('W')
        .help("Don't write metadata to file(s)")
        .switch()
        .fallback(!add_cfg.write_metadata);
    let no_auto_tag = long("no-auto-tag")
        .short('T')
        .help("Use track metadata as-is")
        .switch()
        .fallback(!add_cfg.auto_tag);
    let auto_rename = long("auto-rename")
        .short('r')
        .help("Auto-rename files")
        .switch()
        .fallback(add_cfg.auto_rename);
    let file_action = long("file-action")
        .short('a')
        .help("Manage files post-import: copy (default), move, none")
        .argument::<AddFileAction>("ACTION")
        .fallback(add_cfg.file_action);
    let strategy = long("strategy")
        .short('s')
        .help("Merge strategy: prompt (default), overwrite, keep")
        .argument::<AddStrategy>("STRATEGY")
        .fallback(add_cfg.strategy);
    let path = positional::<PathBuf>("PATH").help("File(s) to add");

    construct!(Options::Add {
        no_write,
        no_auto_tag,
        auto_rename,
        file_action,
        strategy,
        path
    })
}

pub fn run(opts: Options) -> anyhow::Result<()> {
    println!("{:?}", opts);

    Ok(())
}
