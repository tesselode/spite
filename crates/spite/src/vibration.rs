#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vibration {
	pub left: f64,
	pub right: f64,
	pub left_trigger: f64,
	pub right_trigger: f64,
}
