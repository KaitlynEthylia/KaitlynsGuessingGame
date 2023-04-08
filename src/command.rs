use std::{process::exit, str::FromStr};

use clearscreen::clear;

use crate::GameState;

use self::play::play;
use self::set::{Config, ConfigParseError};

mod play;
mod set;

#[derive(Debug)]
pub enum SubcommandError {
	ConfigParseError(ConfigParseError),
}

#[derive(Debug)]
pub enum CommandParseError {
	UnknownCommand,
	SubcommandError(SubcommandError),
}

#[derive(Debug)]
pub enum Command {
	EXIT,
	HELP,
	CLEAR,
	SET(Config),
	PLAY,
}

impl FromStr for Command {
	type Err = CommandParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (command, args) = s.split_once(' ').unwrap_or((s, ""));

		match command {
			"exit" => Ok(Command::EXIT),
			"help" => Ok(Command::HELP),
			"clear" => Ok(Command::CLEAR),
			"set" => parse_set(args),
			"play" => Ok(Command::PLAY),
			_ => Err(CommandParseError::UnknownCommand),
		}
	}
}

fn parse_set(args: &str) -> Result<Command, CommandParseError> {
	match args.parse::<Config>() {
		Ok(config) => Ok(Command::SET(config)),
		Err(e) => Err(CommandParseError::SubcommandError(
			SubcommandError::ConfigParseError(e),
		)),
	}
}

impl Command {
	pub fn action(&self, gamestate: GameState) -> GameState {
		match self {
			Self::EXIT => exit(0),
			Self::HELP => {
				print_help_text();
				gamestate
			}
			Self::CLEAR => {
				clear().unwrap_or(());
				gamestate
			}
			Self::SET(setting) => setting.apply(gamestate),
			Self::PLAY => play(gamestate),
		}
	}
}

fn print_help_text() {
	println!(
		r#"
I cannot be bothered to write a useful help text right now.
Here's a list of commands

:play - play the game.
:help - print this text.
:exit - quit the game.
:clear - clear the screen.
:set - change settings about the game.

Currently avaiable settings are 'max' and 'attempts', each of them can be assigned to a u16.
         "#
	)
}
