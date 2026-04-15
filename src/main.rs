use std::{
    error::Error,
    io::{Write, stdin, stdout},
};

use beekeeper::{parse_command, process_command};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        let command = {
            let mut command = String::new();
            let _ = stdin.read_line(&mut command);
            match parse_command(&command) {
                Ok(command) => command,
                Err(error) => {
                    writeln!(&mut stdout, "{}", error)?;
                    continue;
                }
            }
        };
        let outputs = process_command(command)?;

        for output in outputs {
            writeln!(stdout, "{}", output)?;
        }

        writeln!(stdout, "ok")?;

        stdout.flush()?;
    }
}
