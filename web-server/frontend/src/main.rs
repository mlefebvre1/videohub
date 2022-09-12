mod app;
mod fetch;
mod route;

use std::{net::Ipv4Addr, str::FromStr};
use anyhow::Result;
use app::Props;

use crate::app::Model;

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let ipv4_addr = Ipv4Addr::from_str("10.26.135.201")?;
    yew::start_app_with_props::<Model>(Props { ipv4_addr });
    Ok(())
}
