use rom::*;
use vram::*;
use wram::*;
use oam::*;
use hram::*;
use gpu::*;
use interrupt::*;
use memory_map::*;

pub struct Interconnect {
	rom: Rom,
	vram: Vram,
	wram: Wram,
	oam: Oam,
	hram: Hram,
	gpu: Gpu,
	pub interrupt: InterruptHandler,
}

impl Interconnect {
	pub fn new(_rom: Rom) -> Interconnect {
		Interconnect {
			rom: _rom,
			vram: Vram::new(),
			wram: Wram::new(),
			oam: Oam::new(),
			hram: Hram::new(),
			gpu: Gpu::new(),
			interrupt: InterruptHandler::new(),
		}
	}

	pub fn read(&self, address: u16) -> u8 {

		// Has a specific register been requested?
		if let Some(value) = self.read_registers(address) {
			return value
		}

		// No specific register, read general data
		match address {
			ROM_START  ... ROM_END  => self.rom.read(address),
			VRAM_START ... VRAM_END => self.vram.read(address - VRAM_START),
			ERAM_START ... ERAM_END => panic!("Read from ERAM not implemented"),
			WRAM_START ... WRAM_END => self.wram.read(address - WRAM_START),
			ECHO_START ... ECHO_END => self.wram.read(address - ECHO_START),
			OAM_START  ... OAM_END  => self.oam.read(address - OAM_START),
			HRAM_START ... HRAM_END => self.hram.read(address - HRAM_START),
			_ => panic!("Invalid Read")
		}
	}

	pub fn write(&mut self, address: u16, data: u8) {
		//println!("WRITE ${:2X} TO ${:4X}", data, address);
		match address {
			ROM_START  ... ROM_END  => self.rom.write(address, data),
			VRAM_START ... VRAM_END => self.vram.write(address - VRAM_START, data),
			ERAM_START ... ERAM_END => panic!("Write to ERAM not implemented"),
			WRAM_START ... WRAM_END => self.wram.write(address - WRAM_START, data),
			ECHO_START ... ECHO_END => {
				// Note: Use of the area from 0xE000 to 0xFDFF is prohibited.
				// self.wram.write(address - ECHO_START, data)
				panic!("Attempt to write to ECHO RAM");
			},
			OAM_START  ... OAM_END  => self.oam.write(address - OAM_START, data),
			HRAM_START ... HRAM_END => self.hram.write(address - HRAM_START, data),
			_ => panic!("Invalid Write")
		}
	}

	// Take the latest number of machine cycles and keep other hardware in sync
	pub fn cycles(&mut self, cycles: usize) {
		self.gpu.cycles(cycles);
	}

	// Intercept and re-route reads to memory registers to their actual location
	fn read_registers(&self, address: u16) -> Option<u8> {
		match address {
			IE => Some(self.interrupt.IE.get()),
			IF => Some(self.interrupt.IF.get()),
			LCDC => Some(self.gpu.LCDC.get()),
			STAT => Some(self.gpu.STAT.get()),
			LYC => Some(self.gpu.LYC.get()),
			LY => Some(self.gpu.LY.get()),
			_ => None
		}
	}
}