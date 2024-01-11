use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

mod cmd;
mod config;
mod storage;
mod utils;

use cmd::{add, init, pull, push};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init(init::InitArgs),
    Add(add::AddArgs),
    Push(push::PushArgs),
    Pull(pull::PullArgs),
}

const STORAGE_PATH: &str = "~/.local/share/dotr";

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Init(args)) => {
            init::do_command(args)?;
        }
        Some(Commands::Add(args)) => {
            add::do_command(args)?;
        }
        Some(Commands::Push(args)) => {
            push::do_command(args)?;
        }
        Some(Commands::Pull(args)) => {
            pull::do_command(args)?;
        }
        None => Cli::command().print_help()?,
    }

    Ok(())
}

fn storage_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!("Cannot get home dir"))?;
    let storage_path = home_dir.join(STORAGE_PATH.replace("~/", ""));

    Ok(storage_path)
}
