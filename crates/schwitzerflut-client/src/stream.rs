use crate::command_generator::CommandGenerator;
use image::codecs::png::CompressionType::Default;
use std::io;
use std::io::Bytes;
use std::marker::PhantomData;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub struct Disconnected;
pub struct Connected;

pub struct StreamWrapper<S> {
    addr: SocketAddr,
    stream: Option<Arc<RwLock<TcpStream>>>,
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

    pub async fn connect(self) -> io::Result<StreamWrapper<Connected>> {
        let stream = TcpStream::connect(self.addr).await?;

        Ok(StreamWrapper {
            addr: self.addr,
            stream: Some(Arc::new(RwLock::new(stream))),
            _state: PhantomData,
        })
    }
}

impl StreamWrapper<Connected> {
    pub async fn send(&self, payload: impl AsRef<[u8]>) -> io::Result<usize> {
        let stream_ref = self.stream.clone().expect("stream always exist in ");
        let mut stream_guard = stream_ref.write().await;

        stream_guard.write(payload.as_ref()).await
    }
}
