use std::sync::{Arc, Mutex, MutexGuard};

use windows::Gaming::Input::{
	Gamepad as WgiGamepad, GamepadButtons, GamepadVibration, RawGameController,
};

use crate::{Axis, Button, Error, Result, Vibration, backend::GamepadTrait};

#[derive(Debug, Clone)]
pub struct WgiBackendGamepad(Arc<Mutex<WgiBackendGamepadInner>>);

impl WgiBackendGamepad {
	pub fn new(raw_game_controller: RawGameController) -> windows::core::Result<Self> {
		let wgi_gamepad = WgiGamepad::FromGameController(&raw_game_controller)?;
		let inner = WgiBackendGamepadInner {
			connected: true,
			raw_game_controller,
			wgi_gamepad,
		};
		Ok(Self(Arc::new(Mutex::new(inner))))
	}

	pub fn id(&self) -> windows::core::Result<String> {
		let id = self
			.inner()
			.raw_game_controller
			.NonRoamableId()?
			.to_string();
		Ok(id)
	}

	pub fn set_connected(&self, connected: bool) {
		self.0.lock().unwrap().connected = connected;
	}

	pub fn update_raw_game_controller(
		&self,
		raw_game_controller: RawGameController,
	) -> windows::core::Result<()> {
		let wgi_gamepad = WgiGamepad::FromGameController(&raw_game_controller)?;
		let mut inner = self.inner();
		inner.raw_game_controller = raw_game_controller;
		inner.wgi_gamepad = wgi_gamepad;
		Ok(())
	}

	fn inner(&self) -> MutexGuard<'_, WgiBackendGamepadInner> {
		self.0.lock().unwrap()
	}
}

impl GamepadTrait for WgiBackendGamepad {
	fn name(&self) -> Result<String> {
		Ok(self
			.inner()
			.raw_game_controller
			.DisplayName()
			.map_err(|err| Error::Other(err.message()))?
			.to_string())
	}

	fn id(&self) -> Result<String> {
		self.id().map_err(|err| Error::Other(err.message()))
	}

	fn connected(&self) -> bool {
		self.inner().connected
	}

	fn axis(&self, axis: Axis) -> f64 {
		let current = self.inner().wgi_gamepad.GetCurrentReading().unwrap();
		match axis {
			Axis::LeftStickX => current.LeftThumbstickX,
			Axis::LeftStickY => current.LeftThumbstickY,
			Axis::RightStickX => current.RightThumbstickX,
			Axis::RightStickY => current.RightThumbstickY,
			Axis::LeftTrigger => current.LeftTrigger,
			Axis::RightTrigger => current.RightTrigger,
		}
	}

	fn button(&self, button: Button) -> bool {
		let current = self.inner().wgi_gamepad.GetCurrentReading().unwrap();
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

	fn vibration(&self) -> Vibration {
		self.inner()
			.wgi_gamepad
			.Vibration()
			.map(|vibration| Vibration {
				left: vibration.LeftMotor,
				right: vibration.RightMotor,
				left_trigger: vibration.LeftTrigger,
				right_trigger: vibration.RightTrigger,
			})
			.unwrap_or_default()
	}

	fn set_vibration(&self, vibration: Vibration) {
		self.inner()
			.wgi_gamepad
			.SetVibration(GamepadVibration {
				LeftMotor: vibration.left,
				RightMotor: vibration.right,
				LeftTrigger: vibration.left_trigger,
				RightTrigger: vibration.right_trigger,
			})
			.unwrap();
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WgiBackendGamepadInner {
	connected: bool,
	raw_game_controller: RawGameController,
	wgi_gamepad: WgiGamepad,
}
