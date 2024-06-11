pub mod commons;
pub mod errors;
pub mod rk68;

use std::path::PathBuf;

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
    pub command: Option<Commands>,
}

#[cfg(feature = "udev")]
pub const UDEV_PATH: &str = "/etc/udev/rules.d/99-kludged.rules";

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    #[cfg(all(target_family = "unix", feature = "udev"))]
    /// Write, or configure udev rules.
    Udev {
        #[clap(default_value=UDEV_PATH)]
        path: PathBuf,
    },
}
