use crate::commands::{GreetCmd, WorldCmd};

use super::get_styles;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "crustcli", styles = get_styles(), version, about)]
pub struct Args {
    /// Increase output verbosity.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors and warnings.
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Run in dry-run mode. Commands will be printed but not executed.
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Accepts all interactive prompts.
    #[arg(short = 'y', long, global = true)]
    pub accept_all: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Basic greet command example.
    #[command(visible_alias = "g")]
    Greet(GreetCmd),
    /// Basic subcommand example.
    #[command(visible_alias = "h")]
    Hello {
        #[command(subcommand)]
        command: HelloSubcmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum HelloSubcmd {
    /// Basic hello world command inside subcommand.
    #[command(visible_alias = "w")]
    World(WorldCmd),
}
