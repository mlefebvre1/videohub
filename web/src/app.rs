use std::net::Ipv4Addr;

use crate::route;
use log::warn;
use videohub::{protocol::HubInfo, Hub, HubError, DEFAULT_DEVICE_PORT};
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

pub struct Model {
    nb_input_ports: Option<usize>,
    nb_output_ports: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipv4_addr: Ipv4Addr,
}

impl Component for Model {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
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
            nb_input_ports: Some(1),
            nb_output_ports: Some(1),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{"Test"}</p>
                <p>{format!("{}", _ctx.props().ipv4_addr)}</p>
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
