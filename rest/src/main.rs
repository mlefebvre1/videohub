#[macro_use]
extern crate rocket;
use rocket::{http::Status, serde::json::Json};
use std::{net::Ipv4Addr, str::FromStr};

use videohub::{
    protocol::{HubInfo, Label, WriteType},
    Hub, HubError,
};

#[get("/", format = "json")]
fn root_get() -> &'static str {
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
fn device_info_get() -> String {
    let hub_info = get_hub_info().unwrap();
    let resp = serde_json::to_string(&hub_info.device_info);
    resp.unwrap()
}

#[get("/input_labels", format = "json")]
fn input_labels_get() -> String {
    let hub_info = get_hub_info().unwrap();
    let resp = serde_json::to_string(&hub_info.input_labels);
    resp.unwrap()
}

#[put("/input_labels", format = "json", data = "<labels>")]
fn input_labels_put(labels: Json<Vec<Label>>) -> Status {
    let hub = connect_to_hub().unwrap();
    let resp = match hub.write(WriteType::InputLabel(&labels)) {
        Ok(_) => Status::Ok,
        Err(_) => Status::BadRequest,
    };
    resp
}

#[get("/output_labels", format = "json")]
fn output_labels_get() -> String {
    let hub_info = get_hub_info().unwrap();
    let resp = serde_json::to_string(&hub_info.output_labels);
    resp.unwrap()
}

#[put("/output_labels", format = "json", data = "<labels>")]
fn output_labels_put(labels: Json<Vec<Label>>) -> Status {
    let hub = connect_to_hub().unwrap();
    let resp = match hub.write(WriteType::OutputLabel(&labels)) {
        Ok(_) => Status::Ok,
        Err(_) => Status::BadRequest,
    };
    resp
}

#[get("/video_output_locks", format = "json")]
fn video_output_locks_get() -> String {
    let hub_info = get_hub_info().unwrap();
    let resp = serde_json::to_string(&hub_info.video_output_locks);
    resp.unwrap()
}

#[get("/video_output_routing", format = "json")]
fn video_output_routing_get() -> String {
    let hub_info = get_hub_info().unwrap();
    let resp = serde_json::to_string(&hub_info.video_output_routing);
    resp.unwrap()
}

#[get("/configuration", format = "json")]
fn configuration_get() -> String {
    let hub_info = get_hub_info().unwrap();
    let resp = serde_json::to_string(&hub_info.configuration);
    resp.unwrap()
}

fn get_hub_info() -> Result<HubInfo, HubError> {
    let hub = connect_to_hub()?;
    let hub_info = hub.read()?;
    Ok(hub_info)
}

fn connect_to_hub() -> Result<Hub, HubError> {
    let hub = Hub::new(
        Ipv4Addr::from_str("10.26.135.201").unwrap(),
        videohub::DEFAULT_DEVICE_PORT,
    );
    Ok(hub)
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            root_get,
            device_info_get,
            input_labels_get,
            input_labels_put,
            output_labels_get,
            output_labels_put,
            video_output_locks_get,
            video_output_routing_get,
            configuration_get,
        ],
    )
}
