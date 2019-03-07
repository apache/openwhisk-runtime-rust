extern crate actions;
extern crate libc;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use actions::main as actionMain;
use serde_derive::Deserialize;

use serde_json::{Error, Value};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{stderr, stdin, stdout, Write};
use std::os::unix::io::FromRawFd;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Input {
    value: HashMap<String, Value>,
    #[serde(flatten)]
    environment: HashMap<String, Value>,
}

fn main() {
    let mut fd3 = unsafe { File::from_raw_fd(3) };
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let parsed_input: Result<Input, Error> = serde_json::from_str(&buffer);
        match parsed_input {
            Ok(input) => {
                for (key, val) in input.environment {
                    env::set_var(format!("__OW_{}", key.to_uppercase()), val.to_string());
                }
                match serde_json::to_string(&actionMain(input.value)) {
                    Ok(result) => {
                        writeln!(&mut fd3, "{}", result).expect("Error writing on fd3");
                    }
                    Err(err) => {
                        eprintln!("Error formatting result value json: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error parsing input: {}", err);
            }
        }
        stdout().flush().expect("Error flushing stdout");
        stderr().flush().expect("Error flushing stderr");
    }
}
