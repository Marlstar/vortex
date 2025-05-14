use vortex::args::ARGS;
use vortex::args::Commands;

fn main() {
    stderrlog::new().module(module_path!()).init().expect("failed to initialise logger");

    match ARGS.cmd {
        Commands::Send { path: _ } => send(),
        Commands::Receive { } => receive(),
    }
}

fn send() {
    use vortex::network::server::Server;
    let mut server = Server::new();
}
