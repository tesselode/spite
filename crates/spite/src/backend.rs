#[cfg(windows)]
pub mod wgi;

use crate::{Result, axis::Axis, button::Button, event::Event};

pub trait Backend {
	type Gamepad: GamepadTrait;

	fn gamepads(&self) -> Result<Vec<Box<dyn GamepadTrait>>>;

	fn pop_event(&self) -> Option<Event>;
}

pub trait GamepadTrait: Send {
	fn name(&self) -> Result<String>;

	fn axis(&self, axis: Axis) -> f32;

	fn button(&self, button: Button) -> bool;
}
