use core::gameboy::*;
use core::rom::*;

pub struct Emulator {

	gameboy: GameBoy,
}

impl Emulator {

	pub fn new(rom: Rom) -> Emulator {
		Emulator {
			gameboy: GameBoy::new(rom),
		}
	}

	pub fn run(&mut self) {
		//for i in 1..20000000 {
		loop {
			self.gameboy.step();

			if self.gameboy.cpu.regs.pc == 0x282A {
				break;
			}
		}

		//self.gameboy.interconnect.gpu.dump();
		self.gameboy.cpu.debug();
		self.gameboy.interconnect.gpu.get_tile_map();
	}
}