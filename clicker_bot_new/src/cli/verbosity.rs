use color_eyre::eyre::{eyre, Report};
use color_eyre::section::Section;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub enum Verbosity {
    Info,
    Warn,
    Debug,
}

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert to lowercase for RUST_LOG env var compatibility
        let lowercase = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase}")
    }
}

impl FromStr for Verbosity {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self, Report> {
        match input {
            "debug" => Ok(Verbosity::Debug),
            "info" => Ok(Verbosity::Info),
            "warn" => Ok(Verbosity::Warn),
            _ => Err(eyre!("Unknown verbosity level: {input}"))
                .suggestion("Please pick one of: info, warn, or debug"),
        }
    }
}

// #[derive(Debug)]
// pub struct UnknownVerbosityError;

// impl FromStr for Verbosity {
//     type Err = UnknownVerbosityError;

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         match input {
//             "debug" => Ok(Verbosity::Debug),
//             "info" => Ok(Verbosity::Info),
//             "warn" => Ok(Verbosity::Warn),
//             _ => Err(UnknownVerbosityError),
//         }
//     }
// }
