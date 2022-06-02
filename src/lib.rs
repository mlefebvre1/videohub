use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
mod protocol;

use std::{io::Read, net::TcpStream};

pub type HubError = Box<dyn std::error::Error>;
pub type HubResult<T> = Result<T, HubError>;

#[derive(Debug)]
pub struct Hub {
    socket_addr: SocketAddrV4,
}

impl Hub {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        let socket_addr = SocketAddrV4::new(ip, port);
        Self { socket_addr }
    }

    pub fn dump_hub_info(&self) -> Result<String, HubError> {
        let mut stream = TcpStream::connect(&self.socket_addr)?;
        let mut buffer = [0; 4096];
        let mut content = "".to_string();
        loop {
            let nb_bytes = stream.read(&mut buffer)?;
            let partial_content = String::from_utf8(buffer[..nb_bytes].to_vec()).unwrap(); // TODO(check how to return different error type)
            content.push_str(&partial_content);
            if partial_content.contains("END PRELUDE") {
                break;
            }
        }
        Ok(content)
    }

    pub fn split_hub_info_in_blocks(utf8_dump: &str) -> Result<Vec<String>, HubError> {
        let blocks = Vec::new();
        for line in utf8_dump.lines() {}
        Ok(blocks)
    }
}
