#[macro_use]
extern crate rocket;
use rocket::{fs::FileServer, response::status::BadRequest, serde::json::Json};
use std::{net::Ipv4Addr, path::Path, str::FromStr};

use videohub::{
    protocol::{
        Configuration, DeviceInfo, HubInfo, Label, OutputLock, OutputRoutings, Route, WriteType,
    },
    Hub, HubError,
};

type RequestResult<T> = Result<Json<T>, BadRequest<String>>;

#[get("/", format = "json")]
fn hub() -> &'static str {
    r#"[
      "device_info",
      "input_labels",
      "output_labels",
      "video_output_locks",
      "video_output_routing",
      "configuration"
    ]"#
}

#[get("/device_info", format = "json")]
fn device_info_get() -> RequestResult<DeviceInfo> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.device_info))
}

#[get("/input_labels", format = "json")]
fn input_labels_get() -> RequestResult<Vec<Label>> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.input_labels))
}

#[put("/input_labels", format = "json", data = "<labels>")]
fn input_labels_put(labels: Json<Vec<Label>>) -> RequestResult<Vec<Label>> {
    let _ = write_hub_info(WriteType::InputLabel(&labels))?;
    Ok(labels)
}

#[get("/output_labels", format = "json")]
fn output_labels_get() -> RequestResult<Vec<Label>> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.output_labels))
}

#[put("/output_labels", format = "json", data = "<labels>")]
fn output_labels_put(labels: Json<Vec<Label>>) -> RequestResult<Vec<Label>> {
    let _ = write_hub_info(WriteType::InputLabel(&labels))?;
    Ok(labels)
}

#[get("/video_output_locks", format = "json")]
fn video_output_locks_get() -> RequestResult<Vec<OutputLock>> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.video_output_locks))
}

#[put("/video_output_locks", format = "json", data = "<locks>")]
fn video_output_locks_put(locks: Json<Vec<OutputLock>>) -> RequestResult<Vec<OutputLock>> {
    let _ = write_hub_info(WriteType::VideoOutputLocks(&locks))?;
    Ok(locks)
}

#[get("/video_output_routing", format = "json")]
fn video_output_routing_get() -> RequestResult<Vec<Route>> {
    let hub_info = read_hub_info()?;
    Ok(Json(hub_info.video_output_routing))
}

#[put("/video_output_routing", format = "json", data = "<routes>")]
fn video_output_routing_put(routes: Json<OutputRoutings>) -> RequestResult<OutputRoutings> {
    let _ = write_hub_info(WriteType::VideoOutputRouting(routes.to_vec()))?;
    Ok(routes)
}

#[get("/configuration", format = "json")]
fn configuration_get() -> RequestResult<Configuration> {
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

fn write_hub_info(hub_info: WriteType) -> Result<usize, BadRequest<String>> {
    let hub = connect_to_hub()
        .map_err(|_| BadRequest(Some("Failed to connect to videohub device".to_string())))?;
    let nb_bytes = hub
        .write(hub_info)
        .map_err(|_| BadRequest(Some("Failed to write infos to videohub device".to_string())))?;
    Ok(nb_bytes)
}

fn connect_to_hub() -> Result<Hub, HubError> {
    let hub = Hub::new(
        Ipv4Addr::from_str("10.26.135.201")?,
        videohub::DEFAULT_DEVICE_PORT,
    );
    Ok(hub)
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(Path::new("frontend/dist/")))
        .mount(
            "/hub",
            routes![
                hub,
                device_info_get,
                input_labels_get,
                input_labels_put,
                output_labels_get,
                output_labels_put,
                video_output_locks_get,
                video_output_locks_put,
                video_output_routing_get,
                video_output_routing_put,
                configuration_get,
            ],
        )
}
