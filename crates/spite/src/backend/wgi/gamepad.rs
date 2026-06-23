use std::sync::{Arc, Mutex, MutexGuard};

use windows::Gaming::Input::{Gamepad as WgiGamepad, GamepadButtons, RawGameController};

use crate::{Axis, Button, Error, Result, backend::GamepadTrait};

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

	fn axis(&self, axis: Axis) -> f32 {
		let current = self.inner().wgi_gamepad.GetCurrentReading().unwrap();
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WgiBackendGamepadInner {
	connected: bool,
	raw_game_controller: RawGameController,
	wgi_gamepad: WgiGamepad,
}
