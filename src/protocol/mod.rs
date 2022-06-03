/// Blackmagic Videohub Ethernet Protocol v2.3
/// https://documents.blackmagicdesign.com/DeveloperManuals/VideohubDeveloperInformation.pdf?_v=1526627637000
///
/// The device sends information in blocks. Each block is separated by a blank line and starts with an identifier in ALL_CAP.
/// See the document for more information
///
mod de;
mod error;
mod ser;

#[derive(Debug, Default)]
pub struct ProtocolPreamble {
    version: String,
}

#[derive(Debug, PartialEq)]
enum DevicePresent {
    Present,
    NotPresent,
    NeedUpdate,
}

impl DevicePresent {
    pub fn from_str(s: &str) -> Result<Self, error::Error> {
        match s {
            "true" => Ok(DevicePresent::Present),
            "false" => Ok(DevicePresent::NotPresent),
            "needs_update" => Ok(DevicePresent::NeedUpdate),
            _ => Err(error::Error::DevicePresentError),
        }
    }
}

impl Default for DevicePresent {
    fn default() -> Self {
        DevicePresent::NotPresent
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct DeviceInfo {
    device_present: DevicePresent,
    model_name: String,
    friendly_name: String,
    unique_id: String,
    nb_video_inputs: usize,
    nb_video_processing_units: usize,
    nb_video_outputs: usize,
    nb_video_monitoring_outputs: usize,
    nb_serial_ports: usize,
}

type Label = String;

type InputLabel = Vec<Label>;

type OutputLabel = Vec<Label>;

enum IOLabel {
    Input(InputLabel),
    Output(OutputLabel),
}

#[derive(Debug, PartialEq)]
pub enum LockStatus {
    ForceUnlock,
    Locked,
    Unlocked,
}

impl LockStatus {
    pub fn from_str(s: &str) -> Result<Self, error::Error> {
        match s {
            "O" => Ok(LockStatus::Locked),
            "U" => Ok(LockStatus::Unlocked),
            "F" => Ok(LockStatus::ForceUnlock),
            _ => Err(error::Error::LockStatusError),
        }
    }
}

impl Default for LockStatus {
    fn default() -> Self {
        LockStatus::Locked
    }
}

type OutputLocks = Vec<LockStatus>;

#[derive(Debug, Default)]
pub struct Route {
    source: usize,
    destination: usize,
}
type OutputRoutings = Vec<Route>;

#[derive(Debug, Default)]
pub struct Configuration {
    take_mode: bool,
}

#[derive(Debug, Default)]
pub struct HubInfo {
    protocol_preamble: ProtocolPreamble,
    device_info: DeviceInfo,
    input_labels: InputLabel,
    output_labels: OutputLabel,
    video_output_locks: OutputLocks,
    video_output_routing: OutputRoutings,
    configuration: Configuration,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    ProtocolPreamble,
    DeviceInfo,
    InputLabel,
    OutputLabel,
    VideoOutputLocks,
    VideoOutputRouting,
    Configuration,
    EndPrelude,
}
