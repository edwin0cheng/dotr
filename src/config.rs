use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::storage_path;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub git_url: Option<String>,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub path: String,
    pub hash: String,
}

impl Config {
    pub fn config_path_exists() -> bool {
        let storage_path = match storage_path() {
            Ok(path) => path,
            Err(_) => return false,
        };
        if !storage_path.exists() {
            return false;
        }
        let config_path = storage_path.join(".dotr.toml");
        config_path.exists()
    }

    pub fn config_path() -> Result<PathBuf, anyhow::Error> {
        let storage_path = storage_path()?;
        if !storage_path.exists() {
            eprintln!(
                "The storage path {} is not exist, please run `dotr init` first",
                storage_path.to_string_lossy()
            );
            return Err(anyhow::anyhow!("The storage path is not exist"));
        }
        let config_path = storage_path.join(".dotr.toml");
        if !config_path.exists() {
            eprintln!(
                "The config file {:} is not exist",
                config_path.to_string_lossy()
            );
            return Err(anyhow::anyhow!("The config file is not exist"));
        }

        Ok(config_path)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let config_content = toml::to_string(self)?;
        std::fs::write(Self::config_path()?, config_content)?;
        Ok(())
    }

    pub fn load() -> Result<Config, anyhow::Error> {
        // check if there is a .git folder
        let git_path = storage_path()?.join(".git");
        if !git_path.exists() {
            eprintln!(
                "The git path {} is not exist, please run `dotr init` first",
                git_path.to_string_lossy()
            );
            return Err(anyhow::anyhow!("The git path is not exist"));
        }

        let config_content = std::fs::read_to_string(Self::config_path()?)?;
        let config = toml::from_str(&config_content)?;

        Ok(config)
    }
}
