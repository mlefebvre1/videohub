#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use std::path::Path;

mod api;
mod defs;

use api::{
    configuration_get, device_info_get, hub, input_ports_get, input_ports_put, output_ports_get,
    output_ports_put,
};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(Path::new("frontend/dist/")))
        .mount(
            "/hub",
            routes![
                hub,
                device_info_get,
                input_ports_get,
                input_ports_put,
                output_ports_get,
                output_ports_put,
                configuration_get,
            ],
        )
}
