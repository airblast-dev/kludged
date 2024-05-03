use kludged::keyboards::{
    rk68::{ColorOptions, Rk68},
    Keyboard, KeyboardColorOption, KeyboardColorable,
};

use clap::{ArgMatches, Args, Command, FromArgMatches};
use palette::Srgb;

use super::commons::color_arg;

pub fn rk68_command(cmd: Command) -> Command {
    let rk68_cmd = ColorOptions::augment_args(Command::new("set-color").arg(color_arg()));
    let rk68_cmd = Command::new("rk68")
        .subcommand(rk68_cmd)
        .subcommand_required(true);
    cmd.subcommand(rk68_cmd)
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
        _ => todo!(),
    }
}
