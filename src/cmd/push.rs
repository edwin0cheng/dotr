use clap::Args;

use crate::{
    storage::Storage,
    utils::{check_git_user, exec_process},
};

#[derive(Args)]
pub struct PushArgs {}

pub fn do_command(args: &PushArgs) -> Result<(), anyhow::Error> {
    let PushArgs {} = args;

    let config = crate::config::Config::load()?;
    let files = config.files.clone();
    let mut storage = Storage::new(config)?;

    for file in files {
        storage.sync_from(&file)?;
    }

    // execute git status
    let output = exec_process("git", &["status", "--porcelain=v2"])?;
    if output == "" {
        println!("No changes to commit");
        return Ok(());
    }

    let output = exec_process("git", &["add", "-A"])?;
    if output != "" {
        print!("git: {}", output);
    }

    check_git_user()?;

    let commit_msg = "Update files";
    let output = exec_process("git", &["commit", "-m", commit_msg])?;
    if output != "" {
        print!("git: {}", output);
    }

    let output = exec_process("git", &["push"])?;
    if output != "" {
        print!("git: {}", output);
    }

    Ok(())
}
