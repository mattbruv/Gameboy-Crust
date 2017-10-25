use core::mbc::*;

pub struct MBC1 {
	rom_bank: u8
}

impl MBC1 {
	pub fn new() -> MBC1 {
		MBC1 {
			rom_bank: 0
		}
	}
}

impl MemoryController for MBC1 {
	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
		println!("Read from MBC1");
		bytes[address as usize]
	}
	fn write(&self, address: u16, data: u8) {
		println!("Write to MBC1");
	}
}