use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
pub mod protocol;

use std::net::TcpStream;

use protocol::{HubInfo, WriteType};

pub type HubError = Box<dyn std::error::Error>;
pub type HubResult<T> = Result<T, HubError>;

pub const DEFAULT_DEVICE_PORT: u16 = 9990;

#[derive(Debug)]
pub struct Hub {
    socket_addr: SocketAddrV4,
}

impl Hub {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        let socket_addr = SocketAddrV4::new(ip, port);
        Self { socket_addr }
    }

    pub fn read(&self) -> Result<HubInfo, HubError> {
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
        let mut de = protocol::de::Deserializer::new();
        let hub_info = de.deserialize(&content)?;
        Ok(hub_info)
    }

    pub fn write(&self, data: WriteType) -> Result<usize, HubError> {
        let ser = protocol::ser::Serializer::new();

        let content_to_write = match data {
            WriteType::VideoOutputRouting(output_routes) => {
                { ser.serialize_video_output_routes(&output_routes) }?
            }
            WriteType::OutputLabel(labels) => ser.serialize_output_labels(labels)?,
            WriteType::InputLabel(labels) => ser.serialize_input_labels(labels)?,
            WriteType::VideoOutputLocks(output_locks) => {
                ser.serialize_output_locks(output_locks)?
            }
        };

        let mut stream = TcpStream::connect(&self.socket_addr)?;
        Ok(stream.write(content_to_write.as_bytes())?)
    }
}
