use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
pub mod protocol;

use std::{io::Read, net::TcpStream};

use protocol::HubInfo;

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

    pub fn get_hub_info(&self, content: &str) -> Result<HubInfo, HubError> {
        let mut de = protocol::de::Deserializer::new();
        let hub_info = de.deserialize(content)?;
        Ok(hub_info)
    }
}
