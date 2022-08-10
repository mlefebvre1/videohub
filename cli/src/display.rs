use std::collections::HashMap;

use itertools::izip;
use serde_json::value::{from_value, to_value, Value};
use tera::{Error, Result};
use videohub::protocol::{InputLabel, OutputLabel, OutputLocks, OutputRoutings};

pub fn format_input_labels(args: &HashMap<String, Value>) -> Result<Value> {
    let labels = match args.get("input_labels") {
        Some(val) => match from_value::<InputLabel>(val.clone()) {
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
        .enumerate()
        .map(|(id, label)| {
            format!(
                "|{index:^8}| {label_text:<78}|",
                index = id + 1,
                label_text = label.text
            )
        })
        .collect::<Vec<String>>(); // +1 because indexes are 1 based
    Ok(to_value(formatted_output)?)
}

pub fn format_output_labels(args: &HashMap<String, Value>) -> Result<Value> {
    // TODO: check if we can just make a macro to get these params..
    let output_label = match args.get("output_labels") {
        Some(val) => match from_value::<OutputLabel>(val.clone()) {
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
        Some(val) => match from_value::<OutputLocks>(val.clone()) {
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
        Some(val) => match from_value::<OutputRoutings>(val.clone()) {
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
        .map(|(label, output_lock, route)| {
            format!(
                "|{dest:^8}| {label_text:<50}| {lock_status:^13}| {src:^11}|",
                dest = route.destination + 1,
                label_text = label.text,
                lock_status = output_lock.lock_status,
                src = route.source + 1
            ) // +1 because indexes are 1 based
        })
        .collect::<Vec<String>>();
    Ok(to_value(formatted_output)?)
}
