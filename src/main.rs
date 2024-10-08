use kludged::{
    cli::{commons, rk68, Cli, Commands},
    keyboards::KeyboardModels,
    udev::rules,
};

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use clap::{Command, CommandFactory, Parser};

use anyhow::{bail, Result};
use color_print::cstr;

fn main() -> Result<()> {
    let cmd = Cli::command().subcommand_required(true);

    // Before passing the arguments to the keyboard handlers, we check if a non keyboard modifying
    // command was used.
    if let Ok(cli) = Cli::try_parse() {
        if let Some(Commands::Udev { path }) = cli.command {
            return handle_udev(&path);
        }
    };

    handle_kb(cmd)?;

    Ok(())
}

fn handle_kb(cmd: Command) -> Result<()> {
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
        cmd = match kb {
            KeyboardModels::Rk68(_) => rk68::command(cmd),
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

fn handle_udev(path: &Path) -> Result<()> {
    let mut buf = {
        let file = if !path.exists() {
            File::create_new(path)?
        } else {
            File::options().write(true).truncate(true).open(path)?
        };

        BufWriter::new(file)
    };

    rules(&mut buf)?;

    buf.flush()?;

    Ok(())
}
