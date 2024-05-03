pub mod commons;
pub mod errors;
pub mod rk68;

use clap::{clap_derive::Parser, Subcommand};
use clap_verbosity_flag::Verbosity;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long)]
    /// Launch the GUI.
    ///
    /// If this flag is enabled, any other argument (excluding verbosity) is ignored.
    pub gui: bool,

    #[command(flatten)]
    /// Set the level of verbosity.
    pub verbosity: Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[cfg(target_family = "unix")]
    /// Write, or configure udev rules.
    Udev,
}
