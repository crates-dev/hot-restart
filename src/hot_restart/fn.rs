use crate::*;

pub fn hot_restart(command_args: &str) -> Result<(), HotRestartError> {
    let check_output: Output = Command::new("cargo")
        .args(&["install", "--list"])
        .output()
        .map_err(|e| HotRestartError::Other(e.to_string()))?;
    let check_output_str: Cow<'_, str> = String::from_utf8_lossy(&check_output.stdout);
    if !check_output_str.contains("cargo-watch") {
        eprintln!("Cargo-watch is not installed. Attempting to install...");
        let install_status: ExitStatus = Command::new("cargo")
            .args(&["install", "cargo-watch"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?;
        if !install_status.success() {
            return Err(HotRestartError::CargoWatchNotInstalled);
        }
        eprintln!("Cargo-watch installed successfully.");
    }
    let mut command: Command = Command::new("cargo-watch");
    let args: Vec<&str> = command_args.split_whitespace().collect();
    command
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    command
        .spawn()
        .map_err(|e| HotRestartError::CommandSpawnFailed(e.to_string()))?
        .wait()
        .map_err(|e| HotRestartError::CommandWaitFailed(e.to_string()))?;
    Ok(())
}
