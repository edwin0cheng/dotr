use crate::storage_path;

fn do_exec_process(
    command: &str,
    args: &[&str],
    allow_empty_error: bool,
) -> Result<String, anyhow::Error> {
    let output = std::process::Command::new(command)
        .args(args)
        .current_dir(storage_path()?)
        .output()
        .map_err(|err| anyhow::anyhow!("failed to execute command: {}", err))?;

    if !output.status.success() {
        if allow_empty_error && String::from_utf8_lossy(&output.stderr).trim() == "" {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        } else {
            eprintln!("command failed");

            return Err(anyhow::anyhow!(
                "command failed {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn exec_process_allow_empty_error(
    command: &str,
    args: &[&str],
) -> Result<String, anyhow::Error> {
    do_exec_process(command, args, true)
}

pub fn exec_process(command: &str, args: &[&str]) -> Result<String, anyhow::Error> {
    do_exec_process(command, args, false)
}

pub fn check_git_user() -> Result<(), anyhow::Error> {
    // Make sure the user.name and user.email is set
    let output = exec_process_allow_empty_error("git", &["config", "user.name"])?;
    if output.is_empty() {
        // ask the user input the user.name
        let mut user_name = String::new();
        println!("git config user.name is empty. Please input the user.name: ");
        std::io::stdin().read_line(&mut user_name)?;
        let user_name = user_name.trim();
        exec_process("git", &["config", "user.name", user_name])?;
    }

    let output = exec_process_allow_empty_error("git", &["config", "user.email"])?;
    if output.is_empty() {
        // ask the user input the user.email
        let mut user_email = String::new();
        println!("git config user.email is empty. Please input the user.email: ");
        std::io::stdin().read_line(&mut user_email)?;
        let user_email = user_email.trim();
        exec_process("git", &["config", "user.email", user_email])?;
    }

    Ok(())
}
