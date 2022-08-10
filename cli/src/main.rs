mod defs;
mod display;

use defs::Cli;
use display::{format_input_labels, format_output_labels};
use log::info;
use std::{net::Ipv4Addr, str::FromStr};
use tera::{Context, Tera};
use videohub::{
    protocol::{HubInfo, Label, LockStatus, OutputLock, Route, WriteType},
    Hub, DEFAULT_DEVICE_PORT,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::get();
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let ipv4_addr = Ipv4Addr::from_str(&args.ip_address)?;

    let videohub = Hub::new(ipv4_addr, DEFAULT_DEVICE_PORT);

    if let Some(input_label) = args.input_label {
        let data = vec![Label {
            id: input_label.index,
            text: input_label.value.clone(),
        }];
        info!(
            "Changing label of input port {} to {}",
            input_label.index, input_label.value
        );
        videohub.write(WriteType::InputLabel(&data))?;
    }
    if let Some(output_label) = args.output_label {
        let data = vec![Label {
            id: output_label.index,
            text: output_label.value.clone(),
        }];
        info!(
            "Changing label of output port {} to {}",
            output_label.index, output_label.value
        );
        videohub.write(WriteType::OutputLabel(&data))?;
    }
    if let Some(output_route) = args.output_route {
        let data = vec![Route {
            source: output_route.a,
            destination: output_route.b,
        }];
        info!(
            "Routing -- Input={} to Output={}",
            output_route.a, output_route.b
        );
        videohub.write(WriteType::VideoOutputRouting(data))?;
    }
    if let Some(index) = args.unlock {
        let data = vec![OutputLock {
            id: index,
            lock_status: LockStatus::ForceUnlock,
        }];
        videohub.write(WriteType::VideoOutputLocks(&data))?;
    }
    if let Some(index) = args.lock {
        let data = vec![OutputLock {
            id: index,
            lock_status: LockStatus::Locked,
        }];
        videohub.write(WriteType::VideoOutputLocks(&data))?;
    }
    if args.display {
        let hub_info = videohub.read()?;
        display_hub_info(&hub_info).unwrap();
    }

    Ok(())
}

fn display_hub_info(hub_info: &HubInfo) -> Result<(), Box<dyn std::error::Error>> {
    let mut tera = Tera::new("**/templates/**/*.j2")?;

    tera.register_function("format_input_labels", format_input_labels);
    tera.register_function("format_output_labels", format_output_labels);

    // Find the relative template path name from the file name
    let template_name = tera
        .get_template_names()
        .find(|name| name.contains("device_info.j2"))
        .unwrap_or("");

    let result = tera.render(template_name, &Context::from_serialize(hub_info)?)?;
    println!("{result}");
    Ok(())
}
