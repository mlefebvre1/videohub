use rocket::{response::status::BadRequest, serde::json::Json};
use std::{net::Ipv4Addr, str::FromStr};

use videohub_proto::{
    protocol::{BlockType, Configuration, DeviceInfo, HubInfo, Label, OutputLock, Route},
    Hub,
};

use super::defs::{InputPort, OutputPort};

use itertools::izip;

use anyhow::Result;

type RequestResult<T> = Result<Json<T>, BadRequest<String>>;

#[get("/", format = "json")]
pub async fn hub() -> &'static str {
    r#"[
      "device_info",
      "input_ports",
      "output_ports",
      "configuration"
    ]"#
}

#[get("/device_info", format = "json")]
pub async fn device_info_get() -> RequestResult<DeviceInfo> {
    let hub_info = read_hub_info().await?;
    Ok(Json(hub_info.device_info))
}

#[get("/input_ports", format = "json")]
pub async fn input_ports_get() -> RequestResult<Vec<InputPort>> {
    let hub_info = read_hub_info().await?;
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

#[put("/input_ports", format = "json", data = "<input_ports>")]
pub async fn input_ports_put(input_ports: Json<Vec<InputPort>>) -> RequestResult<Vec<InputPort>> {
    let labels: Vec<Label> = input_ports
        .iter()
        .map(|input_port| Label(input_port.id, input_port.label.to_string()))
        .collect();
    let _ = write_hub_info(BlockType::InputLabels(labels)).await?;
    Ok(input_ports.clone())
}

#[get("/output_ports", format = "json")]
pub async fn output_ports_get() -> RequestResult<Vec<OutputPort>> {
    let hub_info = read_hub_info().await?;

    let response_data: Vec<OutputPort> = izip!(
        hub_info.output_labels,
        hub_info.video_output_locks,
        hub_info.video_output_routing
    )
    .map(
        |(Label(_, output_label), OutputLock(_, lock_state), Route(dst, src))| OutputPort {
            id: dst,
            input_port: Some(src),
            label: Some(output_label),
            lock_state: Some(lock_state),
        },
    )
    .collect();
    Ok(Json(response_data))
}

#[put("/output_ports", format = "json", data = "<output_ports>")]
pub async fn output_ports_put(
    output_ports: Json<Vec<OutputPort>>,
) -> RequestResult<Vec<OutputPort>> {
    let labels: Vec<Label> = output_ports
        .iter()
        .filter_map(|output_port| {
            output_port
                .label
                .as_ref()
                .map(|label| Label(output_port.id, label.to_string()))
        })
        .collect();
    if !labels.is_empty() {
        let _ = write_hub_info(BlockType::OutputLabels(labels)).await?;
    }

    let lock_statuses: Vec<OutputLock> = output_ports
        .iter()
        .filter_map(|output_port| {
            output_port
                .lock_state
                .as_ref()
                .map(|lock_state| OutputLock(output_port.id, lock_state.clone()))
        })
        .collect();
    if !lock_statuses.is_empty() {
        let _ = write_hub_info(BlockType::VideoOutputLocks(lock_statuses)).await?;
    }

    let routes: Vec<Route> = output_ports
        .iter()
        .filter_map(|output_port| {
            output_port
                .input_port
                .as_ref()
                .map(|input_port| Route(output_port.id, *input_port))
        })
        .collect();
    if !routes.is_empty() {
        let _ = write_hub_info(BlockType::VideoOutputRouting(routes)).await?;
    }

    Ok(output_ports.clone())
}

#[get("/configuration", format = "json")]
pub async fn configuration_get() -> RequestResult<Configuration> {
    let hub_info = read_hub_info().await?;
    Ok(Json(hub_info.configuration))
}

async fn read_hub_info() -> Result<HubInfo, BadRequest<String>> {
    let hub = get_hub();
    let hub_info = hub
        .read()
        .await
        .map_err(|_| BadRequest(Some("Failed to read videohub device infos".to_string())))?;
    Ok(hub_info)
}

async fn write_hub_info(hub_info: BlockType) -> Result<usize, BadRequest<String>> {
    let hub = get_hub();
    let nb_bytes = hub
        .write(hub_info)
        .await
        .map_err(|_| BadRequest(Some("Failed to write infos to videohub device".to_string())))?;
    Ok(nb_bytes)
}

fn get_hub() -> Hub {
    Hub::new(
        Ipv4Addr::from_str("10.26.135.201")
            .expect("Failed to create videohub, the ip address you provided is probably wrong."),
        videohub_proto::hub::DEFAULT_DEVICE_PORT,
    )
}
