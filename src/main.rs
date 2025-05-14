use vortex::args::ARGS;
use vortex::args::Commands;

fn main() {
    vortex::init_logger();

    match ARGS.cmd {
        Commands::Send { path: _ } => send(),
        Commands::Receive { } => receive(),
    }
}

fn send() {
    use vortex::network::server::Server;
    let mut server = Server::new().unwrap();
    server.main();
}
