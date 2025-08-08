<center>

## hot-restart

[![](https://img.shields.io/crates/v/hot-restart.svg)](https://crates.io/crates/hot-restart)
[![](https://img.shields.io/crates/d/hot-restart.svg)](https://img.shields.io/crates/d/hot-restart.svg)
[![](https://docs.rs/hot-restart/badge.svg)](https://docs.rs/hot-restart)
[![](https://github.com/crates-dev/hot-restart/workflows/Rust/badge.svg)](https://github.com/crates-dev/hot-restart/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hot-restart.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hot-restart/)

[Api Docs](https://docs.rs/hot-restart/latest/hot_restart/)

> A Rust library for hot restarting applications without downtime. Provides seamless process replacement for servers and long-running services, enabling zero-downtime updates and configuration reloads.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hot-restart
```

## Use

```rust
use hot_restart::*;

async fn before_restart_hook() {}

#[tokio::main]
async fn main() {
    let res = hot_restart(
        &["--once", "-x", "check", "-x", "build", "--release"],
        before_restart_hook(),
    )
    .await;
    println!("hot_restart result: {:?}", res);
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
