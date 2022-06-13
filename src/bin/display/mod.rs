use std::collections::HashMap;

use serde_json::value::{from_value, to_value, Value};
use tera::{Error, Result};
use videohub::protocol::InputLabel;

pub fn format_input_labels(args: &HashMap<String, Value>) -> Result<Value> {
    let labels = match args.get("input_labels") {
        Some(val) => match from_value::<InputLabel>(val.clone()) {
            Ok(v) => v
                .into_iter()
                .enumerate()
                .map(|(id, label)| format!("{id:^7}| {label}").to_string())
                .collect::<Vec<String>>(),
            Err(_) => {
                return Err(Error::msg(format!("")));
            }
        },
        None => {
            return Err(Error::msg(""));
        }
    };
    Ok(to_value(labels).unwrap())
}
