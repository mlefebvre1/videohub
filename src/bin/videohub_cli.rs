mod display;
use std::net::Ipv4Addr;

use display::format_input_labels;
use tera::{Context, Tera};
use videohub::Hub;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let videohub = Hub::new(Ipv4Addr::new(10, 26, 135, 201), 9990);
    let content = videohub.dump_hub_info()?;
    let hub_info = videohub.get_hub_info(&content)?;
    let mut tera = Tera::new("**/display/templates/**/*")?;
    tera.register_function("format_input_labels", format_input_labels);
    let s = tera.render(
        "src/bin/display/templates/device_info.j2",
        &Context::from_serialize(hub_info)?,
    )?;
    println!("{}", s);
    Ok(())
}

// fn format_video_hub_info(hub_info: &HubInfo) -> String {}
