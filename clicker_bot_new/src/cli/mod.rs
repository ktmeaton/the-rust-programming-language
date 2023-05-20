pub mod verbosity;

use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser, Serialize)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[clap(verbatim_doc_comment)]
/// DMO Bot
pub struct Cli {
    /// Config yaml file
    #[arg(short, long, required = true)]
    pub config: PathBuf,

    /// Make output more quiet or more verbose
    #[arg(short, long, default_value = "info")]
    pub verbosity: Option<String>,
}
