use std::{
	collections::VecDeque,
	sync::{Arc, Mutex},
};

use windows::{
	Foundation::EventHandler,
	Gaming::Input::{Gamepad as WgiGamepad, GamepadButtons, RawGameController},
	core::Ref,
};

use crate::{
	Axis, Button, Error, Gamepad, Result,
	backend::{Backend, GamepadTrait},
	event::Event,
};

pub struct WgiBackend {
	event_queue: Arc<Mutex<VecDeque<Event>>>,
	gamepad_added_event_handler_token: i64,
	gamepad_removed_event_handler_token: i64,
}

impl WgiBackend {
	pub fn new() -> Result<Self> {
		let event_queue = Arc::new(Mutex::new(VecDeque::new()));

		let event_queue_clone = event_queue.clone();
		let gamepad_added_event_handler =
			EventHandler::new(move |_, gamepad: Ref<'_, WgiGamepad>| {
				let Some(gamepad) = gamepad.as_ref() else {
					return Ok(());
				};
				let mut event_queue = event_queue_clone.lock().unwrap();
				let gamepad = Gamepad::from_backend_gamepad(gamepad.clone());
				let event = Event::GamepadAdded(gamepad);
				event_queue.push_back(event);
				Ok(())
			});
		let gamepad_added_event_handler_token =
			WgiGamepad::GamepadAdded(&gamepad_added_event_handler)
				.map_err(|err| Error::Other(err.message()))?;

		let event_queue_clone = event_queue.clone();
		let gamepad_removed_event_handler =
			EventHandler::new(move |_, gamepad: Ref<'_, WgiGamepad>| {
				let Some(gamepad) = gamepad.as_ref() else {
					return Ok(());
				};
				let mut event_queue = event_queue_clone.lock().unwrap();
				let gamepad = Gamepad::from_backend_gamepad(gamepad.clone());
				let event = Event::GamepadRemoved(gamepad);
				event_queue.push_back(event);
				Ok(())
			});
		let gamepad_removed_event_handler_token =
			WgiGamepad::GamepadRemoved(&gamepad_removed_event_handler)
				.map_err(|err| Error::Other(err.message()))?;

		Ok(Self {
			event_queue,
			gamepad_added_event_handler_token,
			gamepad_removed_event_handler_token,
		})
	}
}

impl Backend for WgiBackend {
	type Gamepad = WgiGamepad;

	fn gamepads(&self) -> Result<Vec<Box<dyn GamepadTrait>>> {
		Ok(WgiGamepad::Gamepads()
			.map_err(|err| Error::Other(err.message()))?
			.into_iter()
			.map(|gamepad| Box::new(gamepad) as Box<dyn GamepadTrait>)
			.collect())
	}

	fn pop_event(&self) -> Option<Event> {
		self.event_queue.lock().unwrap().pop_front()
	}
}

impl Drop for WgiBackend {
	fn drop(&mut self) {
		WgiGamepad::RemoveGamepadAdded(self.gamepad_added_event_handler_token).ok();
		WgiGamepad::RemoveGamepadRemoved(self.gamepad_removed_event_handler_token).ok();
	}
}

impl GamepadTrait for WgiGamepad {
	fn name(&self) -> Result<String> {
		let raw = RawGameController::FromGameController(self)
			.map_err(|err| Error::Other(err.message()))?;
		raw.DisplayName()
			.map(|hstring| hstring.to_string())
			.map_err(|err| Error::Other(err.message()))
	}

	fn axis(&self, axis: Axis) -> f32 {
		let current = self.GetCurrentReading().unwrap();
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
		let current = self.GetCurrentReading().unwrap();
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
