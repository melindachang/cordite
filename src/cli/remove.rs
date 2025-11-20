//! Configures argument parser for `cordite remove` subcommand.

use bpaf::*;

use crate::cli::{EntityKind, Options};

pub fn parser() -> impl Parser<Options> {
    let kind = short('t')
        .long("type")
        .help("Type of entity to match (one of: artist, release, track)")
        .argument::<EntityKind>("TYPE")
        .fallback(EntityKind::Track);

    let pattern = positional::<String>("PATTERN").help("Regex pattern to search for");

    construct!(Options::Remove { kind, pattern })
}

pub fn run(opts: Options) -> anyhow::Result<()> {
    println!("{:?}", opts);

    Ok(())
}
