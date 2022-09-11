use serde::{Deserialize, Serialize};
use videohub::protocol::LockStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputPort {
    #[serde(rename = "Port #")]
    pub id: usize,
    #[serde(rename = "Port Name")]
    pub label: Option<String>,
    #[serde(rename = "Port State")]
    pub lock_state: Option<LockStatus>,
    #[serde(rename = "Source Port #")]
    pub input_port: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputPort {
    #[serde(rename = "Port #")]
    pub id: usize,
    #[serde(rename = "Port Name")]
    pub label: String,
}
