pub enum HotRestartError {
    CargoWatchNotInstalled,
    CommandSpawnFailed(String),
    CommandWaitFailed(String),
    CommandFailed(Option<i32>),
    Other(String),
}
