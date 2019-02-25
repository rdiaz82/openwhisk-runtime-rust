use std::collections::HashMap;

pub fn mainAction(input_data: HashMap<String, String>) -> String {
    let mut output: String = "Echo received data:".to_owned();
    for (key, value) in input_data {
        output = format!("{} {}:{}", output, key, value);
    }
    output
}
