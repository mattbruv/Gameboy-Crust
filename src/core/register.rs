use std::fmt;
use core::helper::*;

enum Flag {
	Zero      = 0b10000000,
	Sub       = 0b01000000,
	HalfCarry = 0b00100000,
	Carry     = 0b00010000,
}

pub struct Registers {
	// 8bit registers
	pub a: u8,
	pub f: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub h: u8,
	pub l: u8,

	// 16bit registers
	pub sp: u16,
	pub pc: u16,
}

impl Registers {

	pub fn new() -> Registers {
		// Registers are set to specific values after GB BIOS runs
		Registers {
			a: 0x01,
			f: 0xB0,
			b: 0x00,
			c: 0x13,
			d: 0x00,
			e: 0xD8,
			h: 0x01,
			l: 0x4D,
			sp: 0xFFFE,
			pc: 0x0150,
		}
	}

	pub fn af(&self) -> u16 {
		combine!(self.a, self.f)
	}

	pub fn bc(&self) -> u16 {
		combine!(self.b, self.c)
	}

	pub fn de(&self) -> u16 {
		combine!(self.d, self.e)
	}

	pub fn hl(&self) -> u16 {
		combine!(self.h, self.l)
	}

	pub fn set_af(&mut self, value: u16) {
		self.a = high!(value);
		self.f = low!(value);
	}

	pub fn set_bc(&mut self, value: u16) {
		self.b = high!(value);
		self.c = low!(value);
	}

	pub fn set_de(&mut self, value: u16) {
		self.d = high!(value);
		self.e = low!(value);
	}

	pub fn set_hl(&mut self, value: u16) {
		self.h = high!(value);
		self.l = low!(value);
	}

	// Sets the state of a flag in the F register based on a condition
	fn set_flag(&mut self, flag: Flag, condition: bool) {
		if condition {
			self.f |= flag as u8;
		}
		else {
			self.f &= !(flag as u8);
		}
	}

}

impl fmt::Display for Registers {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Registers:\nAF: ${:02X}{:02X}\nBC: ${:02X}{:02X}\nDE: ${:02X}{:02X}\nHL: ${:02X}{:02X}\nSP: ${:04X}\nPC: ${:04X}", 
			self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc)
	}
}