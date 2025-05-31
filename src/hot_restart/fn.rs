use crate::*;

fn run_hot_restart(run_args: &[&str], wait: bool) -> ResultHotRestartError {
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
    command
        .args(run_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    let mut child: Child = command
        .spawn()
        .map_err(|e| HotRestartError::CommandSpawnFailed(e.to_string()))?;
    if wait {
        child
            .wait()
            .map_err(|e| HotRestartError::CommandWaitFailed(e.to_string()))?;
    }
    exit(0);
}

pub fn hot_restart(run_args: &[&str]) -> ResultHotRestartError {
    run_hot_restart(run_args, false)
}

pub fn hot_restart_wait(run_args: &[&str]) -> ResultHotRestartError {
    run_hot_restart(run_args, true)
}
