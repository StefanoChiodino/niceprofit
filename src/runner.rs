use std::process::{Command, Stdio};
use std::{thread, time};
use std::io::prelude::*;
use std::io::Error;


pub fn run(mut command: Command, milliseconds: u64) -> Result<Option<String>, Error> {
    let mut child = command.stdout(Stdio::piped()).spawn()?;
        let wait_time = time::Duration::from_millis(milliseconds);
        thread::sleep(wait_time);
        child.kill();
        if let Some(mut output) = child.stdout {
            let mut output_string = String::new();
            output.read_to_string(&mut output_string);
            Ok(Some(output_string))
        } else {
            Ok(None)
        }
}

#[test]
fn can_run_ping() {
    let path = "ping".to_string();
    let argument = "8.8.8.8".to_string();
    let mut command = Command::new(path);
    command.arg(argument);

    let run = run(command, 1000);

    assert!(run.is_ok());
}
