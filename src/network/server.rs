use std::net::{TcpListener, TcpStream};
use crate::ARGS;
use crate::Error;

pub struct Server {
    listener: TcpListener,
    client: Option<TcpStream>,
}
impl Server {
    pub fn new() -> Result<Self, Error> {
        return Ok(Self {
            listener: listener()?,
            client: None,
        });
    }

    fn accept(&mut self) -> bool {
        if let Ok((sock, addr)) = self.listener.accept() {
            // TODO: confirmation
            log::info!("client {addr:?} connected");
            self.client = Some(sock);
            return true;
        } else {
            return false;
        }
    }
}


pub fn listener() -> Result<TcpListener, std::io::Error> {
    let l = match TcpListener::bind(("0.0.0.0", ARGS.port)) {
        Ok(s) => s,
        Err(e) => {
            log::error!("[server] failed to bind to port {} with error {e:?}", ARGS.port);
            return Err(e);
        }
    };

    return Ok(l);
}
