mod core;

use std::env;
use core::*;

fn main() {

	let mut args = env::args();
	let rom_path = args.nth(1).expect("No ROM Path Given");

	let rom = rom::Rom::load(rom_path);
	println!("{}", rom);
	let mut gbc = gameboy::GameBoy::new(rom);

	let test = gbc.interconnect.read(0xC001);
	println!("${:02X}", test);

}