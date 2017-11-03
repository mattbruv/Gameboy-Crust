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

		let mut tile_window: Option<Window> = None;

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

				self.vram_loop(&mut tile_window);
			}

			thread::sleep(time::Duration::from_millis(10));
			//let IF = self.gameboy.interconnect.interrupt.IF.get();
			//let IE = self.gameboy.interconnect.interrupt.IE.get();
			//let STAT = self.gameboy.interconnect.gpu.STAT.get();

			//println!("IF: {:08b}", IF);
			//println!("IE: {:08b}", IE);
			//println!("STAT: {:08b}", STAT);
		}

		self.gameboy.cpu.debug();
	}

	fn vram_loop(&mut self, window: &mut Option<Window>) {
		let mut close = false;
		if let Some(vram) = window.as_mut() {
			if vram.is_open() {
				let buffer = self.gameboy.interconnect.gpu.get_tiles();
				vram.update_with_buffer(buffer.as_slice()).unwrap();
			} else {
				close = true;
			}
		}
		if close {
			*window = None;
		}
	}

	fn toggle_vram(&self, tile_viewer: &mut Option<Window>) {
		if tile_viewer.is_some() {
			println!("Closing VRAM window");
			*tile_viewer = None
		} else {
			println!("Opening VRAM window");
			*tile_viewer = Some(Window::new("VRAM Tile Viewer", 128, 192, WindowOptions {
				borderless: false,
				title: true,
				resize: false,
				scale: Scale::X4,
			}).unwrap());
		}
	}
}
