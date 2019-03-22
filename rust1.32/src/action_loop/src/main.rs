use actions::main as actionMain;

use serde_derive::Deserialize;
use serde_json::{Error, Value};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{stderr, stdin, stdout, BufRead, Write},
    os::unix::io::FromRawFd,
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Input {
    value: Value,
    #[serde(flatten)]
    environment: HashMap<String, Value>,
}

fn log_error(fd3: &mut File, error: Error) {
    writeln!(fd3, "{{\"error\":\"{}\"}}\n", error).expect("Error writing on fd3");
    eprintln!("error: {}", error);
}

fn main() {
    let mut fd3 = unsafe { File::from_raw_fd(3) };
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let buffer: String = line.expect("Error reading line");
        let parsed_input: Result<Input, Error> = serde_json::from_str(&buffer);
        match parsed_input {
            Ok(input) => {
                for (key, val) in input.environment {
                    env::set_var(format!("__OW_{}", key.to_uppercase()), val.to_string());
                }
                match actionMain(input.value) {
                    Ok(action_result) => match serde_json::to_string(&action_result) {
                        Ok(response) => {
                            writeln!(&mut fd3, "{}", response).expect("Error writing on fd3")
                        }
                        Err(err) => log_error(&mut fd3, err),
                    },
                    Err(err) => {
                        log_error(&mut fd3, err);
                    }
                }
            }
            Err(err) => {
                log_error(&mut fd3, err);
            }
        }
        stdout().flush().expect("Error flushing stdout");
        stderr().flush().expect("Error flushing stderr");
    }
}

