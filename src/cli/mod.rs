use bpaf::*;
use std::path::PathBuf;

mod add;
mod list;

pub enum Command {
    Add { path: PathBuf },
    List { format: String, pattern: String },
}
