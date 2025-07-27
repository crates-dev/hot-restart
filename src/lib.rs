//! hot-restart
//!
//! A hot reboot lib project.

pub(crate) mod error;
pub(crate) mod hot_restart;

pub use error::r#enum::*;
pub use hot_restart::{r#fn::*, r#type::*};

pub(crate) use std::{
    borrow::Cow,
    fmt,
    io::Error,
    process::{Child, Command, ExitStatus, Output, Stdio, exit},
};
