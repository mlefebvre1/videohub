mod defs;
mod display;

use defs::Cli;
use display::{format_input_labels, format_output_labels};
use std::{net::Ipv4Addr, str::FromStr};
use tera::{Context, Tera};
use videohub::{protocol::HubInfo, Hub};

const DEVICE_PORT: u16 = 9990;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::get();
    let ipv4_addr = Ipv4Addr::from_str(&args.ip_address)?;

    let videohub = Hub::new(ipv4_addr, DEVICE_PORT);

    if let Some(input_label) = args.input_label {
        println!(
            "Changing label of input port {} to {}",
            input_label.index, input_label.value
        );
    }
    if let Some(output_label) = args.output_label {
        // do something
        println!(
            "Changing label of output port {} to {}",
            output_label.index, output_label.value
        );
    }
    if let Some(output_route) = args.output_route {
        println!(
            "Routing -- Input={} to Output={}",
            output_route.a, output_route.b
        );
    }
    if args.display {
        let hub_info = videohub.read()?;
        display_hub_info(&hub_info)?;
    }

    Ok(())
}

fn display_hub_info(hub_info: &HubInfo) -> Result<(), Box<dyn std::error::Error>> {
    let mut tera = Tera::new("**/display/templates/**/*")?;
    tera.register_function("format_input_labels", format_input_labels);
    tera.register_function("format_output_labels", format_output_labels);
    let s = tera.render(
        "src/bin/cli/display/templates/device_info.j2",
        &Context::from_serialize(hub_info)?,
    )?;
    println!("{s}");
    Ok(())
}
