pub fn example_device_info() -> &'static str {
    r#"{
    "device_present": "present",
    "model_name": "Smart Videohub 12G 40x40",
    "friendly_name": "SDI Router G-B - Smart Videohub 12G 40 x 40",
    "unique_id": "7C2E0D0BF1BE",
    "nb_video_inputs": 40,
    "nb_video_processing_units": 0,
    "nb_video_outputs": 40,
    "nb_video_monitoring_outputs": 0,
    "nb_serial_ports": 0
  }"#
}

pub fn example_input_ports() -> &'static str {
    r#"{
    "port_number": 0,
    "port_name": "X16 Multiviewer - Out 2"
  }"#
}

pub fn example_output_ports() -> &'static str {
    r#"{
    "port_number": 0,
    "port_name": "out_to_sebas_box6",
    "port_state": "unlocked",
    "source_port": 16
  }"#
}
