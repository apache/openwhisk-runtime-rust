extern crate serde_json;

use std::collections::HashMap;
use serde_json::Value;

pub fn main(args: HashMap<String, Value>) -> HashMap<String, Value> {
    let name_opt = args.get("name");
    let name = if name_opt.is_some() {
        name_opt.unwrap().as_str().unwrap()    
    } else {
        "stranger"
    };
    let mut out = HashMap::new();
    out.insert("greeting".to_string(), Value::String(format!("Hello, {}", name)));
    out
}
