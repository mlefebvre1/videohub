use crate::protocol;

use super::HubInfo;
use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct Deserializer {
    hub_infos: HubInfo,
}

impl Deserializer {
    pub fn new() -> Self {
        Self {
            hub_infos: Default::default(),
        }
    }
    pub fn deserialize(&mut self, s: &str) -> Result<HubInfo, protocol::error::Error> {
        let mut lines = s.lines();
        loop {
            //Loop until we can't find any block
            if let Some(block_type) = lines.find_map(Self::get_block_type) {
                let block = lines
                    .by_ref()
                    .take_while(|&line| !line.is_empty())
                    .collect::<Vec<&str>>()
                    .join("\n");
                self.deserialize_block(block_type, &block)?;
            } else {
                return Ok(self.hub_infos.clone());
            }
        }
    }

    fn deserialize_block(
        &mut self,
        block_type: protocol::BlockType,
        block: &str,
    ) -> Result<(), protocol::error::Error> {
        match block_type {
            protocol::BlockType::ProtocolPreamble => {
                self.hub_infos.protocol_preamble = Self::deserialize_protocol_preamble(block)?
            }
            protocol::BlockType::DeviceInfo => {
                self.hub_infos.device_info = Self::deserialize_device_info(block)?
            }
            protocol::BlockType::InputLabel => {
                self.hub_infos.input_labels =
                    Self::deserialize_labels(block, self.hub_infos.device_info.nb_video_inputs)?
            }
            protocol::BlockType::OutputLabel => {
                self.hub_infos.output_labels =
                    Self::deserialize_labels(block, self.hub_infos.device_info.nb_video_outputs)?
            }
            protocol::BlockType::VideoOutputLocks => {
                self.hub_infos.video_output_locks = Self::deserialize_output_locks(
                    block,
                    self.hub_infos.device_info.nb_video_outputs,
                )?
            }
            protocol::BlockType::VideoOutputRouting => {
                self.hub_infos.video_output_routing = Self::deserialize_output_routing(
                    block,
                    self.hub_infos.device_info.nb_video_outputs,
                )?
            }
            protocol::BlockType::Configuration => {
                self.hub_infos.configuration = Self::deserialize_configuration(block)?
            }
            protocol::BlockType::EndPrelude => (),
        }
        Ok(())
    }

    fn get_block_type(line: &str) -> Option<protocol::BlockType> {
        match line {
            line if line.starts_with("PROTOCOL PREAMBLE:") => {
                Some(protocol::BlockType::ProtocolPreamble)
            }
            line if line.starts_with("VIDEOHUB DEVICE:") => Some(protocol::BlockType::DeviceInfo),
            line if line.starts_with("INPUT LABELS:") => Some(protocol::BlockType::InputLabel),
            line if line.starts_with("OUTPUT LABELS:") => Some(protocol::BlockType::OutputLabel),
            line if line.starts_with("VIDEO OUTPUT LOCKS:") => {
                Some(protocol::BlockType::VideoOutputLocks)
            }
            line if line.starts_with("VIDEO OUTPUT ROUTING:") => {
                Some(protocol::BlockType::VideoOutputRouting)
            }
            line if line.starts_with("CONFIGURATION:") => Some(protocol::BlockType::Configuration),
            line if line.starts_with("END PRELUDE:") => Some(protocol::BlockType::EndPrelude),
            _ => None,
        }
    }

    fn deserialize_protocol_preamble(
        block: &str,
    ) -> Result<protocol::ProtocolPreamble, protocol::error::Error> {
        let mut protocol_preamble: protocol::ProtocolPreamble = Default::default();
        for line in block.lines() {
            match Self::get_key_and_value_from_line(line)? {
                (key, value) if &key == "Version" => {
                    protocol_preamble.version = value;
                }
                _ => (),
            }
        }
        Ok(protocol_preamble)
    }

    fn deserialize_device_info(
        block: &str,
    ) -> Result<protocol::DeviceInfo, protocol::error::Error> {
        // Parse and extract info for the block VIDEOHUB DEVICE
        // It is expected that VIDEOHUB DEVICE: label is removed
        let mut device_info: protocol::DeviceInfo = Default::default();
        for line in block.lines() {
            match Self::get_key_and_value_from_line(line)? {
                (key, value) if &key == "Device present" => {
                    device_info.device_present = protocol::DevicePresent::from_str(&value)?
                }
                (key, value) if &key == "Model name" => {
                    device_info.model_name = value;
                }
                (key, value) if &key == "Friendly name" => {
                    device_info.friendly_name = value;
                }
                (key, value) if &key == "Unique ID" => {
                    device_info.unique_id = value;
                }
                (key, value) if &key == "Video inputs" => {
                    device_info.nb_video_inputs = value
                        .parse::<usize>()
                        .map_err(protocol::error::Error::ParseInt)?;
                }
                (key, value) if &key == "Video processing units" => {
                    device_info.nb_video_processing_units = value
                        .parse::<usize>()
                        .map_err(protocol::error::Error::ParseInt)?;
                }
                (key, value) if &key == "Video outputs" => {
                    device_info.nb_video_outputs = value
                        .parse::<usize>()
                        .map_err(protocol::error::Error::ParseInt)?;
                }
                (key, value) if &key == "Video monitoring outputs" => {
                    device_info.nb_video_monitoring_outputs = value
                        .parse::<usize>()
                        .map_err(protocol::error::Error::ParseInt)?;
                }
                (key, value) if &key == "Serial ports" => {
                    device_info.nb_serial_ports = value
                        .parse::<usize>()
                        .map_err(protocol::error::Error::ParseInt)?;
                }
                _ => (),
            }
        }
        Ok(device_info)
    }

    fn deserialize_labels(
        block: &str,
        expected_size: usize,
    ) -> Result<Vec<protocol::Label>, protocol::error::Error> {
        let seq = Self::deserialize_seq(block, expected_size)?;
        Ok(seq
            .iter()
            .map(|(id, text)| protocol::Label {
                id: *id,
                text: text.to_string(),
            })
            .collect())
    }

    fn deserialize_output_locks(
        block: &str,
        expected_size: usize,
    ) -> Result<protocol::OutputLocks, protocol::error::Error> {
        let seq = Self::deserialize_seq(block, expected_size)?;
        Ok(seq
            .iter()
            .map(|(id, lock_status)| protocol::OutputLock {
                id: *id,
                lock_status: protocol::LockStatus::from_str(lock_status).unwrap(),
            })
            .collect())
    }

    fn deserialize_output_routing(
        block: &str,
        expected_size: usize,
    ) -> Result<protocol::OutputRoutings, protocol::error::Error> {
        let seq = Self::deserialize_seq(block, expected_size)?;
        let output_routing: protocol::OutputRoutings = seq
            .iter()
            .map(|(output_index, input_index_str)| protocol::Route {
                source: input_index_str.parse::<usize>().unwrap() + 1,
                destination: *output_index,
            })
            .collect();
        Ok(output_routing)
    }

    fn deserialize_configuration(
        block: &str,
    ) -> Result<protocol::Configuration, protocol::error::Error> {
        let mut configuration = protocol::Configuration { take_mode: false };
        for line in block.lines() {
            match Self::get_key_and_value_from_line(line)? {
                (key, value) if &key == "Take Mode" => {
                    configuration.take_mode = value
                        .parse::<bool>()
                        .map_err(protocol::error::Error::ParseBool)?
                }
                _ => (),
            }
        }
        Ok(configuration)
    }

    fn deserialize_seq(
        block: &str,
        expected_size: usize,
    ) -> Result<Vec<(usize, String)>, protocol::error::Error> {
        let mut seq = Vec::with_capacity(expected_size);

        for line in block.lines() {
            let (index, value) = Self::get_index_and_value_from_line(line)?;
            seq.push((index + 1, value.trim().to_string()));
        }
        Ok(seq)
    }

    fn get_key_and_value_from_line(line: &str) -> Result<(String, String), protocol::error::Error> {
        let mut item = line.split(':');
        let key = item.next();
        let value = item.next();
        if let (Some(key), Some(value)) = (key, value) {
            Ok((key.to_string(), value.trim().to_string()))
        } else {
            Err(protocol::error::Error::ParseValueError)
        }
    }

    fn get_index_and_value_from_line(
        line: &str,
    ) -> Result<(usize, String), protocol::error::Error> {
        let mut chars = line.chars();
        let index = chars
            .by_ref()
            .take_while(|c| !c.is_ascii_whitespace())
            .collect::<String>()
            .parse::<usize>()
            .map_err(protocol::error::Error::ParseInt)?;
        let value: String = chars.collect();
        Ok((index, value))
    }
}

#[test]
fn test_deserialize_device_info() {
    let block = "\
        Device present: true\n\
        Model name: Smart Videohub 12G 40x40\n\
        Friendly name: SDI Router G-A - Smart Videohub 12G 40 x 40\n\
        Unique ID: XXXXXX\n\
        Video inputs: 40\n\
        Video processing units: 0\n\
        Video outputs: 40\n\
        Video monitoring outputs: 0\n\
        Serial ports: 0\n\
        ";
    let expected = protocol::DeviceInfo {
        device_present: protocol::DevicePresent::Present,
        model_name: "Smart Videohub 12G 40x40".to_string(),
        friendly_name: "SDI Router G-A - Smart Videohub 12G 40 x 40".to_string(),
        unique_id: "XXXXXX".to_string(),
        nb_video_inputs: 40,
        nb_video_processing_units: 0,
        nb_video_outputs: 40,
        nb_video_monitoring_outputs: 0,
        nb_serial_ports: 0,
    };

    let device_info = Deserializer::deserialize_device_info(block).unwrap();

    assert!(device_info == expected)
}

#[test]
fn test_deserialize_labels() {
    let block = "\
        0 from_RTR_B\n\
        1 BNC Patch RD1-C - 2\n\
        2 BNC Patch RD1-C - 3\n\
        ";
    let expected = vec![
        protocol::Label {
            id: 1,
            text: "from_RTR_B".to_string(),
        },
        protocol::Label {
            id: 2,
            text: "BNC Patch RD1-C - 2".to_string(),
        },
        protocol::Label {
            id: 3,
            text: "BNC Patch RD1-C - 3".to_string(),
        },
    ];

    let input_labels = Deserializer::deserialize_labels(block, 3).unwrap();
    assert!(input_labels == expected);
}

#[test]
fn test_deserialize_output_locks() {
    let block = "\
        0 U\n\
        1 O\n\
        2 F\n\
        3 L\n\
        ";
    let expected = vec![
        protocol::OutputLock {
            id: 1,
            lock_status: protocol::LockStatus::Unlocked,
        },
        protocol::OutputLock {
            id: 2,
            lock_status: protocol::LockStatus::Owned,
        },
        protocol::OutputLock {
            id: 3,
            lock_status: protocol::LockStatus::ForceUnlock,
        },
        protocol::OutputLock {
            id: 4,
            lock_status: protocol::LockStatus::Locked,
        },
    ];
    let input_labels = Deserializer::deserialize_output_locks(block, 4).unwrap();
    assert!(input_labels == expected);
}

#[test]
fn test_deserialize_output_routing() {
    let block = "\
                        0 39\n\
                        1 1\n\
                        6 3\n\
                        0 14\n\
                        14 15\n\
                        31 19\n\
                        32 39\n\
                        ";
    let expected = vec![
        protocol::Route {
            source: 40,
            destination: 1,
        },
        protocol::Route {
            source: 2,
            destination: 2,
        },
        protocol::Route {
            source: 4,
            destination: 7,
        },
        protocol::Route {
            source: 15,
            destination: 1,
        },
        protocol::Route {
            source: 16,
            destination: 15,
        },
        protocol::Route {
            source: 20,
            destination: 32,
        },
        protocol::Route {
            source: 40,
            destination: 33,
        },
    ];
    let actual = Deserializer::deserialize_output_routing(block, 7).unwrap();
    assert!(actual == expected);
}

#[test]
fn test_deserialize_configuration() {
    let block = "\
        Take Mode: true\n\
        ";
    let expected = protocol::Configuration { take_mode: true };

    let acutal = Deserializer::deserialize_configuration(block).unwrap();
    assert!(acutal == expected);
}
