//! Configures argument parser for `cordite grep` subcommand.

use bpaf::*;

use crate::cli::{EntityKind, Options};

pub fn parser() -> impl Parser<Options> {
    let r#type = short('t')
        .long("type")
        .help("Type of entity to match (one of: artist, album, track)")
        .argument::<EntityKind>("TYPE");

    let format = short('f')
        .long("format")
        .help("Display items according to template string")
        .argument("FORMAT");

    let pattern = positional::<String>("PATTERN").help("Regex pattern to search for");

    construct!(Options::Grep {
        r#type,
        format,
        pattern
    })
}
