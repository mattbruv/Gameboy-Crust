use memory_map::*;

// Each bank is 4KB, 2 banks in GB, 8 banks in CGB, 32KB Total
pub const WRAM_SIZE: usize = 32768;

pub struct Wram {
	bytes: Vec<u8>,
    ram_bank: u8,
}

impl Wram {
	pub fn new() -> Wram {
		Wram {
			bytes: vec![0; WRAM_SIZE],
            ram_bank: 0,
		}
	}

	pub fn size(&self) -> usize {
		self.bytes.len()
	}

	pub fn read(&self, address: u16) -> u8 {
        match address {
            0xC000 ... 0xCFFF => {
                let rel_address = address - 0xC000;
		        self.bytes[rel_address as usize]
            },
            0xD000 ... 0xDFFF => {
                let rel_address = address - 0xD000;
                let mut bank = self.ram_bank;
                if bank == 0 { bank = 1; }
                if bank > 7 { panic!("wram bank > 7!"); }

                let offset = (bank as u16 * 0x1000) + rel_address;
                self.bytes[offset as usize]
            },
            _ => unreachable!("Addr: ${:04X}", address),
        }
	}

	pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0xC000 ... 0xCFFF => {
                let rel_address = address - 0xC000;
		        self.bytes[rel_address as usize] = data;
            },
            0xD000 ... 0xDFFF => {
                let rel_address = address - 0xD000;
                let mut bank = self.ram_bank;
                if bank == 0 { bank = 1; }
                if bank > 7 { panic!("wram bank > 7!"); }

                let offset = (bank as u16 * 0x1000) + rel_address;
                self.bytes[offset as usize] = data;
            },
            _ => unreachable!(),
        }
	}

    // SVBK read
    pub fn get_ram_bank(&self) -> u8 {
        self.ram_bank
	}

    // SVBK write
    pub fn set_ram_bank(&mut self, data: u8) {
        self.ram_bank = data;
	}
}
