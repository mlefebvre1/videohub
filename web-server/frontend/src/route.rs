use super::{
    config::HOST_ADDRESS,
    fetch::{fetch_input_ports, fetch_output_ports},
};
use gloo::timers::callback::Interval;
use reqwest::Client;
use videohub_server_api_def::defs::{InputPort, OutputPort};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

const DEFAULT_BUTTON_BG_COLOR: &str = "rgb(155, 28, 30)";
const FOCUSED_BUTTON_BG_COLOR: &str = "rgb(27, 77, 203)";
const ROUTED_TO_OUTPUT_BUTTON_BG_COLOR: &str = "rgb(26, 188, 156)";
const ROUTED_TO_INPUT_BUTTON_BG_COLOR: &str = "rgb(214, 146, 20)";

pub enum Msg {
    InPortClicked(usize),
    OutPortClicked(usize),
    FetchInputPorts(Vec<InputPort>),
    FetchOutputPorts(Vec<OutputPort>),
    FetchVideohubInfo,
    Route,
    RouteDone,
}

pub struct Model {
    current_in_port_selected: Option<usize>,
    current_out_port_selected: Option<usize>,
    input_ports: Option<Vec<InputPort>>,
    output_ports: Option<Vec<OutputPort>>,
    _interval_handle: Interval,
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub nb_input_ports: usize,
    pub nb_output_ports: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchVideohubInfo);

        let fetch_videohub_info_interval_handle = {
            let link = ctx.link().clone();
            Interval::new(3_000, move || link.send_message(Msg::FetchVideohubInfo))
        };

        Self {
            current_in_port_selected: None,
            current_out_port_selected: None,
            input_ports: None,
            output_ports: None,
            _interval_handle: fetch_videohub_info_interval_handle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InPortClicked(i) => {
                self.current_in_port_selected = Some(i);
                self.update_button_colors(ctx);
                true
            }
            Msg::OutPortClicked(i) => {
                self.current_out_port_selected = Some(i);
                self.update_button_colors(ctx);
                true
            }
            Msg::FetchInputPorts(input_ports) => {
                self.input_ports = Some(input_ports);
                true
            }
            Msg::FetchOutputPorts(output_ports) => {
                self.output_ports = Some(output_ports);
                true
            }
            Msg::FetchVideohubInfo => {
                fetch_input_ports(ctx);
                fetch_output_ports(ctx);
                true
            }
            Msg::Route => {
                let body = serde_json::to_string(&vec![OutputPort {
                    id: self.current_out_port_selected.unwrap(),
                    input_port: self.current_in_port_selected,
                    label: None,
                    lock_state: None,
                }])
                .unwrap();

                ctx.link().send_future(async {
                    let client = Client::new();
                    client
                        .put(format!("http://{}/hub/output_ports", *HOST_ADDRESS))
                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                        .body(body)
                        .send()
                        .await
                        .unwrap();
                    Msg::RouteDone
                });
                true
            }
            Msg::RouteDone => {
                ctx.link().send_message(Msg::FetchVideohubInfo);
                self.current_in_port_selected = None;
                self.current_out_port_selected = None;
                self.set_default_input_buttons_colors(ctx);
                self.set_default_output_buttons_colors(ctx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let input_ports = match self.input_ports.clone() {
            Some(labels) => labels,
            None => vec![],
        };

        let output_ports = match self.output_ports.clone() {
            Some(labels) => labels,
            None => vec![],
        };

        html! {
            <>
                <div id="input_ports">
                    <h1 style={"text-align: center"}>{"Input Ports"}</h1>
                    {
                        html!{
                                for input_ports.into_iter().map(|input_port| html!(<button type="button" id={format!("in_button_{}", input_port.id)}
                                 onclick={link.callback(move |_| Msg::InPortClicked(input_port.id))} >{format!("IN{}", input_port.id)}<br/>{input_port.label.to_string()}</button>))
                        }
                    }
                </div>
                <div id="output_ports">
                    <h1 style={"text-align: center"}>{"Output Ports"}</h1>
                    {
                        html!{
                            for output_ports.into_iter().map(|output_port| html!(<button type="button" id={format!("out_button_{}", output_port.id)}
                             onclick={link.callback(move |_| Msg::OutPortClicked(output_port.id))} >{format!("OUT{}", output_port.id)}<br/>{output_port.label.unwrap().to_string()}</button>))
                        }
                    }
                </div>
                <div id="route">
                    <button id={"route_button"} onclick={link.callback(move |_| Msg::Route)}>{"Route"}</button>
                </div>
            </>
        }
    }
}

impl Model {
    fn update_button_colors(&self, ctx: &Context<Self>) {
        self.set_default_output_buttons_colors(ctx);
        self.set_default_input_buttons_colors(ctx);
        self.set_input_routed_to_output_color(ctx);
        self.set_output_routed_to_input_color(ctx);
        self.set_focused_input_button_color(ctx);
        self.set_focused_output_button_color(ctx);
    }

    fn set_default_input_buttons_colors(&self, ctx: &Context<Self>) {
        Self::set_default_buttons_colors("in_button_", ctx.props().nb_input_ports);
    }

    fn set_focused_input_button_color(&self, _ctx: &Context<Self>) {
        if let Some(i) = self.current_in_port_selected {
            Self::set_button_color(&format!("in_button_{i}"), FOCUSED_BUTTON_BG_COLOR);
        }
    }

    fn set_input_routed_to_output_color(&self, _ctx: &Context<Self>) {
        if let Some(output_id) = self.current_out_port_selected {
            let output_ports = self.output_ports.as_ref().unwrap();
            let output_ports_matched = output_ports
                .iter()
                .filter(|output_port| output_port.id == output_id);
            for output_port in output_ports_matched {
                Self::set_button_color(
                    &format!("in_button_{}", output_port.input_port.unwrap()),
                    ROUTED_TO_OUTPUT_BUTTON_BG_COLOR,
                )
            }
        }
    }
    fn set_output_routed_to_input_color(&self, _ctx: &Context<Self>) {
        if let Some(input_id) = self.current_in_port_selected {
            let output_ports = self.output_ports.as_ref().unwrap();
            let output_ports_matched = output_ports
                .iter()
                .filter(|output_port| output_port.input_port.unwrap() == input_id);
            for output_port in output_ports_matched {
                Self::set_button_color(
                    &format!("out_button_{}", output_port.id),
                    ROUTED_TO_INPUT_BUTTON_BG_COLOR,
                )
            }
        }
    }

    fn set_default_output_buttons_colors(&self, ctx: &Context<Self>) {
        Self::set_default_buttons_colors("out_button_", ctx.props().nb_output_ports);
    }

    fn set_focused_output_button_color(&self, _ctx: &Context<Self>) {
        if let Some(i) = self.current_out_port_selected {
            Self::set_button_color(&format!("out_button_{i}"), FOCUSED_BUTTON_BG_COLOR);
        }
    }

    fn set_default_buttons_colors(id_prefix: &str, nb_ports: usize) {
        for i in 0..nb_ports {
            Self::set_button_color(&format!("{id_prefix}{i}"), DEFAULT_BUTTON_BG_COLOR)
        }
    }

    fn set_button_color(element_id: &str, color: &str) {
        let document = web_sys::window().unwrap_throw().document().unwrap_throw();
        let el = document.get_element_by_id(element_id).unwrap_throw();
        let button = el.dyn_into::<web_sys::HtmlButtonElement>().unwrap();
        button
            .style()
            .set_property("background-color", color)
            .unwrap_throw();
    }
}
