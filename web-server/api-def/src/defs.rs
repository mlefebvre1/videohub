use super::api_doc_examples::{example_device_info, example_input_ports, example_output_ports};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use videohub_proto::protocol;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum DevicePresent {
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "not_present")]
    NotPresent,
    #[serde(rename = "need_update")]
    NeedUpdate,
}

impl From<protocol::DevicePresent> for DevicePresent {
    fn from(rhs: protocol::DevicePresent) -> Self {
        match rhs {
            protocol::DevicePresent::NeedUpdate => Self::NeedUpdate,
            protocol::DevicePresent::NotPresent => Self::NotPresent,
            protocol::DevicePresent::Present => Self::Present,
        }
    }
}

impl From<DevicePresent> for protocol::DevicePresent {
    fn from(rhs: DevicePresent) -> Self {
        match rhs {
            DevicePresent::NeedUpdate => Self::NeedUpdate,
            DevicePresent::NotPresent => Self::NotPresent,
            DevicePresent::Present => Self::Present,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum LockStatus {
    #[serde(rename = "force_unlock")]
    ForceUnlock,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "owned")]
    Owned,
    #[serde(rename = "unlocked")]
    Unlocked,
}

impl From<protocol::LockStatus> for LockStatus {
    fn from(rhs: protocol::LockStatus) -> Self {
        match rhs {
            protocol::LockStatus::ForceUnlock => Self::ForceUnlock,
            protocol::LockStatus::Locked => Self::Locked,
            protocol::LockStatus::Owned => Self::Owned,
            protocol::LockStatus::Unlocked => Self::Unlocked,
        }
    }
}

impl From<LockStatus> for protocol::LockStatus {
    fn from(rhs: LockStatus) -> Self {
        match rhs {
            LockStatus::ForceUnlock => Self::ForceUnlock,
            LockStatus::Locked => Self::Locked,
            LockStatus::Owned => Self::Owned,
            LockStatus::Unlocked => Self::Unlocked,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[schemars(example = "example_device_info")]
pub struct DeviceInfo {
    /// Indicate if a device is available and if it needs to be updated
    pub device_present: DevicePresent,
    /// Vendor model name
    pub model_name: String,
    /// User videohub device name
    pub friendly_name: String,
    pub unique_id: String,
    pub nb_video_inputs: usize,
    pub nb_video_processing_units: usize,
    pub nb_video_outputs: usize,
    pub nb_video_monitoring_outputs: usize,
    pub nb_serial_ports: usize,
}

impl From<protocol::DeviceInfo> for DeviceInfo {
    fn from(rhs: protocol::DeviceInfo) -> Self {
        Self {
            device_present: DevicePresent::from(rhs.device_present),
            model_name: rhs.model_name,
            friendly_name: rhs.friendly_name,
            unique_id: rhs.unique_id,
            nb_video_inputs: rhs.nb_video_inputs,
            nb_video_processing_units: rhs.nb_video_processing_units,
            nb_video_outputs: rhs.nb_video_outputs,
            nb_video_monitoring_outputs: rhs.nb_video_monitoring_outputs,
            nb_serial_ports: rhs.nb_serial_ports,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[schemars(example = "example_output_ports")]
pub struct OutputPort {
    #[serde(rename = "port_number")]
    pub id: usize,
    #[serde(rename = "port_name")]
    pub label: Option<String>,
    #[serde(rename = "port_state")]
    pub lock_state: Option<LockStatus>,
    #[serde(rename = "source_port")]
    pub input_port: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[schemars(example = "example_input_ports")]
pub struct InputPort {
    #[serde(rename = "port_number")]
    pub id: usize,
    #[serde(rename = "port_name")]
    pub label: String,
}

// Configuraton
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Configuration {
    pub take_mode: bool,
}

impl From<protocol::Configuration> for Configuration {
    fn from(rhs: protocol::Configuration) -> Self {
        Self {
            take_mode: rhs.take_mode,
        }
    }
}
