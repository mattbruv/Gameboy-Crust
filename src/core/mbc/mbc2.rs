use core::mbc::*;

pub struct MBC2 {
	rom_bank: u8
}

impl MBC2 {
	pub fn new() -> MBC2 {
		MBC2 {
			rom_bank: 0
		}
	}
}

impl MemoryController for MBC2 {
	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
		bytes[address as usize]
	}
	fn write(&self, address: u16, data: u8) {
		unimplemented!();
	}
}