use std::io;

use crate::{command::Command, gamestate::GameState};

mod command;
mod gamestate;

fn main() {
	let mut state = GameState {
		games: 0,
		score: 0,
		max: 50,
		max_attempts: 10,
		attempts: 0,
		ingame: false,
	};

	println!("Something something gussing game, something something :help\n");

	loop {
		state = play_game(state)
	}
}

fn play_game(gamestate: GameState) -> GameState {
	println!("Games: {}\nScore: {}\n", gamestate.games, gamestate.score);

	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");

	if let Some(command) = input.trim().strip_prefix(':') {
		let command = command.parse::<Command>();

		match command {
			Err(e) => {
				eprintln!("{e:?}");
				gamestate
			}
			Ok(command) => command.action(gamestate),
		}
	} else {
		eprintln!("You are currently not in a game. run :play to start the game.");
		gamestate
	}
}
