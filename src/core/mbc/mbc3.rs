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
		println!("Read from MBC3");
		bytes[address as usize]
	}
	fn write(&self, address: u16, data: u8) {
		println!("Write to MBC3");
	}
}