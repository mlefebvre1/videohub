use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;

use std::net::TcpStream;

use crate::protocol;

use super::protocol::{de, ser, BlockType, HubInfo};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to connect to the videohub")]
    ConnectionError(#[from] std::io::Error),
    #[error("Failed to read bytes from the videohub")]
    ReadError,
    #[error("An error occured during the protocol deserialization")]
    DeserializeError(#[from] protocol::error::Error),
}

#[derive(Debug)]
pub struct Hub {
    socket_addr: SocketAddrV4,
}

pub type Result<T> = std::result::Result<T, Error>;

pub const DEFAULT_DEVICE_PORT: u16 = 9990;

impl Hub {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        let socket_addr = SocketAddrV4::new(ip, port);
        Self { socket_addr }
    }

    pub fn read(&self) -> Result<HubInfo> {
        let mut stream = TcpStream::connect(&self.socket_addr)?;
        let mut buffer = [0; 4096];
        let mut content = "".to_string();
        loop {
            let nb_bytes = stream.read(&mut buffer)?;
            let partial_content = String::from_utf8(buffer[..nb_bytes].to_vec()).unwrap();
            content.push_str(&partial_content);
            if partial_content.contains("END PRELUDE") {
                break;
            }
        }
        Ok(de::from_str(&content)?)
    }

    pub fn write(&self, block: BlockType) -> Result<usize> {
        let block = ser::to_string(&block)?;
        let mut stream = TcpStream::connect(&self.socket_addr)?;
        Ok(stream.write(block.as_bytes())?)
    }
}
