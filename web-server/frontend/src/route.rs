use super::fetch::{fetch_input_labels, fetch_output_labels, fetch_output_routings};

use gloo::timers::callback::Interval;
use videohub::protocol::{InputLabel, OutputLabel, OutputRoutings};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

const DEFAULT_BUTTON_BG_COLOR: &str = "rgb(155, 28, 30)";
const FOCUSED_BUTTON_BG_COLOR: &str = "rgb(27, 77, 203)";
const ROUTED_TO_OUTPUT_BUTTON_BG_COLOR: &str = "rgb(26, 188, 156)";
const ROUTED_TO_INPUT_BUTTON_BG_COLOR: &str = "rgb(214, 146, 20)";

pub enum Msg {
    InPortClicked(usize),
    OutPortClicked(usize),
    FetchInputLabels(InputLabel),
    FetchOutputLabels(OutputLabel),
    FetchOutputRoutings(OutputRoutings),
    FetchVideohubInfo,
    Route,
}

pub struct Model {
    current_in_port_selected: Option<usize>,
    current_out_port_selected: Option<usize>,
    input_labels: Option<InputLabel>,
    output_labels: Option<OutputLabel>,
    routes: Option<OutputRoutings>,
    _interval_handle: Interval,
}

#[derive(Properties, PartialEq)]
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
            input_labels: None,
            output_labels: None,
            routes: None,
            _interval_handle: fetch_videohub_info_interval_handle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InPortClicked(i) => {
                self.current_in_port_selected = Some(i);
                self.set_default_output_buttons_colors(ctx);
                self.set_default_input_buttons_colors(ctx);
                self.set_input_routed_to_output_color(ctx);
                self.set_output_routed_to_input_color(ctx);
                self.set_focused_input_button_color(ctx);
                self.set_focused_output_button_color(ctx);
                true
            }
            Msg::OutPortClicked(i) => {
                self.current_out_port_selected = Some(i);
                self.set_default_output_buttons_colors(ctx);
                self.set_default_input_buttons_colors(ctx);
                self.set_input_routed_to_output_color(ctx);
                self.set_output_routed_to_input_color(ctx);
                self.set_focused_input_button_color(ctx);
                self.set_focused_output_button_color(ctx);
                true
            }
            Msg::FetchInputLabels(labels) => {
                self.input_labels = Some(labels);
                true
            }
            Msg::FetchOutputLabels(labels) => {
                self.output_labels = Some(labels);
                true
            }
            Msg::FetchOutputRoutings(routes) => {
                self.routes = Some(routes);
                true
            }
            Msg::FetchVideohubInfo => {
                fetch_input_labels(ctx);
                fetch_output_labels(ctx);
                fetch_output_routings(ctx);
                true
            }
            Msg::Route => {
                // TODO ROUTE!
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

        let input_labels = match self.input_labels.clone() {
            Some(labels) => labels,
            None => vec![],
        };

        let output_labels = match self.output_labels.clone() {
            Some(labels) => labels,
            None => vec![],
        };

        html! {
            <>
                <div id="input_ports">
                    <h1 style={"text-align: center"}>{"Input Ports"}</h1>
                    {
                        html!{
                            for input_labels.into_iter().map(|label| html!(<button id={format!("in_button_{}", label.id)}
                             onclick={link.callback(move |_| Msg::InPortClicked(label.id))} >{format!("IN{}\n{}", label.id, label.text)}</button>))
                        }
                    }
                </div>
                <div id="output_ports">
                    <h1 style={"text-align: center"}>{"Output Ports"}</h1>
                    {
                        html!{
                            for output_labels.into_iter().map(|label| html!(<button id={format!("out_button_{}", label.id)}
                             onclick={link.callback(move |_| Msg::OutPortClicked(label.id))} >{format!("OUT{}\n{}", label.id, label.text)}</button>))
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
            let routes = self.routes.as_ref().unwrap();
            let matched_routes = routes.iter().filter(|route| route.destination == output_id);
            for route in matched_routes {
                Self::set_button_color(
                    &format!("in_button_{}", route.source),
                    ROUTED_TO_OUTPUT_BUTTON_BG_COLOR,
                )
            }
        }
    }
    fn set_output_routed_to_input_color(&self, _ctx: &Context<Self>) {
        if let Some(input_id) = self.current_in_port_selected {
            let routes = self.routes.as_ref().unwrap();
            let matched_routes = routes.iter().filter(|route| route.source == input_id);
            for route in matched_routes {
                Self::set_button_color(
                    &format!("out_button_{}", route.destination),
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
        for i in 1..=nb_ports {
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
