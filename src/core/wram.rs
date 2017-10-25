// Each bank is 4KB, 2 banks in GB, 8 banks in CGB, 32KB Total
pub const WRAM_SIZE: usize = 32768;

pub struct Wram {
	bytes: Vec<u8>
}

impl Wram {
	pub fn new() -> Wram {
		Wram {
			bytes: vec![0; WRAM_SIZE]
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