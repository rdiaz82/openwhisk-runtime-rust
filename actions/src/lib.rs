extern crate serde_json;

use std::collections::HashMap;
use serde_json::Value;


pub fn main(input_data: HashMap<String, Value>) -> String {
    let mut output: String = "Echo received data:".to_owned();
    for (key, value) in input_data {
        output = format!("{} {}:{}", output, key, value.to_string());
    }
    output
}
