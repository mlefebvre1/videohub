use std::net::Ipv4Addr;

use super::fetch::fetch_device_info;
use super::route;

use videohub_proto::protocol::DeviceInfo;
use yew::prelude::*;

pub struct Model {
    nb_input_ports: Option<usize>,
    nb_output_ports: Option<usize>,
    friendly_name: Option<String>,
}
#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub ipv4_addr: Ipv4Addr,
}

pub enum Msg {
    FetchDeviceInfo(DeviceInfo),
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        fetch_device_info(ctx);
        Self {
            nb_input_ports: Some(0),
            nb_output_ports: Some(0),
            friendly_name: Some("".to_owned()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchDeviceInfo(device_info) => {
                self.nb_input_ports = Some(device_info.nb_video_inputs);
                self.nb_output_ports = Some(device_info.nb_video_outputs);
                self.friendly_name = Some(device_info.friendly_name);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{format!("Friendly Name: {}", self.friendly_name.as_ref().unwrap())}</p>
                <p>{format!("NB INPUT PORTS : {}", self.nb_input_ports.unwrap())}</p>
                <p>{format!("NB OUTPUT PORTS : {}", self.nb_output_ports.unwrap())}</p>
                <header style={"color: white; background: rgb(155, 28, 30)"}>
                    <h1>{"Videohub"}</h1>
                    <h3>{"SDI Router"}</h3>
                </header>
                <div>
                    <route::Model nb_input_ports={self.nb_input_ports.unwrap()} nb_output_ports={self.nb_output_ports.unwrap()}/>
                </div>
            </>
        }
    }
}
