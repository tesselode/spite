use crate::Gamepad;

pub enum Event {
	GamepadAdded(Gamepad),
	GamepadRemoved(Gamepad),
}
