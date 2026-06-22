use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
	GamepadDisconnected,
	Other(String),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::GamepadDisconnected => f.write_str("The gamepad was disconnected."),
			Error::Other(message) => f.write_str(message),
		}
	}
}

impl std::error::Error for Error {}
