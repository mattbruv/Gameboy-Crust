use core::mbc::*;

pub struct MBC3 {
	rom_bank: u8
}

impl MBC3 {
	pub fn new() -> MBC3 {
		MBC3 {
			rom_bank: 0
		}
	}
}

impl MemoryController for MBC3 {
	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
		bytes[address as usize]
	}
	fn write(&mut self, address: u16, data: u8) {
		unimplemented!();
	}
}