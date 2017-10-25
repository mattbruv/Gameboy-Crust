use core::mbc::*;

pub struct MBC0;

impl MemoryController for MBC0 {
	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
		bytes[address as usize]
	}
	fn write(&self, address: u16, data: u8) {}
}