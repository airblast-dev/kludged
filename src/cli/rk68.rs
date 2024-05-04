use crate::keyboards::{
    rk68::{Animation, AnimationOptions, ColorOptions, Rk68},
    Keyboard, KeyboardAnimatable, KeyboardAnimationOption, KeyboardColorOption, KeyboardColorable,
};

use clap::{ArgMatches, Args, Command, FromArgMatches};
use palette::Srgb;

use super::commons::{anim_arg, color_arg};

/// Construct keyboard subcommand(s).
pub fn command(cmd: Command) -> Command {
    let rk68_cmd = Command::new("rk68")
        .subcommands(single_kb_command())
        .subcommand_required(true);
    cmd.subcommand(rk68_cmd)
}

/// Construct inner keyboard subcommand(s).
pub fn single_kb_command() -> impl IntoIterator<Item = Command> {
    [
        ColorOptions::augment_args(Command::new("set-color").arg(color_arg().required(true))),
        AnimationOptions::augment_args(Command::new("set-anim").arg(anim_arg::<Animation>())),
    ]
}

pub fn handle_args(arg_matches: &ArgMatches) -> anyhow::Result<()> {
    // Ok to unwrap, subcommand is required.
    match arg_matches.subcommand().unwrap() {
        ("set-color", arg_matches) => {
            // Ok to unwrap as we require the argument.
            let color: &Srgb<u8> = arg_matches.get_one("color").unwrap();

            // Ok to unwrap as a default value is used in case the argument was not provided.
            let color_options = ColorOptions::from_arg_matches(arg_matches)?;

            Rk68::new()?
                .set_color(*color)
                .set_color_parameters(color_options)
                .apply_color()?;

            Ok(())
        }
        ("set-anim", arg_matches) => {
            let animation: Animation = *arg_matches.get_one("anim").unwrap();

            let anim_options = AnimationOptions::from_arg_matches(arg_matches)?;

            Rk68::new()?
                .set_animation(animation)
                .set_animation_parameters(anim_options)
                .apply_animation()?;

            Ok(())
        }
        _ => todo!(),
    }
}
