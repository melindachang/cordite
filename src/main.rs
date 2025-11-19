pub mod cli;
pub mod config;

use std::error::Error;

use config::AppConfig;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = AppConfig::new()?;

    println!("{:?}", cli::options(&cfg).run());

    Ok(())
}
