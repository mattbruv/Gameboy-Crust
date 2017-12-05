use core::mbc::*;

pub struct MBC0;

impl MemoryController for MBC0 {
	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
		bytes[address as usize]
	}
	fn write(&mut self, address: u16, data: u8) {}
    fn set_title(&mut self, name: String) {}
    fn load(&mut self) {}
}
