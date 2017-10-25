// Sprite table is 160 bytes large
pub const OAM_SIZE: usize = 160;

pub struct Oam {
	bytes: Vec<u8>
}

impl Oam {
	pub fn new() -> Oam {
		Oam {
			bytes: vec![0; OAM_SIZE]
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