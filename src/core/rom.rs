use std::string::String;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::fmt;
use core::mbc::*;

pub struct Rom {
	bytes: Vec<u8>,
	controller: Box<MemoryController>
}

impl Rom {

	// Load a ROM image from a given path
	pub fn load(path: String) -> Rom {
		let mut buffer = Vec::new();
		let mut file = File::open(path).expect("Invalid ROM path");
		file.read_to_end(&mut buffer).expect("Unable to read ROM");
		let cart_type = buffer[0x147];

		Rom {
			controller: match cart_type {
				0x00 => Box::new(mbc0::MBC0),
				0x01 ... 0x03 => Box::new(mbc1::MBC1::new()),
				0x05 ... 0x06 => Box::new(mbc2::MBC2::new()),
				0x0F ... 0x13 => Box::new(mbc3::MBC3::new()),
				_ => panic!("Unsupported Cartridge Type: ${:02X}", cart_type)
			},
			bytes: buffer
		}
	}

	pub fn read(&self, address: u16) -> u8 {
		self.controller.read(&self.bytes, address)
	}

	pub fn write(&self, address: u16, data: u8) {
		self.controller.write(address, data);
	}

	pub fn size(&self) -> usize {
		self.bytes.len()
	}

	pub fn cart_type(&self) -> u8 {
		self.read(0x147)
	}

	pub fn cgb_flag(&self) -> u8 {
		self.read(0x143)
	}

	pub fn sgb_flag(&self) -> u8 {
		self.read(0x146)
	}

	pub fn rom_size(&self) -> u8 {
		self.read(0x148)
	}

	pub fn ram_size(&self) -> u8 {
		self.read(0x149)
	}

	pub fn name(&self) -> String {
		let mut name = String::new();
		for index in 0x134..0x143 {
			let code = self.read(index);
			match code {
				0 => break,
				_ => name.push(code as char)
			}
		}
		name
	}

}

impl fmt::Display for Rom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,

"== ROM HEADER ==
Title: {}
CGB Flag: ${:02X} - {}
SGB Flag: ${:02X} - {}
ROM Size: ${:02X} - {}
RAM Size: ${:02X} - {}
Destination Code: {}
Cartridge Type: ${:02X} - {}
== END ROM HEADER ==",

			self.name(),
			self.cgb_flag(),
			match self.cgb_flag() {
				0x80 => "Game supports CGB functions, but works on GB also",
				0xC0 => "Game only works on CGB",
				_ => "Pre-GBC game"
			},
			self.sgb_flag(),
			match self.sgb_flag() {
				0x00 => "No SGB functions (Normal Gameboy or CGB only)",
				0x03 => "Game supports SGB functions",
				_ => ""
			},
			self.rom_size(),
			match self.rom_size() {
				0x00 => "32KByte (no ROM banking)",
				0x01 => "64KByte (4 banks)",
				0x02 => "128KByte (8 banks)",
				0x03 => "256KByte (16 banks)",
				0x04 => "512KByte (32 banks)",
				0x05 => "1MByte (64 banks) - only 63 banks used by MBC1",
				0x06 => "2MByte (128 banks) - only 125 banks used by MBC1",
				0x07 => "4MByte (256 banks)",
				0x52 => "1.1MByte (72 banks)",
				0x53 => "1.2MByte (80 banks)",
				0x54 => "1.5MByte (96 banks)",
				_ => "UNKNOWN ROM SIZE"
			},
			self.ram_size(),
			match self.ram_size() {
				0x00 => "None",
				0x01 => "2 KBytes",
				0x02 => "8 Kbytes",
				0x03 => "32 KBytes (4 banks of 8KBytes each)",
				_ => "UNKNOWN RAM SIZE"
			},
			match self.read(0x14A) {
				0x00 => "Japanese",
				0x01 => "Non-Japanese",
				_ => "UNKNOWN DEST CODE"
			},
			self.cart_type(),
			match self.cart_type() {
				0x00 => "ROM ONLY",
				0x01 => "MBC1",
				0x02 => "MBC1+RAM",
				0x03 => "MBC1+RAM+BATTERY",
				0x05 => "MBC2",
				0x06 => "MBC2+BATTERY",
				0x08 => "ROM+RAM",
				0x09 => "ROM+RAM+BATTERY",
				0x0B => "MMM01",
				0x0C => "MMM01+RAM",
				0x0D => "MMM01+RAM+BATTERY",
				0x0F => "MBC3+TIMER+BATTERY",
				0x10 => "MBC3+TIMER+RAM+BATTERY",
				0x11 => "MBC3",
				0x12 => "MBC3+RAM",
				0x13 => "MBC3+RAM+BATTERY",
				0x15 => "MBC4",
				0x16 => "MBC4+RAM",
				0x17 => "MBC4+RAM+BATTERY",
				0x19 => "MBC5",
				0x1A => "MBC5+RAM",
				0x1B => "MBC5+RAM+BATTERY",
				0x1C => "MBC5+RUMBLE",
				0x1D => "MBC5+RUMBLE+RAM",
				0x1E => "MBC5+RUMBLE+RAM+BATTERY",
				0xFC => "POCKET CAMERA",
				0xFD => "BANDAI TAMA5",
				0xFE => "HuC3",
				0xFF => "HuC1+RAM+BATTERY",
				_ => "UNKNOWN CARTRIDGE TYPE"
			}
		)
	}
}
