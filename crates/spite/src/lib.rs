use std::time::Instant;

use ::windows::Gaming::Input::Gamepad as WgiGamepad;
use windows::{
	Gaming::Input::{GamepadReading, RawGameController},
	core::Interface,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gamepad(WgiGamepad);

impl Gamepad {
	pub fn read(&self) -> GamepadReading {
		let a = Instant::now();
		let raw = RawGameController::FromGameController(&self.0).unwrap();
		let b = Instant::now();
		println!("{:#?}", b - a);
		dbg!(raw.DisplayName().unwrap());
		self.0.GetCurrentReading().unwrap()
	}
}

pub fn gamepads() -> Vec<Gamepad> {
	WgiGamepad::Gamepads()
		.unwrap()
		.into_iter()
		.map(Gamepad)
		.collect()
}
