use clap::Parser;
use clicker_bot::cli::{verbosity::Verbosity, Cli};
use clicker_bot::component::bot::Bot;
use color_eyre::eyre::{Report, Result};
use log::info;
use std::env;
use std::str::FromStr;

fn setup(args: Cli) -> Result<(), Report> {
    let verbosity = args.verbosity;
    if let Some(verbosity) = verbosity {
        let level = Verbosity::from_str(&verbosity).expect("Unknown verbosity level");
        env::set_var("RUST_LOG", level.to_string());
    }

    env_logger::init();

    Ok(())
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    // Parse CLI parameters
    let args = Cli::parse();

    // Misc setup actions like logging
    setup(args.clone()).unwrap();

    // If the config file doesn't exist, create a fresh bot
    if !args.config.exists() | args.overwrite {
        let mut bot = Bot::new()?;
        bot.export(&args.config)?;
    }

    let bot = Bot::from_config(&args.config)?;

    info!("Starting bot.");
    bot.start()?;

    Ok(())
}
