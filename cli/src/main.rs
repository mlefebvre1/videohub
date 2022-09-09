mod defs;
mod display;

use defs::Cli;
use display::{format_input_labels, format_output_labels};
use log::info;
use std::{net::Ipv4Addr, str::FromStr};
use tera::Tera;
use videohub::{
    protocol::{
        BlockType, HubInfo, InputLabels, Label, LockStatus, OutputLabels, OutputLock, OutputLocks,
        OutputRoutings, Route,
    },
    Hub, DEFAULT_DEVICE_PORT,
};

use anyhow::Result;

fn main() -> Result<()> {
    let args = Cli::get();
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let ipv4_addr = Ipv4Addr::from_str(&args.ip_address)?;

    let videohub = Hub::new(ipv4_addr, DEFAULT_DEVICE_PORT);

    if let Some(Label(id, text)) = args.input_label {
        let block = BlockType::InputLabels(InputLabels(vec![Label(id, text.clone())]));

        info!("Changing label of input port {} to {}", id, text);
        videohub.write(block)?;
    }
    if let Some(Label(id, text)) = args.output_label {
        let block = BlockType::OutputLabels(OutputLabels(vec![Label(id, text.clone())]));
        info!("Changing label of output port {} to {}", id, text);
        videohub.write(block)?;
    }
    if let Some(Route(dst, src)) = args.output_route {
        let block = BlockType::VideoOutputRouting(OutputRoutings(vec![Route(dst, src)]));

        info!("Routing -- Input={} to Output={}", src, dst);
        videohub.write(block)?;
    }
    if let Some(index) = args.unlock {
        let block = BlockType::VideoOutputLocks(OutputLocks(vec![OutputLock(
            index,
            LockStatus::ForceUnlock,
        )]));

        videohub.write(block)?;
    }
    if let Some(index) = args.lock {
        let block =
            BlockType::VideoOutputLocks(OutputLocks(vec![OutputLock(index, LockStatus::Locked)]));
        videohub.write(block)?;
    }
    if args.display {
        let hub_info = videohub.read()?;
        display_hub_info(&hub_info).unwrap();
    }

    Ok(())
}

fn display_hub_info(hub_info: &HubInfo) -> Result<()> {
    let mut tera = Tera::new("**/templates/**/*.j2")?;

    tera.register_function("format_input_labels", format_input_labels);
    tera.register_function("format_output_labels", format_output_labels);

    // Find the relative template path name from the file name
    let template_name = tera
        .get_template_names()
        .find(|name| name.contains("device_info.j2"))
        .unwrap_or("");

    let context = get_context(hub_info);
    let result = tera.render(template_name, &context)?;
    println!("{result}");
    Ok(())
}

fn get_context(hub_info: &HubInfo) -> tera::Context {
    let mut context = tera::Context::new();

    context.insert("version", &hub_info.protocol_preamble.version);
    context.insert("device_present", &hub_info.device_info.device_present);
    context.insert("model_name", &hub_info.device_info.model_name);
    context.insert("friendly_name", &hub_info.device_info.friendly_name);
    context.insert("unique_id", &hub_info.device_info.unique_id);
    context.insert("nb_video_inputs", &hub_info.device_info.nb_video_inputs);
    context.insert("nb_video_outputs", &hub_info.device_info.nb_video_outputs);
    context.insert(
        "nb_video_processing_units",
        &hub_info.device_info.nb_video_processing_units,
    );
    context.insert(
        "nb_video_monitoring_outputs",
        &hub_info.device_info.nb_video_monitoring_outputs,
    );
    context.insert("nb_serial_ports", &hub_info.device_info.nb_serial_ports);
    context.insert("input_labels", &hub_info.input_labels);
    context.insert("output_labels", &hub_info.output_labels);
    context.insert("video_output_routing", &hub_info.video_output_routing);
    context.insert("video_output_locks", &hub_info.video_output_locks);
    context
}
