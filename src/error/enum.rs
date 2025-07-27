/// Errors that can occur during hot restart process.
pub enum HotRestartError {
    /// Indicates cargo-watch is not installed.
    CargoWatchNotInstalled,
    /// Failed to spawn command process.
    CommandSpawnFailed(String),
    /// Failed to wait for command process completion.
    CommandWaitFailed(String),
    /// Other unspecified error with message.
    Other(String),
}
