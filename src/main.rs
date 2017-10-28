mod core;

use std::env;
use core::*;
use std::i8::*;

fn main() {

	let mut args = env::args();
	let rom_path = args.nth(1).expect("No ROM Path Given");

	let rom = rom::Rom::load(rom_path);
	println!("{}", rom);
	let mut gbc = gameboy::GameBoy::new(rom);

	for i in 1..1000 {
		//gbc.cpu.step(&mut gbc.interconnect);
	}

	let foo = gbc.interconnect.read(0xFFFF);
	println!("{:02X}", foo);
	gbc.cpu.debug(&mut gbc.interconnect);

}