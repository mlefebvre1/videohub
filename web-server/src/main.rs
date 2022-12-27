#[macro_use]
extern crate rocket;

mod api;
mod config;

#[launch]
fn rocket() -> _ {
    api::start()
}
