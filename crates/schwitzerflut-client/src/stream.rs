use crate::command_generator::CommandGenerator;
use clap::builder::Str;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::{Arc, RwLock};

pub struct Disconnected;
pub struct Connected;

pub struct StreamWrapper<S> {
    addr: SocketAddr,
    stream: Option<TcpStream>,
    _state: PhantomData<S>,
}

impl StreamWrapper<Disconnected> {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            stream: None,
            _state: PhantomData,
        }
    }

    pub fn connect(self) -> io::Result<StreamWrapper<Connected>> {
        let stream = TcpStream::connect(self.addr)?;

        Ok(StreamWrapper {
            addr: self.addr,
            stream: Some(stream),
            _state: PhantomData,
        })
    }
}

impl StreamWrapper<Connected> {
    pub fn send(mut self, payload: impl AsRef<[u8]>) {
        let mut connection = self.stream.unwrap();

        loop {
            if let Err(e) = connection.write_all(payload.as_ref()) {
                eprintln!("error sending: {}", e);

                break;
            }
        }
    }
}
