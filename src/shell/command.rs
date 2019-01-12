use std::path::Path;
use std::process::{self, Stdio};

pub enum CommandError {
    Fail(String),
    NotFound(String),
}

pub enum CommandResult {
    Ok(Option<String>),
    Exit,
}

pub struct Command<'a> {
    bin: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn from_str(command_str: &'a str) -> Command<'a> {
        let mut elements: Vec<&'a str> = command_str.trim_matches('\n').trim().split(' ').collect();

        let bin = elements.remove(0);

        Command {
            bin,
            arguments: elements,
        }
    }

    pub fn run(&self, os_path: &[String]) -> Result<CommandResult, CommandError> {
        if self.bin == "" {
            return Ok(CommandResult::Ok(None));
        }

        match self.find_bin(os_path) {
            Some(absolute_bin_path) => {
                let mut command = process::Command::new(absolute_bin_path);

                command
                    .args(&self.arguments)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped());

                match command.output() {
                    Ok(output) => {
                        let result = String::from_utf8_lossy(&output.stdout).to_string();

                        Ok(CommandResult::Ok(Some(result)))
                    }

                    Err(error) => Err(CommandError::Fail(error.to_string())),
                }
            }

            None => match self.bin {
                "exit" => Ok(CommandResult::Exit),

                _ => Err(CommandError::NotFound(self.bin.to_string())),
            },
        }
    }

    fn find_bin(&self, os_path: &[String]) -> Option<String> {
        for path in os_path {
            let path = Path::new(path).join(self.bin);

            if path.exists() {
                let bin_folder = path.to_str().unwrap().to_string();

                return Some(bin_folder);
            }
        }

        None
    }
}
