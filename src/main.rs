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

use clap::CommandFactory;
use cli::rk68;

use crate::cli::Cli;

use anyhow::bail;
use kludged::keyboards::KeyboardModels;

#[cfg(feature = "cli")]
fn main() -> anyhow::Result<()> {
    let mut cmd = Cli::command().subcommand_required(true);
    let keyboards = KeyboardModels::keyboards()?;
    if keyboards
        .iter()
        .any(|kb| matches!(kb, KeyboardModels::Rk68(_)))
    {
        cmd = rk68::rk68_command(cmd)
    }

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some((subcommand_name, arg_matches)) => match subcommand_name {
            "rk68" => rk68::handle_args(arg_matches)?,
            _ => todo!(),
        },
        None => bail!("Subcommand not found."),
    }

    Ok(())
}
