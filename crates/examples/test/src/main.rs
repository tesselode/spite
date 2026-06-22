use std::error::Error;
use std::time::Duration;

use spite::{Gamepad, gamepads};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
	window: Option<Window>,
	gamepad: Option<Gamepad>,
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
					let mut gamepads = gamepads();
					if let Some(gamepad) = gamepads.pop() {
						self.gamepad = Some(gamepad);
					}
				}
				if let Some(gamepad) = &self.gamepad {
					print!("\x1B[2J\x1B[1;1H");
					println!("{:#?}", gamepad.read());
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
	let mut app = App::default();
	event_loop.run_app(&mut app)?;
	Ok(())
}
