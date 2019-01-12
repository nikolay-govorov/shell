pub enum CommandError {
    NotFound(String),
}

pub enum CommandResult {
    Exit,
}

pub struct Command<'a> {
    bin: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn from_str(command_str: &'a str) -> Command<'a> {
        let mut elements: Vec<&'a str> = command_str
            .trim_matches('\n')
            .split(' ')
            .collect();

        let bin = elements.remove(0);

        Command {
            bin,
            arguments: elements,
        }
    }

    pub fn run(&self) -> Result<CommandResult, CommandError> {
        match self.bin {
            "exit" => Ok(CommandResult::Exit),

            _ => Err(CommandError::NotFound(self.bin.to_string()))
        }
    }
}
