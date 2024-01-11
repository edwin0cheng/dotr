use clap::Args;

use crate::{config::Config, storage_path, utils::exec_process};

#[derive(Args)]
pub struct InitArgs {
    #[arg(short, long)]
    force_recreate: bool,

    git_url: String,
}

pub fn do_command(args: &InitArgs) -> Result<(), anyhow::Error> {
    let InitArgs {
        force_recreate,
        git_url,
    } = args;

    let storage_path = storage_path()?;
    if storage_path.exists() {
        if *force_recreate {
            eprintln!(
                "The storage path {} is already exist, but force recreate",
                storage_path.to_string_lossy()
            );
            std::fs::remove_dir_all(&storage_path)?;
        } else {
            eprintln!(
                "The storage path {} is already exist",
                storage_path.to_string_lossy()
            );
            return Err(anyhow::anyhow!("The storage path is already exist"));
        }
    }

    std::fs::create_dir_all(&storage_path)?;
    println!("Cloning from {:} ... ", git_url);
    exec_process("git", &["clone", git_url, "."])?;

    if !Config::config_path_exists() {
        let config_content = toml::to_string(&Config::default())?;
        let config_path = storage_path.join(".dotr.toml");

        // The order here is important, we need to clone first, then write the config file
        std::fs::write(config_path, config_content)?;

        // Write the .gitignore
        std::fs::write(storage_path.join(".gitignore"), ".dotrignore\n")?;
    }

    Ok(())
}
