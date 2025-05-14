#![allow(clippy::needless_return)]

pub mod args;
pub use args::ARGS;

mod log;
pub use log::init_logger;

mod error;
pub use error::Error;

pub mod network;
