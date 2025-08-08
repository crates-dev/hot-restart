//! hot-restart
//!
//! A Rust library for hot restarting applications without downtime.
//! Provides seamless process replacement for servers and long-running services,
//! enabling zero-downtime updates and configuration reloads.

pub(crate) mod error;
pub(crate) mod hot_restart;

pub use error::r#enum::*;
pub use hot_restart::{r#fn::*, r#type::*};

pub(crate) use std::{
    borrow::Cow,
    fmt,
    future::Future,
    io::Error,
    process::{Child, Command, ExitStatus, Output, Stdio, exit},
};
