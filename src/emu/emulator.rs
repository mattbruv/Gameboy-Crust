use core::gameboy::*;
use core::rom::*;
use core::sink::*;
use core::joypad::*;
use minifb::{Key, WindowOptions, Window, Scale, KeyRepeat};
use std::thread;
use std::time::{Duration, Instant};

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
		let mut multiplier = 10;
		let mut overclock = false;

		while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

			let start_time = Instant::now();
			let frame_time = Duration::new(0, 16600000); // 16.6 ms as nanoseconds

			let mut video_sink = VideoSink::new();
			let mut clock_speed = CLOCK_SPEED;
			if overclock {
				clock_speed *= multiplier;
			}
			let cycles_per_frame = clock_speed / FRAME_RATE;
			let mut emulated_cycles = 0;

			while emulated_cycles <= cycles_per_frame {
				emulated_cycles += self.gameboy.step(&mut video_sink) as i32;
			}

			if let Some(frame) = video_sink.consume() {
				self.window.update_with_buffer(frame.as_slice()).unwrap();
				if self.window.is_key_pressed(Key::V, KeyRepeat::No) {
					self.toggle_vram(&mut tile_window);
				}
				if self.window.is_key_pressed(Key::D, KeyRepeat::No) {
					self.debug();
				}
				overclock = self.window.is_key_down(Key::Space);
				self.read_input();
				self.vram_loop(&mut tile_window);
			}

			// We have done our calculations, wait the remaning time
			let elapsed_time = start_time.elapsed();
			if !(elapsed_time > frame_time) {
				let remaining_time = frame_time - elapsed_time;
				thread::sleep(remaining_time);
			}
		}

		self.gameboy.cpu.debug();
	}

	fn read_input(&mut self) {
		let i = &mut self.gameboy.interconnect.interrupt;
		self.gameboy.interconnect.joypad.set_direction_pressed(i, PAD_UP, self.window.is_key_down(Key::Up));
		self.gameboy.interconnect.joypad.set_direction_pressed(i, PAD_DOWN, self.window.is_key_down(Key::Down));
		self.gameboy.interconnect.joypad.set_direction_pressed(i, PAD_LEFT, self.window.is_key_down(Key::Left));
		self.gameboy.interconnect.joypad.set_direction_pressed(i, PAD_RIGHT, self.window.is_key_down(Key::Right));
		self.gameboy.interconnect.joypad.set_button_pressed(i, BUTTON_A, self.window.is_key_down(Key::A));
		self.gameboy.interconnect.joypad.set_button_pressed(i, BUTTON_B, self.window.is_key_down(Key::S));
		self.gameboy.interconnect.joypad.set_button_pressed(i, BUTTON_START, self.window.is_key_down(Key::Z));
		self.gameboy.interconnect.joypad.set_button_pressed(i, BUTTON_SELECT, self.window.is_key_down(Key::X));
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

	fn debug(&self) {
		self.gameboy.interconnect.gpu.dump();
	}
}
