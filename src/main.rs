use vortex::args::ARGS;
use vortex::args::Commands;

fn main() {
    vortex::init_logger();

    match &ARGS.cmd {
        Commands::Send(args) => send(args),
        Commands::Receive(args) => receive(args),
    }
}

fn send(args: &vortex::args::SendArgs) {
    use vortex::network::server::Server;
    let mut server = Server::new(args.path.clone()).unwrap();
    server.main();
}

fn receive(args: &vortex::args::ReceiveArgs) {
    use vortex::network::client::Client;
    let mut client = Client::new(args).unwrap();
    client.main();
}
