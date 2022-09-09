/// Blackmagic Videohub Ethernet Protocol v2.3
/// https://documents.blackmagicdesign.com/DeveloperManuals/VideohubDeveloperInformation.pdf?_v=1526627637000
///
/// The device sends information in blocks. Each block is separated by a blank line and starts with an identifier in ALL_CAP.
/// See the document for more information
///
pub mod de;
pub mod error;
pub mod ser;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "PROTOCOL PREAMBLE:\n")]
pub struct ProtocolPreamble {
    #[serde(rename = "Version")]
    pub version: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "VIDEOHUB DEVICE:\n")]
pub struct DeviceInfo {
    #[serde(rename = "Device present")]
    pub device_present: DevicePresent,
    #[serde(rename = "Model name")]
    pub model_name: String,
    #[serde(rename = "Friendly name")]
    pub friendly_name: String,
    #[serde(rename = "Unique ID")]
    pub unique_id: String,
    #[serde(rename = "Video inputs")]
    pub nb_video_inputs: usize,
    #[serde(rename = "Video processing units")]
    pub nb_video_processing_units: usize,
    #[serde(rename = "Video outputs")]
    pub nb_video_outputs: usize,
    #[serde(rename = "Video monitoring outputs")]
    pub nb_video_monitoring_outputs: usize,
    #[serde(rename = "Serial ports")]
    pub nb_serial_ports: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DevicePresent {
    #[serde(rename = "true")]
    Present,
    #[serde(rename = "false")]
    NotPresent,
    #[serde(rename = "needs_update")]
    NeedUpdate,
}

impl Default for DevicePresent {
    fn default() -> Self {
        DevicePresent::NotPresent
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "INPUT LABELS:\n")]
pub struct InputLabels(pub Vec<Label>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "OUTPUT LABELS:\n")]
pub struct OutputLabels(pub Vec<Label>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "")]
pub struct Label(pub usize, pub String); // (id, text)

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "VIDEO OUTPUT LOCKS:\n")]
pub struct OutputLocks(pub Vec<OutputLock>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "")]
pub struct OutputLock(pub usize, pub LockStatus); // (id, lock_status)

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LockStatus {
    #[serde(rename = "F")]
    ForceUnlock,
    #[serde(rename = "L")]
    Locked,
    #[serde(rename = "O")]
    Owned,
    #[serde(rename = "U")]
    Unlocked,
}

impl Default for LockStatus {
    fn default() -> Self {
        LockStatus::Locked
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "VIDEO OUTPUT ROUTING:\n")]
pub struct OutputRoutings(pub Vec<Route>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "")]
pub struct Route(pub usize, pub usize); // (dst, src)

// Configuraton
#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "CONFIGURATION:\n")]
pub struct Configuration {
    #[serde(rename = "Take Mode")]
    pub take_mode: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename = "END PRELUDE:\n")]
pub struct EndPrelude;

#[derive(Debug, Default, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct HubInfo {
    #[serde(rename = "PROTOCOL PREAMBLE")]
    pub protocol_preamble: ProtocolPreamble,
    #[serde(rename = "VIDEOHUB DEVICE")]
    pub device_info: DeviceInfo,
    #[serde(rename = "INPUT LABELS")]
    pub input_labels: InputLabels,
    #[serde(rename = "OUTPUT LABELS")]
    pub output_labels: OutputLabels,
    #[serde(rename = "VIDEO OUTPUT LOCKS")]
    pub video_output_locks: OutputLocks,
    #[serde(rename = "VIDEO OUTPUT ROUTING")]
    pub video_output_routing: OutputRoutings,
    #[serde(rename = "CONFIGURATION")]
    pub configuration: Configuration,
    #[serde(rename = "END PRELUDE")]
    end_prelude: EndPrelude,
}

#[derive(Debug, Clone, Serialize)]
pub enum BlockType {
    ProtocolPreamble(ProtocolPreamble),
    DeviceInfo(DeviceInfo),
    InputLabels(InputLabels),
    OutputLabels(OutputLabels),
    VideoOutputLocks(OutputLocks),
    VideoOutputRouting(OutputRoutings),
    Configuration(Configuration),
    EndPrelude(EndPrelude),
}

use std::{fmt::Display, str::FromStr};

impl FromStr for Label {
    type Err = self::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let index = chars
            .by_ref()
            .take_while(|&c| c != '=')
            .collect::<String>()
            .parse::<usize>()?;
        let value: String = chars.collect();
        Ok(Self(index, value))
    }
}

impl FromStr for Route {
    type Err = self::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_as_int = s.split('=').map(|word| word.parse::<usize>());
        let a = s_as_int.next().unwrap()?;
        let b = s_as_int.next().unwrap()?;
        Ok(Self(a, b))
    }
}

impl Display for LockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LockStatus::Locked => "L",
            LockStatus::Owned => "O",
            LockStatus::Unlocked => "U",
            LockStatus::ForceUnlock => "F",
        };
        if let Some(width) = f.width() {
            write!(f, "{s:width$}")
        } else {
            write!(f, "{s}")
        }
    }
}
