extern crate serde_json;
extern crate actions;
extern crate libc;

use std::env;
use std::io::{self, Write, stdout, stderr};
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::collections::HashMap;
use serde_json::{Value, Error};
use actions::main as actionMain;

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let parsed_input:Result<HashMap<String,Value>,Error> = serde_json::from_str(&buffer);
        let mut payload:HashMap<String, Value> = HashMap::new();
        match parsed_input {
            Ok(n) => {
                for (key, val) in n {
                    if key == "value" {
                        let mut unparsed_payload:Result<HashMap<String,Value>,Error> = serde_json::from_value(val);
                        match unparsed_payload {
                            Ok(value) => payload = value,
                            Err(_) => eprintln!("Error parsing value json")
                        }
                    } else {
                        env::set_var(format!("__OW_{}", key.to_uppercase()), val.to_string());
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e)
        }
        let action_results = actionMain(payload);
        let mut fd3 = unsafe { File::from_raw_fd(3) };
        write!(&mut fd3, "{}", action_results);
        stdout().flush();
        stderr().flush();
    }
}
