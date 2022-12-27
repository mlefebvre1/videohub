use super::{app, config::HOST_ADDRESS, route};
use reqwest::Client;
use videohub_server_api_def::defs::{DeviceInfo, InputPort, OutputPort};
use yew::prelude::*;

pub fn fetch_device_info(ctx: &Context<app::Model>) {
    ctx.link().send_future(async {
        let url = format!("http://{}/hub/device_info", *HOST_ADDRESS);
        let client = Client::new();
        let device_info: DeviceInfo = client
            .get(&url)
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
        let url = format!("http://{}/hub/input_ports", *HOST_ADDRESS);
        let client = Client::new();
        let input_labels: Vec<InputPort> = client
            .get(&url)
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
        let url = format!("http://{}/hub/output_ports", *HOST_ADDRESS);
        let client = Client::new();
        let output_ports: Vec<OutputPort> = client
            .get(&url)
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
