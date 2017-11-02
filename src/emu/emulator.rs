use core::gameboy::*;
use core::rom::*;
use core::sink::*;
use minifb::{Key, WindowOptions, Window, Scale};
use std::{thread, time};

const CLOCK_SPEED: i32 = 4194304;
const FRAME_RATE: i32 = 60;

pub struct Emulator {

	gameboy: GameBoy,
	tile_viewer: Window,

}

impl Emulator {

	pub fn new(rom: Rom) -> Emulator {
		Emulator {
			gameboy: GameBoy::new(rom),
			tile_viewer: Window::new("VRAM Tile Viewer", 128, 192, WindowOptions {
				borderless: false,
				title: true,
				resize: false,
				scale: Scale::X4,
			}).unwrap()
		}
	}

	pub fn run(&mut self) {

		while self.tile_viewer.is_open() && !self.tile_viewer.is_key_down(Key::Escape) {

			let mut video_sink = VideoSink::new();

			let cycles_per_frame = CLOCK_SPEED / FRAME_RATE;
			let mut emulated_cycles = 0;

			while emulated_cycles <= cycles_per_frame {
				emulated_cycles += self.gameboy.step(&mut video_sink) as i32;
			}

			if let Some(frame) = video_sink.consume() {
				let buffer = self.gameboy.interconnect.gpu.get_tiles();
				self.tile_viewer.update_with_buffer(&buffer);
				self.gameboy.interconnect.gpu.refresh_tile(0);
			}

			thread::sleep(time::Duration::from_millis(10));
		}

		//self.gameboy.interconnect.gpu.dump();
		self.gameboy.cpu.debug();
	}
}
