use reqwest::Client;
use videohub::protocol::{DeviceInfo, InputLabel, OutputRoutings};
use yew::prelude::*;

use super::{app, route};

pub fn fetch_device_info(ctx: &Context<app::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let device_info: DeviceInfo = client
            .get("http://192.168.1.102:8000/hub/device_info")
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

pub fn fetch_input_labels(ctx: &Context<route::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let input_labels: InputLabel = client
            .get("http://192.168.1.102:8000/hub/input_labels")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        route::Msg::FetchInputLabels(input_labels)
    });
}

pub fn fetch_output_labels(ctx: &Context<route::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let output_labels: InputLabel = client
            .get("http://192.168.1.102:8000/hub/output_labels")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        route::Msg::FetchOutputLabels(output_labels)
    });
}

pub fn fetch_output_routings(ctx: &Context<route::Model>) {
    ctx.link().send_future(async {
        let client = Client::new();
        let routes: OutputRoutings = client
            .get("http://192.168.1.102:8000/hub/video_output_routing")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        route::Msg::FetchOutputRoutings(routes)
    });
}
