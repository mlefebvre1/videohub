/// Blackmagic Videohub Ethernet Protocol v2.3
/// https://documents.blackmagicdesign.com/DeveloperManuals/VideohubDeveloperInformation.pdf?_v=1526627637000
///
/// The device sends information in blocks. Each block is separated by a blank line and starts with an identifier in ALL_CAP.
/// See the document for more information
///
pub mod de;
mod error;
pub mod ser;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProtocolPreamble {
    pub version: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DevicePresent {
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
            _ => Err(error::Error::DevicePresent),
        }
    }
}

impl Default for DevicePresent {
    fn default() -> Self {
        DevicePresent::NotPresent
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceInfo {
    pub device_present: DevicePresent,
    pub model_name: String,
    pub friendly_name: String,
    pub unique_id: String,
    pub nb_video_inputs: usize,
    pub nb_video_processing_units: usize,
    pub nb_video_outputs: usize,
    pub nb_video_monitoring_outputs: usize,
    pub nb_serial_ports: usize,
}

type Label = String;

type InputLabel = Vec<Label>;

type OutputLabel = Vec<Label>;

enum IOLabel {
    Input(InputLabel),
    Output(OutputLabel),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Route {
    pub source: usize,
    pub destination: usize,
}
type OutputRoutings = Vec<Route>;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Configuration {
    pub take_mode: bool,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HubInfo {
    pub protocol_preamble: ProtocolPreamble,
    pub device_info: DeviceInfo,
    pub input_labels: InputLabel,
    pub output_labels: OutputLabel,
    pub video_output_locks: OutputLocks,
    pub video_output_routing: OutputRoutings,
    pub configuration: Configuration,
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
