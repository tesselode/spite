use std::error::Error;
use std::time::Duration;

use spite::backend::wgi::WgiBackend;
use spite::{Button, Gamepad, GamepadManager};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct App {
	window: Option<Window>,
	gamepad_manager: GamepadManager<WgiBackend>,
	gamepad: Option<Gamepad>,
}

impl App {
	fn new() -> Self {
		Self {
			window: None,
			gamepad_manager: GamepadManager::new(WgiBackend::new().unwrap()),
			gamepad: None,
		}
	}
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		self.window = Some(
			event_loop
				.create_window(Window::default_attributes())
				.unwrap(),
		);
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				if self.gamepad.is_none() {
					let mut gamepads = self.gamepad_manager.gamepads().unwrap();
					if let Some(gamepad) = gamepads.pop() {
						self.gamepad = Some(gamepad);
					}
				}
				if let Some(gamepad) = &self.gamepad {
					println!("{}", gamepad.button(Button::North));
					while let Some(event) = self.gamepad_manager.pop_event() {
						match event {
							spite::Event::GamepadAdded(gamepad) => println!("gamepad added"),
							spite::Event::GamepadRemoved(gamepad) => println!("gamepad removed"),
						}
					}
					std::thread::sleep(Duration::from_millis(50));
				}
				self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Poll);
	let mut app = App::new();
	event_loop.run_app(&mut app)?;
	Ok(())
}
