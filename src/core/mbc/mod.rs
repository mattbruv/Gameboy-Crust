pub mod mbc0;
pub mod mbc1;
pub mod mbc2;
pub mod mbc3;

pub trait MemoryController {
	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8;
	fn write(&self, address: u16, data: u8);
}