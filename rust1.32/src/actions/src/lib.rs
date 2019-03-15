extern crate serde_json;

use std::collections::HashMap;
use serde_json::Value;


pub fn main(mut input_data: HashMap<String, Value>) -> HashMap<String, Value> {
    input_data.insert("added_key".to_string(),Value::String("test".to_string()));
    input_data
}
