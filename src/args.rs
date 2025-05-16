use clap::{Parser, Subcommand};
use std::{net::Ipv4Addr, path::PathBuf, sync::LazyLock};

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);
pub static CWD: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(std::env::args().next().unwrap()));

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
    pub path: PathBuf,
}

#[derive(clap::Args, Debug, Clone)]
pub struct ReceiveArgs {
    /// Server address
    #[arg(value_parser = crate::network::phrase::arg_to_ipv4)]
    pub server_addr: Ipv4Addr,

    /// Where to save the received file
    #[arg(short)]
    pub output_path: Option<PathBuf>,
}
