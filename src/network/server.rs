use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use crate::ARGS;
use crate::Error;

pub struct Server {
    listener: TcpListener,
    client: Option<TcpStream>,
    path: PathBuf,
}
impl Server {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        return Ok(Self {
            listener: listener()?,
            client: None,
            path,
        });
    }
}

impl Server {
    pub fn main(&mut self) {
        let path = self.path.clone();
        let bytes_handle = std::thread::spawn(move || {
            let mut buf = Vec::with_capacity(1024);
            std::fs::File::open(path).unwrap().read_to_end(&mut buf).unwrap();
            buf
        });

        loop {
            if self.accept() { break; }
        }

        self.client.as_mut().unwrap().write_all(&bytes_handle.join().unwrap()).unwrap();
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
