use core::mbc::*;
use core::memory_map::*;
use core::helper::*;

enum ModeSelect {
	Rom,
	Ram
}

pub struct MBC1 {
    title: String,
	rom_bank: u8,
	ram_bank: u8,
	ram_enabled: bool,
	mode: ModeSelect,
	eram: Vec<u8>,
}

impl MBC1 {
	pub fn new() -> MBC1 {
		MBC1 {
            title: "".to_owned(),
			rom_bank: 0x01,
			ram_bank: 0x00,
			ram_enabled: false,
			mode: ModeSelect::Rom,
			eram: vec![0; 0x8000], // 32Kb
		}
	}

    fn adjust_rom_bank(&mut self) {
        self.rom_bank = match self.rom_bank {
            0x00 | 0x20 | 0x40 | 0x60 => {
                self.rom_bank + 1
            },
            _ => self.rom_bank
        }
    }
}

impl MemoryController for MBC1 {

	fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
		match address {
			ROM_START ... ROM_END => {
				bytes[address as usize]
			},
			ROM_BANK_START ... ROM_BANK_END => {
				let index = address - ROM_BANK_START;
				let offset = (0x4000 as u32 * self.rom_bank as u32) + index as u32;
				bytes[offset as usize]
			},
			ERAM_START ... ERAM_END => {
                if !self.ram_enabled { return 0xFF; }
                let index = address - ERAM_START;
				let offset = (0x2000 as u32 * self.ram_bank as u32) + index as u32;
				self.eram[offset as usize]
			},
			_ => unreachable!(),
		}
	}

	fn write(&mut self, address: u16, data: u8) {
		match address {
			// RAM Toggle
			0x0000 ... 0x1FFF => {
				let bits = data & 0xF;
				self.ram_enabled = match bits {
					0x0A => true,
					_    => false,
				};
			},
			// ROM bank number
			0x2000 ... 0x3FFF => {
				// extract lower 5 bits
				let bank_number = data & 0x1F;
				self.rom_bank = (self.rom_bank & 0xE0) | bank_number;
                self.adjust_rom_bank();
            },
			// RAM bank number OR upper ROM bank bits depending on mode
			0x4000 ... 0x5FFF => {
				let bank_id = data & 0b11;
				match self.mode {
					// we are specifying bits 5/6 of ROM bank
					ModeSelect::Rom => {
						self.rom_bank = self.rom_bank | (bank_id << 5); 
                        self.adjust_rom_bank();
					},
					ModeSelect::Ram => {
						self.ram_bank = bank_id;
					},
				}

			},
			// ROM/RAM mode select
			0x6000 ... 0x7FFF => {
				self.mode = match data & 1 {
					0 => ModeSelect::Rom,
					_ => ModeSelect::Ram,
				};
			},
			// ERAM writes
			ERAM_START ... ERAM_END => {
				if !self.ram_enabled { return; }
				let index = address - ERAM_START;
				let offset = (0x2000 as u32 * self.ram_bank as u32) + index as u32;
				self.eram[offset as usize] = data;
			}
			_ => unreachable!(),
		}
	}

    fn set_title(&mut self, name: String) {
        self.title = name;
    }

    fn load(&mut self) {
        let mut title = self.title.clone();
        title.push_str(".sav");
        load(title, &mut self.eram);
    }
}

impl Drop for MBC1 {
    fn drop(&mut self) {
        let mut filename = self.title.clone();
        filename.push_str(".sav");
        dump(&filename, &self.eram);
    }
}
