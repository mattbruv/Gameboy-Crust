use core::interconnect::*;

enum Flag {
	Zero      = 0b10000000,
	Sub       = 0b01000000,
	HalfCarry = 0b00100000,
	Carry     = 0b00010000,
}

pub struct CPU {

	// 8bit registers
	reg_a: u8,
	reg_b: u8,
	reg_c: u8,
	reg_d: u8,
	reg_e: u8,
	reg_f: u8,
	reg_h: u8,
	reg_l: u8,

	// 16bit registers
	reg_sp: u16,
	reg_pc: u16,

}

impl CPU {

	// Initialize CPU state
	pub fn new() -> CPU {
		CPU {
			reg_a: 0x01,
			reg_b: 0x00,
			reg_c: 0x13,
			reg_d: 0x00,
			reg_e: 0xD8,
			reg_f: 0xB0,
			reg_h: 0x01,
			reg_l: 0x4D,
			reg_sp: 0xFFFE,
			reg_pc: 0x0150,
		}
	}

	// Perform one step of the fetch-decode-excecute cycle 
	pub fn step(&mut self, interconnect: &mut Interconnect) -> usize {

		0

	}

	pub fn debug(&mut self) {
		self.printr(self.reg_a, "a");
		self.test(&mut 8);
	}

	fn test(&mut self, reg: &mut u8) {
		println!("SOMETHING");
	}

	fn printr(&self, register: u8, reg_name: &str) {
		println!("reg_{}: ${:02X} | b{:08b}", reg_name,	register, register);
	}

	fn printw(&self, register: u16, reg_name: &str) {
		println!("reg_{}: ${:04X} | b{:16b}", reg_name, register, register);
	}

	// Sets a flag in F register based on an expression
	fn set_flag(&mut self, flag: Flag, condition: bool) {
		if condition {
			self.reg_f |= flag as u8;
		}
		else {
			self.reg_f &= !(flag as u8);
		}
	}

}