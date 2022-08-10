/// Blackmagic Videohub Ethernet Protocol v2.3
/// https://documents.blackmagicdesign.com/DeveloperManuals/VideohubDeveloperInformation.pdf?_v=1526627637000
///
/// The device sends information in blocks. Each block is separated by a blank line and starts with an identifier in ALL_CAP.
/// See the document for more information
///
pub mod de;
mod error;
pub mod ser;

use std::{fmt::Display, str::FromStr};

use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProtocolPreamble {
    pub version: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DevicePresent {
    Present,
    NotPresent,
    NeedUpdate,
}

impl FromStr for DevicePresent {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: usize,
    pub text: String,
}

// pub type Label = String;

pub type InputLabel = Vec<Label>;

pub type OutputLabel = Vec<Label>;

pub enum IOLabel {
    Input(InputLabel),
    Output(OutputLabel),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LockStatus {
    ForceUnlock,
    Locked,
    Owned,
    Unlocked,
}

impl FromStr for LockStatus {
    type Err = error::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(LockStatus::Locked),
            "O" => Ok(LockStatus::Owned),
            "U" => Ok(LockStatus::Unlocked),
            "F" => Ok(LockStatus::ForceUnlock),
            _ => {
                warn!("Invalid LockStatus string value {s}");
                Err(error::Error::LockStatusError)
            }
        }
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

impl Default for LockStatus {
    fn default() -> Self {
        LockStatus::Locked
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutputLock {
    pub id: usize,
    pub lock_status: LockStatus,
}

pub type OutputLocks = Vec<OutputLock>;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Route {
    pub source: usize,
    pub destination: usize,
}
pub type OutputRoutings = Vec<Route>;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub take_mode: bool,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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

pub enum WriteType<'a> {
    VideoOutputRouting(OutputRoutings),
    OutputLabel(&'a [Label]),
    InputLabel(&'a [Label]),
    VideoOutputLocks(&'a [OutputLock]),
}
