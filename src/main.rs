pub mod cli;
pub mod config;
pub mod metadata;

use anyhow::Result;
use config::AppConfig;

fn main() -> Result<()> {
    let cfg = AppConfig::new()?;

    let opts = cli::options(&cfg).run();
    cli::dispatch(opts)?;

    Ok(())
}
