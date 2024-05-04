use std::str::FromStr;

use clap::{builder::EnumValueParser, Arg, ValueEnum};
use color_print::cformat;
use palette::{named::from_str, Srgb};

use super::errors::InvalidColor;

pub fn color_arg() -> Arg {
    Arg::new("color")
        .value_parser(get_color)
        .value_name("COLOR")
}

pub fn anim_arg<T: Sync + Send + ValueEnum + 'static>() -> Arg {
    Arg::new("anim")
        .short('a')
        .long("anim")
        .value_parser(EnumValueParser::<T>::new())
        .required(true)
}

pub fn show_keyboard(kb: &str) -> String {
    cformat!("<green>Displaying commands for {kb}.</green>")
}

pub fn get_color(arg: &str) -> Result<Srgb<u8>, InvalidColor> {
    if let Some(color) = from_str(arg) {
        Ok(color)
    } else {
        Srgb::<u8>::from_str(arg).map_err(|_| InvalidColor)
    }
}
