#[cfg(windows)]
pub mod wgi;

use crate::{Gamepad, Result, axis::Axis, button::Button, event::Event};

pub trait Backend {
	fn gamepads(&self) -> Result<Vec<Gamepad>>;

	fn pop_event(&self) -> Option<Event>;
}

pub trait GamepadTrait: Send {
	fn name(&self) -> Result<String>;

	fn id(&self) -> Result<String>;

	fn connected(&self) -> bool;

	fn axis(&self, axis: Axis) -> f32;

	fn button(&self, button: Button) -> bool;
}
