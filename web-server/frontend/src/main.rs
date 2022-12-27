mod app;
mod config;
mod fetch;
mod route;

use crate::app::Model;
use anyhow::Result;

fn main() -> Result<()> {
    yew::start_app::<Model>();
    Ok(())
}
