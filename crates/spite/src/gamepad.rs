use crate::{Axis, Button, Result, backend::GamepadTrait};

pub struct Gamepad(Box<dyn GamepadTrait>);

impl Gamepad {
	pub fn from_backend_gamepad(gamepad: impl GamepadTrait + 'static) -> Self {
		Self(Box::new(gamepad))
	}

	pub fn from_backend_gamepad_boxed(gamepad: Box<dyn GamepadTrait>) -> Self {
		Self(gamepad)
	}

	pub fn name(&self) -> Result<String> {
		self.0.name()
	}

	pub fn axis(&self, axis: Axis) -> f32 {
		self.0.axis(axis)
	}

	pub fn button(&self, button: Button) -> bool {
		self.0.button(button)
	}
}
