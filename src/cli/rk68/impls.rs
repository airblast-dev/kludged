use std::str::FromStr;

use clap::{
    builder::{PossibleValuesParser, TypedValueParser},
    error::ErrorKind,
    Arg, ArgAction, Args, FromArgMatches,
};
use strum::VariantNames;

use crate::{
    cli::commons::color_arg,
    keyboards::rk68::{Animation, AnimationOptions, Brightness, ColorOptions, Sleep, Speed},
    possible_values,
};

impl Args for ColorOptions {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.arg(
            Arg::new("sleep")
                .short('s')
                .long("sleep")
                .default_value(<Sleep as Into<&'static str>>::into(Sleep::default()))
                .value_parser(PossibleValuesParser::new(Sleep::VARIANTS)),
        )
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        ColorOptions::augment_args(cmd)
    }
}

impl FromArgMatches for ColorOptions {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let sleep: Sleep = Sleep::from_str(matches.get_one::<String>("sleep").unwrap())
            .map_err(|_| clap::Error::new(ErrorKind::InvalidValue))?;

        Ok(ColorOptions { sleep })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;

        Ok(())
    }
}

impl Args for AnimationOptions {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.args([
            Arg::new("mix")
                .short('m')
                .long("color-mix")
                .action(ArgAction::SetTrue)
                .help("Enable color mix.")
                .long_help(
                    "Enable color mix. Can also be used with the color parameter in some cases.",
                ),
            color_arg(),
            Arg::new("sleep")
                .short('s')
                .long("sleep")
                .default_value(<Sleep as Into<&'static str>>::into(Sleep::default()))
                .value_parser(possible_values!(Sleep)),
            Arg::new("speed")
                .long("speed")
                .default_value(<Speed as Into<&'static str>>::into(Speed::default()))
                .value_parser(possible_values!(Speed)),
            Arg::new("anim")
                .short('a')
                .long("anim")
                .value_parser(possible_values!(Animation))
                .required(true),
            Arg::new("brightness")
                .short('b')
                .long("brightness")
                .value_parser(possible_values!(Brightness))
                .default_value(<Brightness as Into<&'static str>>::into(
                    Brightness::default(),
                )),
        ])
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        Self::augment_args(cmd)
    }
}

impl FromArgMatches for AnimationOptions {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(AnimationOptions {
            color: *matches.get_one("color").unwrap(),
            sleep: *matches.get_one("sleep").unwrap(),
            speed: *matches.get_one("speed").unwrap(),
            color_mix: *matches.get_one("mix").unwrap(),
            brightness: *matches.get_one("brightness").unwrap(),
        })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;

        Ok(())
    }
}
