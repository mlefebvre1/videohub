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
                return Err(Error::msg(format!("")));
            }
        },
        None => {
            return Err(Error::msg(""));
        }
    };
    let formatted_output = labels
        .into_iter()
        .enumerate()
        .map(|(id, label)| format!("|{index:^6}| {label:<78}|", index = id + 1).to_string())
        .collect::<Vec<String>>();
    Ok(to_value(formatted_output).unwrap())
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
    println!("{:?}", video_output_routing);
    let formatted_output = izip!(output_label, video_output_locks, video_output_routing)
        .map(|(output_label, lock_state, route)| {
            format!(
                "|{dest:^6}| {output_label:<50}| {lock_state:^13}| {src:^11}|",
                dest = route.destination + 1,
                src = route.source
            )
        })
        .collect::<Vec<String>>();
    Ok(to_value(formatted_output).unwrap())
}
