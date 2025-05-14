use clap::{Parser, Subcommand};
use std::{path::PathBuf, sync::LazyLock};

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(Parser, Debug)]
#[command(
    name = "vortex",
    author = "Marley Reeves",
    version,
    about = "A file transfer tool written in Rust",
    long_about = None,
)]
pub struct Args {
    /// Mode
    #[command(subcommand)]
    pub cmd: Commands,

    /// TCP port for file transfer
    #[arg(short, long, global = true, default_value_t = 7070)]
    pub port: u16,
}

#[derive(Debug, Clone)]
#[derive(Subcommand)]
pub enum Commands {
    /// Send a file
    Send(SendArgs),

    /// Receive a file
    Receive(ReceiveArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct SendArgs {
    /// Path of the file to send
    path: PathBuf,
}

#[derive(clap::Args, Debug, Clone)]
pub struct ReceiveArgs {
}
