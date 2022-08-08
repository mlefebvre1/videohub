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
        Self::deserialize_seq(block, expected_size)
    }

    fn deserialize_output_locks(
        block: &str,
        expected_size: usize,
    ) -> Result<protocol::OutputLocks, protocol::error::Error> {
        let seq = Self::deserialize_seq(block, expected_size)?;
        let output_locks: Result<protocol::OutputLocks, protocol::error::Error> = seq
            .iter()
            .map(|item| protocol::LockStatus::from_str(item))
            .collect();
        output_locks
    }

    fn deserialize_output_routing(
        block: &str,
        expected_size: usize,
    ) -> Result<protocol::OutputRoutings, protocol::error::Error> {
        let seq = Self::deserialize_seq(block, expected_size)?;
        let output_routing: protocol::OutputRoutings = seq
            .iter()
            .map(|item| item.parse::<usize>().unwrap())
            .enumerate()
            .map(|(output, input)| protocol::Route {
                source: input,
                destination: output,
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
    ) -> Result<Vec<String>, protocol::error::Error> {
        let mut seq = Vec::with_capacity(expected_size);

        seq.resize(expected_size, "".to_string());
        for line in block.lines() {
            let (index, value) = Self::get_index_and_value_from_line(line)?;
            if index < expected_size {
                seq[index] = value.trim().to_string();
            } else {
                return Err(protocol::error::Error::IndexError);
            }
        }
        Ok(seq)
    }

    fn get_key_and_value_from_line(line: &str) -> Result<(String, String), protocol::error::Error> {
        let mut item = line.split(":");
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
        3 mlefebvre-input2\n\
        4 BNC Patch RD1-C - 5\n\
        5 BNC Patch RD1-C - 6\n\
        6 BNC Patch RD1-C - 7\n\
        7 sebas input shelf ci\n\
        8 BNC Patch RD1-C - 9\n\
        9 mlefebvre-input1\n\
        10 BNC Patch RD1-C - 11 (simon)\n\
        11 BNC Patch RD1-C - 12\n\
        12 BNC Patch RD1-C - 13\n\
        13 BNC Patch RD1-C - 14\n\
        14 BNC Patch RD1-C - 15\n\
        15 BNC Patch RD1-C - 16\n\
        16 BNC Patch RD1-C - 17\n\
        17 BNC Patch RD1-C - 18\n\
        18 BNC Patch RD1-C - 19\n\
        19 BNC Patch RD1-C - 20\n\
        20 BNC Patch RD1-C - 21\n\
        21 BNC Patch RD1-C - 22\n\
        22 BNC Patch RD1-C - 23\n\
        23 BNC Patch RD1-C - 24\n\
        24 BNC Patch RD1-C - 25\n\
        25 BNC Patch RD1-C - 26\n\
        26 BNC Patch RD1-C - 27\n\
        27 BNC Patch RD1-C - 28\n\
        28 BNC Patch RD1-C - 29\n\
        29 BNC Patch RD1-C - 30\n\
        30 BNC Patch RD1-C - 31\n\
        31 BNC Patch RD1-C - 32\n\
        32 BNC Patch RD1-B - 9\n\
        33 From_Dynamo_out_1\n\
        34 From_POBOX6_Out_1\n\
        35 From_Rocket_out_1\n\
        36 BNC Patch RD1-B - 13\n\
        37 BNC Patch RD1-B - 14\n\
        38 BNC Patch RD1-B - 15\n\
        39 BNC Patch RD1-B - 16\n\
        ";
    let expected = vec![
        "from_RTR_B".to_string(),
        "BNC Patch RD1-C - 2".to_string(),
        "BNC Patch RD1-C - 3".to_string(),
        "mlefebvre-input2".to_string(),
        "BNC Patch RD1-C - 5".to_string(),
        "BNC Patch RD1-C - 6".to_string(),
        "BNC Patch RD1-C - 7".to_string(),
        "sebas input shelf ci".to_string(),
        "BNC Patch RD1-C - 9".to_string(),
        "mlefebvre-input1".to_string(),
        "BNC Patch RD1-C - 11 (simon)".to_string(),
        "BNC Patch RD1-C - 12".to_string(),
        "BNC Patch RD1-C - 13".to_string(),
        "BNC Patch RD1-C - 14".to_string(),
        "BNC Patch RD1-C - 15".to_string(),
        "BNC Patch RD1-C - 16".to_string(),
        "BNC Patch RD1-C - 17".to_string(),
        "BNC Patch RD1-C - 18".to_string(),
        "BNC Patch RD1-C - 19".to_string(),
        "BNC Patch RD1-C - 20".to_string(),
        "BNC Patch RD1-C - 21".to_string(),
        "BNC Patch RD1-C - 22".to_string(),
        "BNC Patch RD1-C - 23".to_string(),
        "BNC Patch RD1-C - 24".to_string(),
        "BNC Patch RD1-C - 25".to_string(),
        "BNC Patch RD1-C - 26".to_string(),
        "BNC Patch RD1-C - 27".to_string(),
        "BNC Patch RD1-C - 28".to_string(),
        "BNC Patch RD1-C - 29".to_string(),
        "BNC Patch RD1-C - 30".to_string(),
        "BNC Patch RD1-C - 31".to_string(),
        "BNC Patch RD1-C - 32".to_string(),
        "BNC Patch RD1-B - 9".to_string(),
        "From_Dynamo_out_1".to_string(),
        "From_POBOX6_Out_1".to_string(),
        "From_Rocket_out_1".to_string(),
        "BNC Patch RD1-B - 13".to_string(),
        "BNC Patch RD1-B - 14".to_string(),
        "BNC Patch RD1-B - 15".to_string(),
        "BNC Patch RD1-B - 16".to_string(),
    ];

    let input_labels = Deserializer::deserialize_labels(block, 40).unwrap();
    assert!(input_labels == expected);
}

#[test]
fn test_deserialize_output_locks() {
    let block = "\
        0 U\n\
        1 U\n\
        2 U\n\
        3 U\n\
        4 U\n\
        5 U\n\
        6 U\n\
        7 U\n\
        8 U\n\
        9 U\n\
        10 U\n\
        11 F\n\
        12 U\n\
        13 U\n\
        14 O\n\
        15 U\n\
        16 O\n\
        17 U\n\
        18 U\n\
        19 U\n\
        20 U\n\
        21 F\n\
        22 U\n\
        23 U\n\
        24 U\n\
        25 U\n\
        26 U\n\
        27 U\n\
        28 U\n\
        29 U\n\
        30 U\n\
        31 U\n\
        32 U\n\
        33 U\n\
        34 U\n\
        35 U\n\
        36 O\n\
        37 O\n\
        38 O\n\
        39 O\n\
        ";
    let expected = vec![
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::ForceUnlock,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Locked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Locked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::ForceUnlock,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Unlocked,
        protocol::LockStatus::Locked,
        protocol::LockStatus::Locked,
        protocol::LockStatus::Locked,
        protocol::LockStatus::Locked,
    ];
    let input_labels = Deserializer::deserialize_output_locks(block, 40).unwrap();
    assert!(input_labels == expected);
}

#[test]
fn test_deserialize_output_routing() {
    let block = "\
                        0 39\n\
                        1 1\n\
                        2 2\n\
                        3 6\n\
                        4 4\n\
                        5 5\n\
                        6 6\n\
                        7 7\n\
                        8 14\n\
                        9 9\n\
                        10 32\n\
                        11 11\n\
                        12 34\n\
                        13 14\n\
                        14 0\n\
                        15 14\n\
                        16 1\n\
                        17 1\n\
                        18 1\n\
                        19 31\n\
                        20 31\n\
                        21 0\n\
                        22 35\n\
                        23 33\n\
                        24 0\n\
                        25 31\n\
                        26 0\n\
                        27 32\n\
                        28 32\n\
                        29 32\n\
                        30 0\n\
                        31 1\n\
                        32 32\n\
                        33 33\n\
                        34 34\n\
                        35 31\n\
                        36 36\n\
                        37 37\n\
                        38 38\n\
                        39 32\n\
                        ";
    let expected = vec![
        protocol::Route {
            source: 39,
            destination: 0,
        },
        protocol::Route {
            source: 1,
            destination: 1,
        },
        protocol::Route {
            source: 2,
            destination: 2,
        },
        protocol::Route {
            source: 6,
            destination: 3,
        },
        protocol::Route {
            source: 4,
            destination: 4,
        },
        protocol::Route {
            source: 5,
            destination: 5,
        },
        protocol::Route {
            source: 6,
            destination: 6,
        },
        protocol::Route {
            source: 7,
            destination: 7,
        },
        protocol::Route {
            source: 14,
            destination: 8,
        },
        protocol::Route {
            source: 9,
            destination: 9,
        },
        protocol::Route {
            source: 32,
            destination: 10,
        },
        protocol::Route {
            source: 11,
            destination: 11,
        },
        protocol::Route {
            source: 34,
            destination: 12,
        },
        protocol::Route {
            source: 14,
            destination: 13,
        },
        protocol::Route {
            source: 0,
            destination: 14,
        },
        protocol::Route {
            source: 14,
            destination: 15,
        },
        protocol::Route {
            source: 1,
            destination: 16,
        },
        protocol::Route {
            source: 1,
            destination: 17,
        },
        protocol::Route {
            source: 1,
            destination: 18,
        },
        protocol::Route {
            source: 31,
            destination: 19,
        },
        protocol::Route {
            source: 31,
            destination: 20,
        },
        protocol::Route {
            source: 0,
            destination: 21,
        },
        protocol::Route {
            source: 35,
            destination: 22,
        },
        protocol::Route {
            source: 33,
            destination: 23,
        },
        protocol::Route {
            source: 0,
            destination: 24,
        },
        protocol::Route {
            source: 31,
            destination: 25,
        },
        protocol::Route {
            source: 0,
            destination: 26,
        },
        protocol::Route {
            source: 32,
            destination: 27,
        },
        protocol::Route {
            source: 32,
            destination: 28,
        },
        protocol::Route {
            source: 32,
            destination: 29,
        },
        protocol::Route {
            source: 0,
            destination: 30,
        },
        protocol::Route {
            source: 1,
            destination: 31,
        },
        protocol::Route {
            source: 32,
            destination: 32,
        },
        protocol::Route {
            source: 33,
            destination: 33,
        },
        protocol::Route {
            source: 34,
            destination: 34,
        },
        protocol::Route {
            source: 31,
            destination: 35,
        },
        protocol::Route {
            source: 36,
            destination: 36,
        },
        protocol::Route {
            source: 37,
            destination: 37,
        },
        protocol::Route {
            source: 38,
            destination: 38,
        },
        protocol::Route {
            source: 32,
            destination: 39,
        },
    ];
    let actual = Deserializer::deserialize_output_routing(block, 40).unwrap();
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
