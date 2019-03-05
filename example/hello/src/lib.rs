extern crate serde_json;

use std::collections::HashMap;
use serde_json::Value;

pub fn main(mut input_data: HashMap<String, Value>) -> HashMap<String, Value> {
    input_data.insert("hello".to_string(),Value::String("world".to_string()));
    input_data
}
