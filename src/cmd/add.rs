use clap::Args;

use crate::{config::Config, storage::Storage};

#[derive(Args)]
pub struct AddArgs {
    path: String,
}

pub fn do_command(args: &AddArgs) -> Result<(), anyhow::Error> {
    let AddArgs { path } = args;

    let config = Config::load()?;

    let mut storage = Storage::new(config)?;

    storage.add(path)?;

    Ok(())
}
