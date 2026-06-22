use windows::Gaming::Input::{Gamepad as WgiGamepad, GamepadButtons, RawGameController};

use crate::{
	Axis, Button, Error, Result,
	backend::{Backend, GamepadTrait},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WgiBackend;

impl Backend for WgiBackend {
	type Gamepad = WgiBackendGamepad;

	fn gamepads(&self) -> Result<Vec<Box<dyn GamepadTrait>>> {
		Ok(WgiGamepad::Gamepads()
			.map_err(|err| Error::Other(err.message()))?
			.into_iter()
			.map(|gamepad| Box::new(WgiBackendGamepad(gamepad)) as Box<dyn GamepadTrait>)
			.collect())
	}
}

pub struct WgiBackendGamepad(WgiGamepad);

impl GamepadTrait for WgiBackendGamepad {
	fn name(&self) -> Result<String> {
		let raw = RawGameController::FromGameController(&self.0)
			.map_err(|err| Error::Other(err.message()))?;
		raw.DisplayName()
			.map(|hstring| hstring.to_string())
			.map_err(|err| Error::Other(err.message()))
	}

	fn axis(&self, axis: Axis) -> f32 {
		let current = self.0.GetCurrentReading().unwrap();
		match axis {
			Axis::LeftStickX => current.LeftThumbstickX as f32,
			Axis::LeftStickY => current.LeftThumbstickY as f32,
			Axis::RightStickX => current.RightThumbstickX as f32,
			Axis::RightStickY => current.RightThumbstickY as f32,
			Axis::LeftTrigger => current.LeftTrigger as f32,
			Axis::RightTrigger => current.RightTrigger as f32,
		}
	}

	fn button(&self, button: Button) -> bool {
		let current = self.0.GetCurrentReading().unwrap();
		current.Buttons.contains(match button {
			Button::North => GamepadButtons::Y,
			Button::South => GamepadButtons::A,
			Button::West => GamepadButtons::X,
			Button::East => GamepadButtons::B,
			Button::DpadUp => GamepadButtons::DPadUp,
			Button::DpadDown => GamepadButtons::DPadDown,
			Button::DpadLeft => GamepadButtons::DPadLeft,
			Button::DpadRight => GamepadButtons::DPadRight,
			Button::LeftShoulder => GamepadButtons::LeftShoulder,
			Button::RightShoulder => GamepadButtons::RightShoulder,
			Button::LeftStick => GamepadButtons::LeftThumbstick,
			Button::RightStick => GamepadButtons::RightThumbstick,
			Button::Back => GamepadButtons::View,
			Button::Menu => GamepadButtons::Menu,
		})
	}
}
