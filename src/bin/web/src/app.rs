use yew::prelude::*;

use crate::route;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <header style={"color: white; background: rgb(155, 28, 30)"}>
            <h1>{"Videohub"}</h1>
            <h3>{"SDI Router"}</h3>
        </header>
        <div>
            <route::Model nb_input_ports=40 nb_output_ports=40/>
        </div>
        </>
    }
}
