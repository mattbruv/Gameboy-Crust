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
		loop {
			self.gameboy.step();

			if self.gameboy.cpu.regs.pc == 0x29FA {
			//	break;
			}
		}

		self.gameboy.cpu.debug();
	}
}