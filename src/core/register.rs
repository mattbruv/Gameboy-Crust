use std::fmt;
use core::helper::*;

pub enum Flag {
	Zero      = 0b10000000,
	Sub       = 0b01000000,
	HalfCarry = 0b00100000,
	Carry     = 0b00010000,
}

#[derive(Copy, Clone)]
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
		// Registers are set to these specific values after GB BIOS runs
		Registers {
			a: 0x11,
			f: 0xB0,
			b: 0x00,
			c: 0x13,
			d: 0x00,
			e: 0xD8,
			h: 0x01,
			l: 0x4D,
			sp: 0xFFFE,
			pc: 0x0100,
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

	pub fn hli(&mut self) {
		let new = self.hl().wrapping_add(1);
		self.set_hl(new);
	}

	pub fn hld(&mut self) {
		let new = self.hl().wrapping_sub(1);
		self.set_hl(new);
	}

	// Sets the state of a flag in the F register based on a condition
	pub fn set_flag(&mut self, flag: Flag, condition: bool) {
		if condition {
			self.f |= flag as u8;
		}
		else {
			self.f &= !(flag as u8);
		}
	}

	pub fn is_flag_set(&self, flag: Flag) -> bool {
		match self.f & flag as u8 {
			0 => false,
			_ => true
		}
	}

}

impl fmt::Display for Registers {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Registers:
AF: ${:02X}{:02X}
BC: ${:02X}{:02X}
DE: ${:02X}{:02X}
HL: ${:02X}{:02X}
SP: ${:04X}
PC: ${:04X}
Flags: {}{}{}{}",
			self.a, self.f,
			self.b, self.c,
			self.d, self.e,
			self.h, self.l,
			self.sp,
			self.pc,
			match (self.f & Flag::Zero as u8)
			{
				0 => "",
				_ => "Zero | "
			},
			match (self.f & Flag::Sub as u8)
			{
				0 => "",
				_ => "Sub | "
			},
			match (self.f & Flag::HalfCarry as u8)
			{
				0 => "",
				_ => "HalfCarry | "
			},
			match (self.f & Flag::Carry as u8)
			{
				0 => "",
				_ => "Carry"
			},
		)
	}
}
