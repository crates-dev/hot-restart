#[test]
fn is_cargo_watch_installed() {
    use crate::*;
    let res: Result<(), HotRestartError> = cargo_watch("install");
    assert!(res.is_ok())
}
