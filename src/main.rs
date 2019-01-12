use std::io::{self, Write};

mod shell;

fn main() {
    let app = shell::Shell::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        buffer.clear();

        stdout.flush().unwrap();
        print!("{} ", app.get_greeting());
        stdout.flush().unwrap();

        stdin.read_line(&mut buffer).unwrap();

        let result = app.exec(&buffer);

        match result {
            Ok(result) => match result {
                shell::CommandResult::Ok(result) => match result {
                    Some(message) => print!("{}", message),

                    None => continue,
                },

                shell::CommandResult::Exit => {
                    break;
                }
            },

            Err(error) => match error {
                shell::CommandError::Fail(error) => {
                    eprintln!("shell: command fail: {}", error);
                }

                shell::CommandError::NotFound(command_name) => {
                    eprintln!("shell: command not found: {}", command_name);
                }
            },
        }
    }
}
