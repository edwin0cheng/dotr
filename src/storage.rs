use std::path::{Path, PathBuf};

use crate::{
    config::{Config, File},
    storage_path,
};
use anyhow::Result;
use sha2::{Digest, Sha256};

pub struct Storage {
    storage_dir: PathBuf,
    base_dir: PathBuf,
    config: Config,
}

impl Storage {
    pub fn new(config: Config) -> Result<Self> {
        let home_dir = dirs::home_dir().expect("Cannot get home dir");

        Self::new_with_storage_dir(config, storage_path()?, home_dir)
    }

    fn new_with_storage_dir(
        config: Config,
        storage_dir: PathBuf,
        base_dir: PathBuf,
    ) -> Result<Self> {
        if !storage_dir.exists() {
            eprintln!(
                "The storage path {} is not exist, please run `dotr init` first",
                storage_dir.to_string_lossy()
            );
            return Err(anyhow::anyhow!("The storage path is not exist"));
        }

        Ok(Self {
            storage_dir,
            base_dir,
            config,
        })
    }

    /// Given an absolute path, return relative to the storage dir
    fn relative(&self, abs_path: &Path) -> PathBuf {
        // Compute the relative path respect to the base dir
        let relative_path = match abs_path.strip_prefix(&self.base_dir) {
            Ok(path) => path,
            Err(_) => {
                panic!(
                    "The file {:} is not in the home dir {:}",
                    abs_path.to_string_lossy(),
                    self.base_dir.to_string_lossy()
                );
            }
        };

        relative_path.to_owned()
    }

    fn absolute(&self, file: &File) -> PathBuf {
        self.base_dir.join(&file.path)
    }

    pub fn add(&mut self, path: &str) -> Result<()> {
        println!("Adding {}...", path);

        let abs_path = Path::new(path).canonicalize()?;
        if !abs_path.exists() {
            eprintln!("The file {} is not exist", path);
            return Err(anyhow::anyhow!("The {:} is not exist", path));
        }

        if abs_path.is_dir() {
            eprintln!("Directory support is not implemented yet!");
            return Err(anyhow::anyhow!("The {:} is a directory", path));
        }

        let relative = self.relative(&abs_path);
        let file_path_str = relative.to_string_lossy();

        if self.config.files.iter().any(|x| x.path == file_path_str) {
            eprintln!("The file {file_path_str} is already added");
            return Err(anyhow::anyhow!("The file is already added"));
        }

        let file = File {
            hash: format!("{:}", hash_file(&abs_path)?),
            path: self.relative(&abs_path).to_string_lossy().to_string(),
        };
        self.config.files.push(file);
        self.config.save()?;

        Ok(())
    }

    fn remove(&mut self, file: &File) -> Result<()> {
        let file_path = self.storage_dir.join(&file.path);
        if !file_path.exists() {
            panic!(
                "The file {:} is not exist in the storage dir {:}",
                file_path.to_string_lossy(),
                self.storage_dir.to_string_lossy()
            );
        }
        std::fs::remove_file(file_path)?;
        self.config.files.retain(|x| x.path != file.path);
        self.config.save()?;

        Ok(())
    }

    pub fn sync_from(&mut self, file: &File) -> Result<()> {
        let origin = self.absolute(file);

        let original_file_path = std::path::Path::new(&origin);
        if !original_file_path.exists() {
            return self.remove(&file);
        }

        let storage_path = self.storage_dir.join(&file.path);
        let mut hash = None;

        if storage_path.exists() {
            let h = hash_file(&original_file_path)?;
            if h == file.hash {
                return Ok(());
            }
            hash = Some(h);
        }

        // Copy the file to the storage dir
        // create directory if not exist
        if let Some(parent) = storage_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        println!(
            "Copying {:} to {:}",
            origin.to_string_lossy(),
            storage_path.to_string_lossy()
        );
        std::fs::copy(&origin, &storage_path)?;

        // Update the hash
        let hash = match hash {
            Some(h) => h,
            None => hash_file(&original_file_path)?,
        };

        self.config
            .files
            .iter_mut()
            .find(|x| x.path == file.path)
            .expect("The file should exist")
            .hash = hash;

        Ok(())
    }

    pub fn sync_to(&self, file: &File) -> Result<()> {
        let storage_path = self.storage_dir.join(&file.path);
        if !storage_path.exists() {
            panic!(
                "The file {:} is not exist in the storage dir {:}",
                storage_path.to_string_lossy(),
                self.storage_dir.to_string_lossy()
            );
        }

        let original_file_path = std::path::Path::new(&self.base_dir).join(&file.path);

        if original_file_path.exists() {
            let h = hash_file(&original_file_path)?;
            if h == file.hash {
                return Ok(());
            }
        } else {
            // check if the file is in the dotrignore file
            let dotrignore_path = self.storage_dir.join(".dotrignore");
            let mut dotrignore_content = String::new();

            if dotrignore_path.exists() {
                dotrignore_content = std::fs::read_to_string(&dotrignore_path)?;
                if dotrignore_content.contains(&file.path) {
                    return Ok(());
                }
            }

            // Ask the user if they want to create the file
            println!(
                "The file {:} is not exist, do you want to create it? [y/N]",
                original_file_path.to_string_lossy()
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim() != "y" {
                // Save it to the dotrignore file
                dotrignore_content.push_str(&format!("\n{}", file.path));
                std::fs::write(&dotrignore_path, dotrignore_content)?;

                return Ok(());
            }
        }

        // Copy the file to the storage dir
        // create directory if not exist
        if let Some(parent) = original_file_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        println!(
            "Copying {:} to {:}",
            storage_path.to_string_lossy(),
            original_file_path.to_string_lossy()
        );
        std::fs::copy(&storage_path, &original_file_path)?;

        Ok(())
    }
}

fn hash_file(path: &Path) -> Result<String> {
    let content = std::fs::read(path)?;

    // FIXME: RA cannot infer the type of hasher
    // https://github.com/RustCrypto/hashes/issues/529
    let mut hasher = <Sha256 as Digest>::new();

    // write input message
    hasher.update(content);

    // read hash digest and consume hasher
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}
