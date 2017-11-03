use core::gameboy::*;
use core::rom::*;
use core::sink::*;
use minifb::{Key, WindowOptions, Window, Scale, KeyRepeat};
use std::{thread, time};

const CLOCK_SPEED: i32 = 4194304;
const FRAME_RATE: i32 = 60;

pub struct Emulator {
	gameboy: GameBoy,
	window: Window,
}

impl Emulator {

	pub fn new(rom: Rom) -> Emulator {
		let mut title = "Gameboy Crust - ".to_owned();
		title.push_str(&rom.name());
		Emulator {
			gameboy: GameBoy::new(rom),
			window: Window::new(title.as_str(), 160, 144, WindowOptions {
				borderless: false,
				title: true,
				resize: false,
				scale: Scale::X4,
			}).unwrap(),
		}
	}

	pub fn run(&mut self) {

		let mut tile_window: Option<Box<Window>> = None;

		while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
			let mut video_sink = VideoSink::new();
			let cycles_per_frame = CLOCK_SPEED / FRAME_RATE;
			let mut emulated_cycles = 0;

			while emulated_cycles <= cycles_per_frame {
				emulated_cycles += self.gameboy.step(&mut video_sink) as i32;
			}

			if let Some(frame) = video_sink.consume() {
				self.window.update_with_buffer(frame.as_slice()).unwrap();

				if self.window.is_key_pressed(Key::V, KeyRepeat::No) {
					self.toggle_vram(&mut tile_window);
				}

				match tile_window.as_mut() {
					Some(window) => {
						let buffer = self.gameboy.interconnect.gpu.get_tiles();
						window.update_with_buffer(&buffer).unwrap();
					},
					None => {},
				}
			}

			thread::sleep(time::Duration::from_millis(10));
		}

		self.gameboy.cpu.debug();
	}

	fn toggle_vram(&self, tile_viewer: &mut Option<Box<Window>>) {
		if tile_viewer.is_some() {
			println!("Closing VRAM window");
			*tile_viewer = None
		} else {
			println!("Opening VRAM window");
			*tile_viewer = Some(Box::new(Window::new("VRAM Tile Viewer", 128, 192, WindowOptions {
				borderless: false,
				title: true,
				resize: false,
				scale: Scale::X4,
			}).unwrap()));
		}
	}
}
