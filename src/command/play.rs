use rand::{thread_rng, Rng};
use std::{
	cmp::Ordering,
	io::{self, Write},
};

use crate::GameState;

use super::Command;

pub fn play(gamestate: GameState) -> GameState {
	if gamestate.ingame {
		println!(
			"A game is currently in progress. Please finish the game before starting a new one."
		);
		return gamestate;
	}

	let mut gamestate = gamestate.clone();

	let ans = thread_rng().gen_range(1..=gamestate.max);
	println!("A number has been decided, enter a guess or run a command.");
	io::stdout().flush().unwrap();

	loop {
		gamestate.increment_attempts();

		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read input.");
		let guess = guess.trim();

		if let Some(command) = guess.strip_prefix(':') {
			let command = command.parse::<Command>();

			match command {
				Err(e) => eprintln!("{e:?}"),
				Ok(command) => {
					gamestate = command.action(GameState {
						ingame: true,
						..gamestate
					})
				}
			};
			continue;
		};

		let guess: u16 = match guess.parse() {
			Ok(num) => num,
			Err(_) => {
				eprintln!("Invalid guess. Run commands by prefixing them with ':'.");
				continue;
			}
		};

		match guess.cmp(&ans) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				let score = gamestate.max_attempts - gamestate.attempts;
				gamestate.update_score(score.into());
				println!("Congratulations! You guessed the answer in {} tries! You have gained {score} points!", gamestate.attempts);
				break;
			}
		}

		if gamestate.attempts >= gamestate.max_attempts {
			println!("You have run out of attempts! The number was {ans}!");
			break;
		}
	}

	gamestate.increment_games();
	gamestate.reset_attempts()
}
