use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;

const DEFAULT_BUTTON_BG_COLOR: &str = "rgb(155, 28, 30)";
const FOCUSED_BUTTON_BG_COLOR: &str = "rgb(27, 77, 203)";

pub enum Msg {
    InPortClicked(usize),
    OutPortClicked(usize),
    Route,
}

pub struct Model {
    current_in_port_selected: Option<usize>,
    current_out_port_selected: Option<usize>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub nb_input_ports: usize,
    pub nb_output_ports: usize,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            current_in_port_selected: None,
            current_out_port_selected: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InPortClicked(i) => {
                self.current_in_port_selected = Some(i);
                self.set_default_input_buttons_colors(ctx);
                self.set_focused_input_button_color(ctx);
                true
            }
            Msg::OutPortClicked(i) => {
                self.current_out_port_selected = Some(i);
                self.set_default_output_buttons_colors(ctx);
                self.set_focused_output_button_color(ctx);
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

        let in_buttons = 1..=ctx.props().nb_input_ports;
        let out_buttons = 1..=ctx.props().nb_output_ports;

        html! {
            <>
                <div id="input_ports">
                <h1 style={"text-align: center"}>{"Input Ports"}</h1>
                {
                    html! {
                        for in_buttons.map(|i| html!(<button id={format!("in_button_{i}")} onclick={link.callback(move |_| Msg::InPortClicked(i))} >{format!("IN{i}")}</button>))
                    }
                }
                </div>
                <div id="output_ports">
                <h1 style={"text-align: center"}>{"Output Ports"}</h1>
                {
                    html! {
                        for out_buttons.map(|i| html!(<button id={format!("out_button_{i}")} onclick={link.callback(move |_| Msg::OutPortClicked(i))} >{format!("OUT{i}")}</button>))
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
