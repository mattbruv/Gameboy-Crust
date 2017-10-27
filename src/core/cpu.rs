use core::register::*;
use core::interconnect::*;

pub struct CPU {
	regs: Registers
}

impl CPU {

	// Initialize CPU state
	pub fn new() -> CPU {
		CPU {
			regs: Registers::new()
		}
	}

	// Perform one step of the fetch-decode-execute cycle 
	pub fn step(&mut self, memory: &mut Interconnect) -> usize {

		let opcode = self.next_byte(memory);
		hex!(opcode);

		0
	}

	// Reads the next byte and increments the program counter
	fn next_byte(&mut self, memory: &Interconnect) -> u8 {
		let byte = memory.read(self.regs.pc);
		self.regs.pc = self.regs.pc.wrapping_add(1);
		byte
	}

	pub fn debug(&mut self) {
		println!("{}", self.regs);
	}

}