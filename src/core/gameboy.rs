use core::cpu::*;
use core::rom::*;
use core::interconnect::*;

pub struct GameBoy {
	pub interconnect: Interconnect,
	pub cpu: CPU
}

impl GameBoy {
	pub fn new(rom: Rom) -> GameBoy {
		GameBoy {
			interconnect: Interconnect::new(rom),
			cpu: CPU::new()
		}
	}
}