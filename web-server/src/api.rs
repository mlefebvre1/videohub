use rocket::{response::status::BadRequest, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, str::FromStr};

use videohub::{
    protocol::{Configuration, DeviceInfo, HubInfo, Label, LockStatus, OutputLock, Route},
    Hub,
};

use itertools::izip;

use anyhow::Result;

type RequestResult<T> = Result<Json<T>, BadRequest<String>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputPort {
    #[serde(rename = "Port #")]
    id: usize,
    #[serde(rename = "Port Name")]
    label: String,
    #[serde(rename = "Port State")]
    lock_state: LockStatus,
    #[serde(rename = "Source Port #")]
    input_port: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputPort {
    #[serde(rename = "Port #")]
    id: usize,
    #[serde(rename = "Port Name")]
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PortManagement {}

#[get("/", format = "json")]
pub fn hub() -> &'static str {
    r#"[
      "device_info",
      "input_ports",
      "output_ports",
      "configuration"
    ]"#
}

#[get("/device_info", format = "json")]
pub fn device_info_get() -> RequestResult<DeviceInfo> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.device_info))
}

#[get("/input_ports", format = "json")]
pub fn input_ports_get() -> RequestResult<Vec<InputPort>> {
    let hub_info = read_hub_info()?;
    let response_data: Vec<InputPort> = hub_info
        .input_labels
        .iter()
        .map(|Label(id, label)| InputPort {
            id: *id,
            label: label.to_string(),
        })
        .collect();
    Ok(Json(response_data))
}

#[get("/output_ports", format = "json")]
pub fn output_ports_get() -> RequestResult<Vec<OutputPort>> {
    let hub_info = read_hub_info()?;

    let response_data: Vec<OutputPort> = izip!(
        hub_info.output_labels,
        hub_info.video_output_locks,
        hub_info.video_output_routing
    )
    .map(
        |(Label(_, output_label), OutputLock(_, lock_state), Route(dst, src))| OutputPort {
            id: dst,
            input_port: src,
            label: output_label,
            lock_state,
        },
    )
    .collect();
    Ok(Json(response_data))
}

#[get("/configuration", format = "json")]
pub fn configuration_get() -> RequestResult<Configuration> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.configuration))
}

fn read_hub_info() -> Result<HubInfo, BadRequest<String>> {
    let hub = connect_to_hub()
        .map_err(|_| BadRequest(Some("Failed to connect to videohub device".to_string())))?;
    let hub_info = hub
        .read()
        .map_err(|_| BadRequest(Some("Failed to read videohub device infos".to_string())))?;
    Ok(hub_info)
}

// fn write_hub_info(hub_info: WriteType) -> Result<usize, BadRequest<String>> {
//     let hub = connect_to_hub()
//         .map_err(|_| BadRequest(Some("Failed to connect to videohub device".to_string())))?;
//     let nb_bytes = hub
//         .write(hub_info)
//         .map_err(|_| BadRequest(Some("Failed to write infos to videohub device".to_string())))?;
//     Ok(nb_bytes)
// }

fn connect_to_hub() -> Result<Hub> {
    let hub = Hub::new(
        Ipv4Addr::from_str("10.26.135.201")?,
        videohub::DEFAULT_DEVICE_PORT,
    );
    Ok(hub)
}
