#[cfg(windows)]
pub mod wgi;

use crate::{Result, axis::Axis, button::Button};

pub trait Backend: Default {
	type Gamepad: GamepadTrait;

	fn gamepads(&self) -> Result<Vec<Box<dyn GamepadTrait>>>;
}

pub trait GamepadTrait {
	fn name(&self) -> Result<String>;

	fn axis(&self, axis: Axis) -> f32;

	fn button(&self, button: Button) -> bool;
}
