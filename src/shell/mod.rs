extern crate dirs;

use std::{env, path};

mod command;
pub use self::command::{Command, CommandError, CommandResult};

pub struct Shell {
    pwd: path::PathBuf,
    os_path: Vec<String>,
}

impl Shell {
    pub fn new() -> Shell {
        let pwd = env::current_dir().unwrap();

        let os_path: Vec<String> = env::var_os("PATH")
            .unwrap()
            .into_string()
            .unwrap()
            .split(':')
            .map(String::from)
            .collect();

        Shell { pwd, os_path }
    }

    pub fn exec(&self, buffer: &str) -> Result<command::CommandResult, command::CommandError> {
        let current_command = command::Command::from_str(buffer);

        // TODO: to log command

        current_command.run(&self.os_path)
    }

    pub fn get_greeting(&self) -> String {
        let home = dirs::home_dir().unwrap();
        let path = self.pwd.strip_prefix(home).unwrap().to_str().unwrap();

        format!("~/{} >", path)
    }
}
