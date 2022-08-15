use std::net::Ipv4Addr;

use reqwest::Client;
use videohub::protocol::{DeviceInfo, OutputRoutings};

use yew::prelude::*;

pub struct Model {
    nb_input_ports: Option<usize>,
    nb_output_ports: Option<usize>,
    msg: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipv4_addr: Ipv4Addr,
}

pub enum Msg {
    FetchDeviceInfo(DeviceInfo),
    FetchText(String),
    FetchOutputRoutings(OutputRoutings),
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let client = Client::new();
            let s = client
                .get("http://192.168.1.102:8000/device_info")
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            Msg::FetchText(s)
        });
        // ctx.link().send_future(async {
        //     let hub_info = get_hub_info(ctx.props().ipv4_addr).await?;
        // });
        // let hub_info = videohub.read().unwrap_or_else(|_| {
        //     warn!("Failed to read data from Videohub, exiting");
        //     panic!();
        // });

        // log!("{}", videohub.read().err().unwrap());
        // let hub_info = videohub.read().unwrap_err();
        // Self {
        //     nb_input_ports: Some(hub_info.device_info.nb_video_inputs),
        //     nb_output_ports: Some(hub_info.device_info.nb_video_outputs),
        // }
        Self {
            nb_input_ports: Some(0),
            nb_output_ports: Some(0),
            msg: "".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchDeviceInfo(device_info) => {
                self.nb_input_ports = Some(device_info.nb_video_inputs);
                self.nb_output_ports = Some(device_info.nb_video_outputs);
                true
            }
            Msg::FetchText(s) => {
                self.msg = s;
                true
            }
            Msg::FetchOutputRoutings(_output_routings) => {
                //todo
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"Test"}</p>
                <p>{format!("Some message: {}", self.msg)}</p>
                <p>{format!("NB INPUT PORTS : {}", self.nb_input_ports.unwrap())}</p>
                <p>{format!("NB OUTPUT PORTS : {}", self.nb_output_ports.unwrap())}</p>
            </>
        }

        // fn view(&self, _ctx: &Context<Self>) -> Html {
        //     if let (Some(nb_input_ports), Some(nb_output_ports)) =
        //         (self.nb_input_ports, self.nb_output_ports)
        //     {
        //         html! {
        //             <>
        //             <header style={"color: white; background: rgb(155, 28, 30)"}>
        //                 <h1>{"Videohub"}</h1>
        //                 <h3>{"SDI Router"}</h3>
        //             </header>
        //             <div>
        //                 <route::Model nb_input_ports={nb_input_ports} nb_output_ports={nb_output_ports}/>
        //             </div>
        //             </>
        //         }
        //     } else {
        //         html! {}
        //     }
    }
}
