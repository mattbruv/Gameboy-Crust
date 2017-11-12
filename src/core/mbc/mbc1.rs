use core::mbc::*;
use core::memory_map::*;

enum ModeSelect {
	Rom,
	Ram
}

pub struct MBC1 {
	rom_bank: u8,
	ram_bank: u8,
	ram_enabled: bool,
	mode: ModeSelect,
	eram: Vec<u8>,
}

impl MBC1 {
	pub fn new() -> MBC1 {
		MBC1 {
			rom_bank: 0x01,
			ram_bank: 0x00,
			ram_enabled: false,
			mode: ModeSelect::Rom,
			eram: vec![0; 0x8000], // 32Kb
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
				let offset = (self.rom_bank as u16 * 0x4000) + index;
				bytes[offset as usize]
			},
			ERAM_START ... ERAM_END => {
				let index = address - ERAM_START;
				let offset = (self.ram_bank as u16 * 0x2000) + index;
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
				let mut bank_number = data & 0x1F;
				self.rom_bank = match bank_number {
					// Disallow selecting the following banks
					0x00 | 0x20 | 0x40 | 0x60 => bank_number + 1,
					_ => bank_number
				};
			},
			// RAM bank number OR upper ROM bank bits depending on mode
			0x4000 ... 0x5FFF => {
				let bits = data & 0b11;
				match self.mode {
					// we are specifying bits 5/6 of ROM bank
					ModeSelect::Rom => {
						let bank = self.rom_bank & 0x1F;
						self.rom_bank = (bits << 5) | bank;
					},
					ModeSelect::Ram => {
						self.ram_bank = bits;
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
				// RAM bank zero can always be used no matter what mode
				let index = address - ERAM_START;
				let offset = (self.ram_bank as u16 * 0x2000) + index;
				self.eram[offset as usize] = data;
			}
			_ => unreachable!(),
		}
	}
}