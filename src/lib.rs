use std::error::Error;

mod board;

pub enum Command {
    Info,
    NewGame,
    Play,
    Pass,
    ValidMoves,
    BestMove,
    Undo,
    Options,
}

#[derive(Debug)]
pub struct InvalidCommandError;

impl std::fmt::Display for InvalidCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid command provided.")
    }
}

impl Error for InvalidCommandError {}

pub fn parse_command(input: &str) -> Result<Command, InvalidCommandError> {
    let command = match input.trim() {
        "info" => Command::Info,
        "newgame" => Command::NewGame,
        "play" => Command::Play,
        "pass" => Command::Pass,
        "validmoves" => Command::ValidMoves,
        "bestmove" => Command::BestMove,
        "undo" => Command::Undo,
        "options" => Command::Options,
        _ => Err(InvalidCommandError)?,
    };

    Ok(command)
}

pub fn process_command(command: Command) -> Result<Vec<String>, Box<dyn Error>> {
    let output = match command {
        Command::Info => vec!["id Beekeeper v0.10".to_string()],
        Command::NewGame => todo!(),
        Command::Play => todo!(),
        Command::Pass => todo!(),
        Command::ValidMoves => todo!(),
        Command::BestMove => todo!(),
        Command::Undo => todo!(),
        Command::Options => todo!(),
    };

    Ok(output)
}
