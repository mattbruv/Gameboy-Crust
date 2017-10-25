// 352 bytes of High RAM
pub const HRAM_SIZE: usize = 352;

pub struct Hram {
	bytes: Vec<u8>
}

impl Hram {
	pub fn new() -> Hram {
		Hram {
			bytes: vec![0; HRAM_SIZE]
		}
	}

	pub fn size(&self) -> usize {
		self.bytes.len()
	}

	pub fn read(&self, address: u16) -> u8 {
		self.bytes[address as usize]
	}

	pub fn write(&mut self, address: u16, data: u8) {
		self.bytes[address as usize] = data;
	}
}