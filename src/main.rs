mod core;
mod emu;

use std::env;
use core::*;
use emu::*;

fn main() {

	let mut args = env::args();
	let rom_path = args.nth(1).expect("No ROM Path Given");
	let rom = rom::Rom::load(rom_path);

	println!("{}", rom);

	let mut emulator = emulator::Emulator::new(rom);
	emulator.run();
}