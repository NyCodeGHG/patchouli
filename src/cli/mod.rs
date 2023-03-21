use clap::{Parser, Subcommand};
use color_eyre::Result;

use crate::actions::{apply, init, rebuild};

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        CliCommand::Apply => apply(),
        CliCommand::Rebuild => rebuild(),
        CliCommand::Init => init(),
    }?;
    Ok(())
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: CliCommand,
}

#[derive(Debug, Subcommand, Clone)]
enum CliCommand {
    /// Applies patches
    Apply,
    /// Rebuild the patches from the git repository
    Rebuild,
    /// Initializes a new patchouli project
    Init,
}
