use std::{
	collections::{HashMap, VecDeque},
	sync::{Arc, Mutex},
};

use windows::{Foundation::EventHandler, Gaming::Input::RawGameController, core::Ref};

use crate::{
	Error, Event, Gamepad, Result,
	backend::{Backend, wgi::gamepad::WgiBackendGamepad},
};

mod gamepad;

pub struct WgiBackend {
	gamepads: Arc<Mutex<HashMap<String, WgiBackendGamepad>>>,
	event_queue: Arc<Mutex<VecDeque<Event>>>,
	added_token: i64,
	removed_token: i64,
}

impl WgiBackend {
	pub fn new() -> Result<Self> {
		let gamepads = Arc::new(Mutex::new(HashMap::<String, WgiBackendGamepad>::new()));
		let event_queue = Arc::new(Mutex::new(VecDeque::new()));

		let gamepads_clone = gamepads.clone();
		let event_queue_clone = event_queue.clone();
		let added_token = RawGameController::RawGameControllerAdded(&EventHandler::new(
			move |_, raw_game_controller: Ref<'_, RawGameController>| {
				let raw_game_controller = raw_game_controller.unwrap().clone();
				let id = raw_game_controller.NonRoamableId()?.to_string();
				let mut gamepads = gamepads_clone.lock().unwrap();
				let backend_gamepad = if let Some(backend_gamepad) = gamepads.get(&id) {
					// if the gamepad with this ID is already known, update its connection state
					backend_gamepad.set_connected(true);
					backend_gamepad.update_raw_game_controller(raw_game_controller)?;
					backend_gamepad.clone()
				} else {
					// otherwise, check if this raw controller is a gamepad...
					let Ok(backend_gamepad) = WgiBackendGamepad::new(raw_game_controller) else {
						return Ok(());
					};
					// ...and if it is, add it
					gamepads.insert(id, backend_gamepad.clone());
					backend_gamepad
				};
				event_queue_clone
					.lock()
					.unwrap()
					.push_back(Event::GamepadAdded(Gamepad::from_gamepad_trait(
						backend_gamepad,
					)));
				Ok(())
			},
		))
		.map_err(|err| Error::Other(err.message()))?;

		let gamepads_clone = gamepads.clone();
		let event_queue_clone = event_queue.clone();
		let removed_token = RawGameController::RawGameControllerRemoved(&EventHandler::new(
			move |_, raw_game_controller: Ref<'_, RawGameController>| {
				// if the gamepad with this ID is already known, mark it as disconnected
				// and emit a GamepadRemoved event
				let raw_game_controller = raw_game_controller.unwrap().clone();
				let id = raw_game_controller.NonRoamableId()?.to_string();
				let gamepads = gamepads_clone.lock().unwrap();
				let Some(backend_gamepad) = gamepads.get(&id) else {
					return Ok(());
				};
				backend_gamepad.set_connected(false);
				event_queue_clone
					.lock()
					.unwrap()
					.push_back(Event::GamepadRemoved(Gamepad::from_gamepad_trait(
						backend_gamepad.clone(),
					)));
				Ok(())
			},
		))
		.map_err(|err| Error::Other(err.message()))?;

		Ok(Self {
			gamepads,
			event_queue,
			added_token,
			removed_token,
		})
	}
}

impl Backend for WgiBackend {
	fn gamepads(&self) -> Result<Vec<Gamepad>> {
		Ok(self
			.gamepads
			.lock()
			.unwrap()
			.values()
			.map(|gamepad| Gamepad::from_gamepad_trait(gamepad.clone()))
			.collect())
	}

	fn pop_event(&self) -> Option<Event> {
		self.event_queue.lock().unwrap().pop_front()
	}
}

impl Drop for WgiBackend {
	fn drop(&mut self) {
		RawGameController::RemoveRawGameControllerAdded(self.added_token).ok();
		RawGameController::RemoveRawGameControllerRemoved(self.removed_token).ok();
	}
}
