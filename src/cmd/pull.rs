use clap::Args;

use crate::{config::Config, storage::Storage, utils::exec_process};

#[derive(Args)]
pub struct PullArgs {}

pub fn do_command(args: &PullArgs) -> Result<(), anyhow::Error> {
    let PullArgs {} = args;

    // Make sure the setup is done
    let _ = Storage::new(Config::load()?)?;

    let output = exec_process("git", &["pull"])?;
    println!("git: {}", output);

    let config = Config::load()?;
    let files = config.files.clone();

    let storage = Storage::new(config)?;

    for file in files {
        storage.sync_to(&file)?;
    }

    Ok(())
}
