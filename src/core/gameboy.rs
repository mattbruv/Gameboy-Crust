use core::cpu::*;
use core::rom::*;
use core::interconnect::*;
use core::sink::*;

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

	// Steps the entire machine through the next instruction and returns cycles taken
	pub fn step(&mut self, video_sink: &mut VideoSink) -> usize {
		let cycles = self.cpu.step(&mut self.interconnect) * 4;
		self.interconnect.cycles(cycles, video_sink);
		cycles
	}
}
