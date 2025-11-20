//! Configures argument parser for `cordite grep` subcommand.

use bpaf::*;

use crate::cli::{EntityKind, Options};

pub fn parser() -> impl Parser<Options> {
    let kind = short('t')
        .long("type")
        .help("Type of entity to match (one of: artist, release, track)")
        .argument::<EntityKind>("TYPE")
        .fallback(EntityKind::Track);

    let format = short('f')
        .long("format")
        .help("Display items according to template string")
        .argument("FORMAT")
        .fallback("{artist} - {release} - {title}".into());

    let pattern = positional::<String>("PATTERN").help("Regex pattern to search for");

    construct!(Options::Grep {
        kind,
        format,
        pattern
    })
}

pub fn run(opts: Options) -> anyhow::Result<()> {
    println!("{:?}", opts);

    Ok(())
}
