use crate::*;

impl fmt::Display for HotRestartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HotRestartError::CargoWatchNotInstalled => write!(
                f,
                "Cargo-watch is not installed. Please install it using: cargo install cargo-watch"
            ),
            HotRestartError::CommandSpawnFailed(e) => {
                write!(f, "Failed to spawn cargo watch command: {}", e)
            }
            HotRestartError::CommandWaitFailed(e) => {
                write!(f, "Failed to wait for cargo watch command: {}", e)
            }
            HotRestartError::CommandFailed(code) => {
                write!(f, "cargo watch command failed with status: {:?}", code)
            }
            HotRestartError::Other(e) => write!(f, "An unexpected error occurred: {}", e),
        }
    }
}

impl From<Error> for HotRestartError {
    fn from(err: Error) -> Self {
        HotRestartError::Other(err.to_string())
    }
}
