use crate::protocol;

struct Deserializer {}

impl Deserializer {
    pub fn deserialize_device_info(
        &self,
        block: &str,
    ) -> Result<protocol::DeviceInfo, protocol::error::Error> {
        // Parse and extract info for the block VIDEOHUB DEVICE
        // It is expected that VIDEOHUB DEVICE: label is removed
        let mut device_info: protocol::DeviceInfo = Default::default();
        for line in block.lines() {
            match line {
                line if line.starts_with("Device present:") => {
                    device_info.device_present =
                        protocol::DevicePresent::from_str(&self.get_info_from_line_raw(line)?)?
                }
                line if line.starts_with("Model name:") => {
                    device_info.model_name = self.get_info_from_line_raw(line)?
                }
                line if line.starts_with("Friendly name:") => {
                    device_info.friendly_name = self.get_info_from_line_raw(line)?
                }
                line if line.starts_with("Unique ID:") => {
                    device_info.unique_id = self.get_info_from_line_raw(line)?
                }
                line if line.starts_with("Video inputs:") => {
                    device_info.nb_video_inputs = self.get_info_from_line_as_usize(line)?
                }
                line if line.starts_with("Video processing units:") => {
                    device_info.nb_video_processing_units =
                        self.get_info_from_line_as_usize(line)?
                }
                line if line.starts_with("Video outputs:") => {
                    device_info.nb_video_outputs = self.get_info_from_line_as_usize(line)?
                }
                line if line.starts_with("Video monitoring outputs:") => {
                    device_info.nb_video_monitoring_outputs =
                        self.get_info_from_line_as_usize(line)?
                }
                line if line.starts_with("Serial ports:") => {
                    device_info.nb_serial_ports = self.get_info_from_line_as_usize(line)?
                }
                _ => (),
            }
        }
        Ok(device_info)
    }

    pub fn deserialize_to_vec_str(
        &self,
        block: &str,
        expected_size: usize,
    ) -> Result<Vec<String>, protocol::error::Error> {
        let mut labels = Vec::new();
        labels.reserve_exact(expected_size);
        labels.resize(expected_size, "".to_string());

        for line in block.lines() {
            let mut line_as_iter = line.chars();

            let label_id = line_as_iter
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>()
                .parse::<usize>()
                .map_err(protocol::error::Error::ParseInt)?;

            let label_value: String = line_as_iter.collect();

            if label_id < expected_size {
                labels[label_id] = label_value.trim().to_string();
            } else {
                return Err(protocol::error::Error::LabelsLengthError);
            }
        }
        Ok(labels)
    }

    fn deserialize_labels(
        &self,
        block: &str,
        expected_size: usize,
    ) -> Result<Vec<protocol::Label>, protocol::error::Error> {
        self.deserialize_to_vec_str(block, expected_size)
    }

    fn deserialize_output_locks(
        &self,
        block: &str,
        expected_size: usize,
    ) -> Result<protocol::OutputLocks, protocol::error::Error> {
        let v = self.deserialize_to_vec_str(block, expected_size)?;
        let output_locks: Result<protocol::OutputLocks, protocol::error::Error> = v
            .iter()
            .map(|item| protocol::LockStatus::from_str(&item))
            .collect();
        Ok(output_locks?)
    }

    fn get_info_from_line_raw(&self, line: &str) -> Result<String, protocol::error::Error> {
        if let Some(value) = line.split(":").nth(1) {
            Ok(value.trim().to_string())
        } else {
            Err(protocol::error::Error::ParseValueError)
        }
    }

    fn get_info_from_line_as_usize(&self, line: &str) -> Result<usize, protocol::error::Error> {
        self.get_info_from_line_raw(line)?
            .parse::<usize>()
            .map_err(protocol::error::Error::ParseInt)
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

    let de = Deserializer {};
    let device_info = de.deserialize_device_info(block).unwrap();

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

    let de = Deserializer {};
    let input_labels = de.deserialize_labels(block, 40).unwrap();
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
    let de = Deserializer {};
    let input_labels = de.deserialize_output_locks(block, 40).unwrap();
    assert!(input_labels == expected);
}
