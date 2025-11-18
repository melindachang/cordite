use bpaf::*;

use crate::cli::Command;

pub fn list() -> impl Parser<Command> {
    let format = long("format")
        .short('f')
        .help("Format string for output")
        .argument("FORMAT");
    let pattern = positional::<String>("PATTERN").help("Pattern to search for");

    construct!(Command::List { format, pattern })
}
