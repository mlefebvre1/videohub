use super::config::VIDEOHUB_IPV4_ADDR;
use anyhow::Result;
use itertools::izip;
use rocket::{fs::FileServer, response::status::BadRequest, serde::json::Json, Build, Rocket};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use std::path::Path;
use videohub_proto::{protocol, Hub};
use videohub_server_api_def::defs::{Configuration, DeviceInfo, InputPort, LockStatus, OutputPort};

type RequestResult<T> = Result<Json<T>, BadRequest<String>>;

#[openapi(tag = "Hub Informations")]
#[get("/device_info", format = "json")]
pub async fn device_info_get() -> RequestResult<DeviceInfo> {
    let hub_info: protocol::HubInfo = read_hub_info().await?;
    Ok(Json(DeviceInfo::from(hub_info.device_info)))
}

#[openapi(tag = "Ports")]
#[get("/input_ports", format = "json")]
pub async fn input_ports_get() -> RequestResult<Vec<InputPort>> {
    let hub_info = read_hub_info().await?;
    let response_data: Vec<InputPort> = hub_info
        .input_labels
        .iter()
        .map(|protocol::Label(id, label)| InputPort {
            id: *id,
            label: label.to_string(),
        })
        .collect();
    Ok(Json(response_data))
}

#[openapi(tag = "Ports")]
#[put("/input_ports", format = "json", data = "<input_ports>")]
pub async fn input_ports_put(input_ports: Json<Vec<InputPort>>) -> RequestResult<Vec<InputPort>> {
    let labels: Vec<protocol::Label> = input_ports
        .iter()
        .map(|input_port| protocol::Label(input_port.id, input_port.label.to_string()))
        .collect();
    let _ = write_hub_info(protocol::BlockType::InputLabels(labels)).await?;
    Ok(input_ports.clone())
}

#[openapi(tag = "Ports")]
#[get("/output_ports", format = "json")]
pub async fn output_ports_get() -> RequestResult<Vec<OutputPort>> {
    let hub_info = read_hub_info().await?;

    let response_data: Vec<OutputPort> = izip!(
        hub_info.output_labels,
        hub_info.video_output_locks,
        hub_info.video_output_routing
    )
    .map(
        |(
            protocol::Label(_, output_label),
            protocol::OutputLock(_, lock_state),
            protocol::Route(dst, src),
        )| OutputPort {
            id: dst,
            input_port: Some(src),
            label: Some(output_label),
            lock_state: Some(LockStatus::from(lock_state)),
        },
    )
    .collect();
    Ok(Json(response_data))
}

#[openapi(tag = "Ports")]
#[put("/output_ports", format = "json", data = "<output_ports>")]
pub async fn output_ports_put(
    output_ports: Json<Vec<OutputPort>>,
) -> RequestResult<Vec<OutputPort>> {
    let labels: Vec<protocol::Label> = output_ports
        .iter()
        .filter_map(|output_port| {
            output_port
                .label
                .as_ref()
                .map(|label| protocol::Label(output_port.id, label.to_string()))
        })
        .collect();
    if !labels.is_empty() {
        let _ = write_hub_info(protocol::BlockType::OutputLabels(labels)).await?;
    }

    let lock_statuses: Vec<protocol::OutputLock> = output_ports
        .iter()
        .filter_map(|output_port| {
            output_port.lock_state.as_ref().map(|lock_state| {
                protocol::OutputLock(output_port.id, lock_state.to_owned().into())
            })
        })
        .collect();
    if !lock_statuses.is_empty() {
        let _ = write_hub_info(protocol::BlockType::VideoOutputLocks(lock_statuses)).await?;
    }

    let routes: Vec<protocol::Route> = output_ports
        .iter()
        .filter_map(|output_port| {
            output_port
                .input_port
                .as_ref()
                .map(|input_port| protocol::Route(output_port.id, *input_port))
        })
        .collect();
    if !routes.is_empty() {
        let _ = write_hub_info(protocol::BlockType::VideoOutputRouting(routes)).await?;
    }

    Ok(output_ports.clone())
}

#[openapi(tag = "Hub Informations")]
#[get("/configuration", format = "json")]
pub async fn configuration_get() -> RequestResult<Configuration> {
    let hub_info = read_hub_info().await?;
    Ok(Json(Configuration::from(hub_info.configuration)))
}

async fn read_hub_info() -> Result<protocol::HubInfo, BadRequest<String>> {
    let hub = get_hub();
    let hub_info = hub
        .read()
        .await
        .map_err(|_| BadRequest(Some("Failed to read videohub device infos".to_string())))?;
    Ok(hub_info)
}

async fn write_hub_info(hub_info: protocol::BlockType) -> Result<usize, BadRequest<String>> {
    let hub = get_hub();
    let nb_bytes = hub
        .write(hub_info)
        .await
        .map_err(|_| BadRequest(Some("Failed to write infos to videohub device".to_string())))?;
    Ok(nb_bytes)
}

fn get_hub() -> Hub {
    Hub::new(
        *VIDEOHUB_IPV4_ADDR,
        videohub_proto::hub::DEFAULT_DEVICE_PORT,
    )
}

pub fn start() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/hub",
            openapi_get_routes![
                device_info_get,
                input_ports_get,
                input_ports_put,
                output_ports_get,
                output_ports_put,
                configuration_get,
            ],
        )
        .mount(
            "/doc/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/hub/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/",
            FileServer::from(Path::new("web-server/frontend/dist/")),
        )
}
