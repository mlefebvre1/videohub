use itertools::izip;
use serde_json::value::{from_value, to_value, Value};
use std::collections::HashMap;
use tera::{Error, Result};
use videohub_proto::protocol::{Label, OutputLock, Route};

pub fn format_input_labels(args: &HashMap<String, Value>) -> Result<Value> {
    let labels = match args.get("input_labels") {
        Some(val) => match from_value::<Vec<Label>>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(String::new()));
            }
        },
        None => {
            return Err(Error::msg(""));
        }
    };
    let formatted_output = labels
        .into_iter()
        .map(|Label(id, text)| {
            format!(
                "|{index:^8}| {label_text:<78}|",
                index = id,
                label_text = text
            )
        })
        .collect::<Vec<String>>();
    Ok(to_value(formatted_output)?)
}

pub fn format_output_labels(args: &HashMap<String, Value>) -> Result<Value> {
    let output_label = match args.get("output_labels") {
        Some(val) => match from_value::<Vec<Label>>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(""));
            }
        },
        None => {
            return Err(Error::msg(""));
        }
    };
    let video_output_locks = match args.get("video_output_locks") {
        Some(val) => match from_value::<Vec<OutputLock>>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(""));
            }
        },
        None => {
            return Err(Error::msg(""));
        }
    };
    let video_output_routing = match args.get("video_output_routing") {
        Some(val) => match from_value::<Vec<Route>>(val.clone()) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::msg(""));
            }
        },
        None => {
            return Err(Error::msg(""));
        }
    };
    let formatted_output = izip!(output_label, video_output_locks, video_output_routing)
        .map(
            |(
                Label(_label_id, label_text),
                OutputLock(_lock_id, lock_status),
                Route(route_dst, route_src),
            )| {
                format!(
                    "|{dest:^8}| {label_text:<50}| {lock_status:^13}| {src:^11}|",
                    dest = route_dst,
                    label_text = label_text,
                    lock_status = lock_status,
                    src = route_src
                )
            },
        )
        .collect::<Vec<String>>();
    Ok(to_value(formatted_output)?)
}
