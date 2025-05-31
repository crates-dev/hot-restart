use crate::*;

pub fn run_cargo_watch_command(command_args: &str) -> Result<(), HotRestartError> {
    let check_output: Output = Command::new("cargo")
        .args(&["install", "--list"])
        .output()
        .map_err(|e| HotRestartError::Other(e.to_string()))?;
    let check_output_str: Cow<'_, str> = String::from_utf8_lossy(&check_output.stdout);
    if !check_output_str.contains("cargo-watch") {
        return Err(HotRestartError::CargoWatchNotInstalled);
    }
    let mut command: Command = Command::new("cargo");
    command.arg("watch");
    let args: Vec<&str> = command_args.split_whitespace().collect();
    command.args(&args);
    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());
    let mut child: Child = command
        .spawn()
        .map_err(|e| HotRestartError::CommandSpawnFailed(e.to_string()))?;
    let status: ExitStatus = child
        .wait()
        .map_err(|e| HotRestartError::CommandWaitFailed(e.to_string()))?;
    if status.success() {
        Ok(())
    } else {
        Err(HotRestartError::CommandFailed(status.code()))
    }
}
