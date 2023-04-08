use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct GameState {
	pub games: u32,
	pub score: u32,
	pub max: u16,
	pub max_attempts: u16,
	pub attempts: u16,
	pub ingame: bool,
}

impl GameState {
	pub fn increment_attempts(&mut self) {
		self.attempts += 1
	}

	pub fn increment_games(&mut self) {
		self.games += 1
	}

	pub fn reset_attempts(&self) -> GameState {
		GameState {
			attempts: 0,
			..self.clone()
		}
	}

	pub fn update_score(&mut self, score: u32) {
		self.score += score
	}
}

impl Display for GameState {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		let out = format!(
			r#"
Game State:

    Games: {}
    Score: {}
    Max: {}
    Attempts: {}

Round State:

    Attempt: {}
            "#,
			self.games, self.score, self.max, self.max_attempts, self.attempts
		);

		writeln!(f, "{}", out)
	}
}
