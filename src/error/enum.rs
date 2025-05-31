pub enum HotRestartError {
    CargoWatchNotInstalled,
    CommandSpawnFailed(String),
    CommandWaitFailed(String),
    Other(String),
}
