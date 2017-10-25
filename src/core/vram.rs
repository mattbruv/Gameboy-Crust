// Each bank is 8KB, 1 bank in GB, 2 banks in CGB, 16KB Total
pub const VRAM_SIZE: usize = 16384;

pub struct Vram {
	bytes: Vec<u8>
}

impl Vram {
	pub fn new() -> Vram {
		Vram {
			bytes: vec![0; VRAM_SIZE]
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