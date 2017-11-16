use std::io::prelude::*;
use std::fs::File;

pub enum Bit {
	Bit0 = 0b00000001,
	Bit1 = 0b00000010,
	Bit2 = 0b00000100,
	Bit3 = 0b00001000,
	Bit4 = 0b00010000,
	Bit5 = 0b00100000,
	Bit6 = 0b01000000,
	Bit7 = 0b10000000,
}

pub fn dump(name: &str, bytes: &Vec<u8>) {
	let mut dir = "dumps/".to_owned();
	dir.push_str(name);
	let mut file = File::create(dir).unwrap();
	file.write_all(bytes);
}

pub struct MemoryRegister {
	value: u8
}

impl MemoryRegister {
	pub fn new(data: u8) -> MemoryRegister {
		MemoryRegister {
			value: data
		}
	}

	pub fn get(&self) -> u8 {
		self.value
	}

	pub fn set(&mut self, data: u8) {
		self.value = data;
	}

	pub fn clear(&mut self) {
		self.set(0x00);
	}

	pub fn set_bit(&mut self, b: Bit) {
		self.value = self.value | b as u8;
	}

	pub fn clear_bit(&mut self, b: Bit) {
		self.value = self.value & !(b as u8);
	}

	pub fn is_set(&self, b: Bit) -> bool {
		self.value & b as u8 > 0
	}

	pub fn add(&mut self, data: u8) {
		self.value = self.value.wrapping_add(data);
	}

	pub fn sub(&mut self, data: u8) {
		self.value = self.value.wrapping_sub(data);
	}
}

// combine two u8s into a u16
macro_rules! combine {
	($h:expr, $l:expr) => (
		((($h as u16) << 8) | $l as u16)
	)
}

// return high byte from u16
macro_rules! high {
	($word:expr) => (
		(($word >> 8) as u8)
	)
}

// return low byte from u16
macro_rules! low {
	($word:expr) => (
		(($word & 0xFF) as u8)
	)
}

// print number as hexidecimal
macro_rules! hex {
	($val:expr) => {{
		println!("${:02X}", $val);
	}}
}