use super::{app, route};
use reqwest::Client;
use videohub::protocol::DeviceInfo;
use yew::prelude::*;

// use videohub_rest::{InputPort, OutputPort};

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

pub fn fetch_device_info(ctx: &Context<app::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let device_info: DeviceInfo = client
            .get("http://127.0.0.1:8000/hub/device_info")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        app::Msg::FetchDeviceInfo(device_info)
    });
}

pub fn fetch_input_ports(ctx: &Context<route::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let input_labels: Vec<InputPort> = client
            .get("http://127.0.0.1:8000/hub/input_ports")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        route::Msg::FetchInputPorts(input_labels)
    });
}

pub fn fetch_output_ports(ctx: &Context<route::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let output_ports: Vec<OutputPort> = client
            .get("http://127.0.0.1:8000/hub/output_ports")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        route::Msg::FetchOutputPorts(output_ports)
    });
}
