#[cfg(not(feature = "cli"))]
fn main() {
    // This is due to `cfg_attr` in the library.
    // It isn't possible to implement traits from another crate.
    // Even so, most of the keyboard options are deriveable.
    //
    // In order to practically support the options in the CLI,
    // trait implementations for traits from `clap` are feature gated to remove the dependency from the library side of things.
    compile_error!("The \"cli\" feature must be enabled to compile to binary.");
}

mod cli;

use clap::{Command, CommandFactory};

use cli::rk68;
use cli::{commons, Cli};

use anyhow::{bail, Result};
use kludged::keyboards::KeyboardModels;

#[cfg(feature = "cli")]
fn main() -> Result<()> {
    use color_print::cstr;

    let cmd = Cli::command().subcommand_required(true);
    let keyboards = KeyboardModels::keyboards()?;

    if keyboards.is_empty() {
        bail!(cstr!("<yellow>Unable to detect a supported keyboard. If you think this is incorrect please create an issue.</yellow>"))
    }

    // If a single keyboard is found, there is no need to specify the device.
    // Instead, we just provide subcommands for the detected keyboard.
    if keyboards.len() == 1 {
        return handle_single_kb(cmd, &keyboards[0]);
    }

    handle_multiple_kb(cmd, keyboards)
}

fn handle_single_kb(mut cmd: Command, kb: &KeyboardModels) -> Result<()> {
    match kb {
        KeyboardModels::Rk68(_) => {
            cmd = cmd
                .after_help(commons::show_keyboard("RK68"))
                .subcommands(rk68::single_kb_command());
            rk68::handle_args(&cmd.get_matches())?;
        }
    }

    Ok(())
}

fn handle_multiple_kb(mut cmd: Command, keyboards: Vec<KeyboardModels>) -> Result<()> {
    // Add subcommands, and their arguments to the command.
    for kb in keyboards.into_iter() {
        match kb {
            KeyboardModels::Rk68(_) => cmd = rk68::command(cmd),
        }
    }

    match cmd.get_matches().subcommand() {
        Some((subcommand_name, arg_matches)) => match subcommand_name {
            "rk68" => rk68::handle_args(arg_matches)?,
            _ => todo!(),
        },
        None => bail!("Subcommand not found."),
    }

    Ok(())
}
