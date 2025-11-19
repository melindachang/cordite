//! Configures argument parser for `cordite lint` subcommand.

use bpaf::*;

use crate::cli::Options;

pub fn parser() -> impl Parser<Options> {
    let full = long("full")
        .short('f')
        .help("Include filesystem-only checks (extras, duplicates)")
        .switch();
    let verbose = short('v')
        .long("verbose")
        .help("Use verbose output")
        .switch();

    construct!(Options::Lint { full, verbose })
}
