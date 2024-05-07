use std::str::FromStr;

use clap::Arg;
use color_print::cformat;
use palette::{named::from_str, Srgb};

use super::errors::InvalidColor;

pub fn color_arg() -> Arg {
    Arg::new("color")
        .value_parser(get_color)
        .short('c')
        .long("color")
        .value_name("COLOR")
        .default_value("red")
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

/// A quick way to create a [`clap::builder::PossibleValuesParser`], using a type that implements
/// [`strum::VariantNames`], and [`FromStr`].
#[macro_export]
macro_rules! possible_values {
    ($ty:ty) => {
        PossibleValuesParser::new(<$ty as strum::VariantNames>::VARIANTS)
            .map(|x| <$ty as FromStr>::from_str(&x).unwrap())
    };
}
