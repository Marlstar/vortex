use std::{io::{Read, Write}, net::{TcpStream, ToSocketAddrs}, path::PathBuf};
use crate::{args::ReceiveArgs, Error, ARGS};

const BUF_CAPACITY: usize = 1024;

pub struct Client {
    socket: TcpStream,
    output_path: PathBuf,
}
impl Client {
    pub fn new(args: &ReceiveArgs) -> Result<Self, Error> {
        return Ok(Self {
            socket: connect((args.server_addr, ARGS.port))?,
            output_path: args.output_path.clone(),
        })
    }

    pub fn main(&mut self) {
        let bytes = self.recv().unwrap();
        let mut file = std::fs::File::create_new(&self.output_path).unwrap();
        file.write_all(&bytes).unwrap();
    }

    pub fn recv(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(BUF_CAPACITY);
        self.socket.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

fn connect(addr: impl ToSocketAddrs) -> Result<TcpStream, Error> {
    Ok(TcpStream::connect(addr)?)
}
