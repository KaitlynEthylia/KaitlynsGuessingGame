use std::{num::ParseIntError, str::FromStr};

use crate::GameState;

#[derive(Debug)]
pub enum ConfigParseError {
	ArgsError,
	InvalidRange(ParseIntError),
	UnknownConfig,
}

#[derive(Debug)]
pub enum Config {
	SHOW,
	MAX(u16),
	ATTEMPTS(u16),
}
impl Config {
	pub fn apply(&self, gamestate: GameState) -> GameState {
		match self {
			Self::MAX(max) => GameState {
				max: *max,
				..gamestate
			},
			Self::ATTEMPTS(max_attempts) => GameState {
				max_attempts: *max_attempts,
				..gamestate
			},
			Self::SHOW => {
				println!("{gamestate}");
				gamestate
			}
		}
	}
}

impl FromStr for Config {
	type Err = ConfigParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Ok(Config::SHOW);
		}

		let slice = s.split(' ');
		let vec: Vec<&str> = slice.collect();

		if vec.len() != 2 {
			return Err(ConfigParseError::ArgsError);
		}

		let val: u16 = match vec[1].parse() {
			Ok(num) => num,
			Err(e) => return Err(ConfigParseError::InvalidRange(e)),
		};

		match vec[0] {
			"max" => Ok(Config::MAX(val)),
			"attempts" => Ok(Config::ATTEMPTS(val)),
			_ => return Err(ConfigParseError::UnknownConfig),
		}
	}
}
