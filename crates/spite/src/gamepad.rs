use crate::{Axis, Button, Result, Vibration, backend::GamepadTrait};

pub struct Gamepad(Box<dyn GamepadTrait>);

impl Gamepad {
	pub fn from_gamepad_trait(gamepad: impl GamepadTrait + 'static) -> Self {
		Self(Box::new(gamepad))
	}

	pub fn name(&self) -> Result<String> {
		self.0.name()
	}

	pub fn id(&self) -> Result<String> {
		self.0.id()
	}

	pub fn connected(&self) -> bool {
		self.0.connected()
	}

	pub fn axis(&self, axis: Axis) -> f64 {
		self.0.axis(axis)
	}

	pub fn button(&self, button: Button) -> bool {
		self.0.button(button)
	}

	pub fn vibration(&self) -> Vibration {
		self.0.vibration()
	}

	pub fn set_vibration(&self, vibration: Vibration) {
		self.0.set_vibration(vibration)
	}
}
