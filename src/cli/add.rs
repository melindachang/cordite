use bpaf::*;
use std::path::PathBuf;

use crate::cli::Command;

pub fn add() -> impl Parser<Command> {
    let path = positional::<PathBuf>("PATH").help("File(s) to add");

    construct!(Command::Add { path })
}
