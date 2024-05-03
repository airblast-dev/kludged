use std::str::FromStr;

use clap::Arg;
use palette::{named::from_str, Srgb};

use super::errors::InvalidColor;

pub fn color_arg() -> Arg {
    Arg::new("color")
        .short('c')
        .long("color")
        .value_parser(|arg: &str| {
            if let Some(color) = from_str(arg) {
                Ok(color)
            } else {
                Srgb::<u8>::from_str(arg).map_err(|_| InvalidColor)
            }
        })
        .value_name("COLOR")
        .required(true)
}
