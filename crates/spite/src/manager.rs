use crate::{Gamepad, Result, backend::Backend};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GamepadManager<B: Backend> {
	backend: B,
}

impl<B: Backend> GamepadManager<B> {
	pub fn new(backend: B) -> Self {
		Self { backend }
	}

	pub fn gamepads(&self) -> Result<Vec<Gamepad>> {
		Ok(self.backend.gamepads()?.drain(..).map(Gamepad).collect())
	}
}
