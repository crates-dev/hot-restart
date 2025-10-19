use crate::*;

/// Implementation of Display trait for HotRestartError.
impl fmt::Display for HotRestartError {
    /// Formats the error message for display.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The HotRestartError instance.
    /// - `&mut fmt::Formatter` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Result of formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HotRestartError::CargoWatchNotInstalled => write!(
                f,
                "Cargo-watch is not installed. Please install it using: cargo install cargo-watch"
            ),
            HotRestartError::CommandSpawnFailed(e) => {
                write!(f, "Failed to spawn cargo watch command: {e}")
            }
            HotRestartError::CommandWaitFailed(e) => {
                write!(f, "Failed to wait for cargo watch command: {e}")
            }
            HotRestartError::Other(e) => write!(f, "An unexpected error occurred: {e}"),
        }
    }
}

/// Implementation of From trait for converting Error to HotRestartError.
impl From<Error> for HotRestartError {
    /// Converts a generic Error into HotRestartError.
    ///
    /// # Arguments
    ///
    /// - `Error` - The source error to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - The converted error.
    fn from(err: Error) -> Self {
        HotRestartError::Other(err.to_string())
    }
}
