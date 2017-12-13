use rom::*;
use wram::*;
use hram::*;
use gpu::*;
use sink::*;
use interrupt::*;
use memory_map::*;
use joypad::*;
use dma::*;
use timer::*;

pub struct Interconnect {
	rom: Rom,
	wram: Wram,
	hram: Hram,
	oam_dma: OamDma,
	timer: Timer,
	pub gpu: Gpu,
	pub interrupt: InterruptHandler,
	pub joypad: Joypad,
}

impl Interconnect {
	pub fn new(_rom: Rom) -> Interconnect {
		Interconnect {
			rom: _rom,
			gpu: Gpu::new(),
			wram: Wram::new(),
			hram: Hram::new(),
			timer: Timer::new(),
			oam_dma: OamDma::new(),
			interrupt: InterruptHandler::new(),
			joypad: Joypad::new(),
		}
	}

	pub fn read(&self, address: u16) -> u8 {

		// Has a specific register been requested?
		if let Some(value) = self.read_registers(address) {
			return value
		}

		// No specific register, read general data
		match address {
			ROM_START  ... ROM_BANK_END  => self.rom.read(address),
			VRAM_START ... VRAM_END => self.gpu.read(address, false),
			ERAM_START ... ERAM_END => self.rom.read(address),
			WRAM_START ... WRAM_END => self.wram.read(address),
			ECHO_START ... ECHO_END => self.wram.read(address),
			OAM_START  ... OAM_END  => self.gpu.read(address, false),
			HRAM_START ... HRAM_END => self.hram.read(address - HRAM_START),
			_ => panic!("Invalid Read")
		}
	}

	pub fn write(&mut self, address: u16, data: u8) {
		//println!("WRITE ${:2X} TO ${:4X}", data, address);
		if self.write_registers(address, data) {
			return;
		}
		match address {
			ROM_START  ... ROM_BANK_END  => self.rom.write(address, data),
			VRAM_START ... VRAM_END => self.gpu.write(address, data),
			ERAM_START ... ERAM_END => self.rom.write(address, data),
			WRAM_START ... WRAM_END => self.wram.write(address, data),
			ECHO_START ... ECHO_END => {
				// Note: Use of the area from 0xE000 to 0xFDFF is prohibited.
				self.wram.write(address, data);
				//panic!("Attempt to write to ECHO RAM");
			},
			OAM_START  ... OAM_END  => self.gpu.write(address, data),
			HRAM_START ... HRAM_END => self.hram.write(address - HRAM_START, data),
			_ => panic!("Invalid Write")
		}
	}

	// Take the latest number of machine cycles and keep other hardware in sync
	pub fn cycles(&mut self, cycles: usize, video_sink: &mut VideoSink) {
		self.gpu.cycles(cycles, &mut self.interrupt, video_sink);
		self.timer.cycles(cycles, &mut self.interrupt);
		self.perform_dma(cycles);
	}

	fn perform_dma(&mut self, cycles: usize) {
		// OAM DMA
		if self.oam_dma.active {
			let (from, to, bytes) = self.oam_dma.cycles(cycles);
			for offset in 0..bytes {
				let value = self.read(from + offset as u16);
				self.write(to + offset as u16, value);
			}
		}
	}

	// Intercept and re-route reads to memory registers to their actual location
	fn read_registers(&self, address: u16) -> Option<u8> {
		match address {
			P1 =>   Some(self.joypad.read()),
			IE =>   Some(self.interrupt.IE.get()),
			IF =>   Some(self.interrupt.IF.get()),
			LCDC => Some(self.gpu.LCDC.get()),
			STAT => Some(self.gpu.STAT.get()),
			LYC =>  Some(self.gpu.LYC.get()),
			LY =>   Some(self.gpu.LY.get()),
			SCY =>  Some(self.gpu.SCY.get()),
			SCX =>  Some(self.gpu.SCX.get()),
			WX =>  Some(self.gpu.WX.get()),
			WY =>  Some(self.gpu.WY.get()),
			DIV =>  Some(self.timer.read_div()),
			TIMA => Some(self.timer.read_counter()),
			TMA =>  Some(self.timer.read_modulo()),
			TAC =>  Some(self.timer.read_control()),

            // Color Gameboy
            SVBK => Some(self.wram.get_ram_bank()),
            VBK  => Some(self.gpu.get_vram_bank()),

			_ => None
		}
	}

	fn write_registers(&mut self, address: u16, data: u8) -> bool {
		let mut found = true;
		match address {
			P1 => self.joypad.write(data),
			BGP | OBP0 | OBP1 | LCDC | STAT |
			LY | LYC | SCY | SCX | WX | WY => self.gpu.write(address, data),
			OAM_DMA => self.oam_dma.request(data),
			IE | IF => self.interrupt.write(address, data),
			DIV => self.timer.write_div(data),
			TIMA => self.timer.write_counter(data),
			TMA => self.timer.write_modulo(data),
			TAC => self.timer.write_control(data),

            // Color Gameboy
            SVBK => self.wram.set_ram_bank(data),
            VBK => self.gpu.set_vram_bank(data),

			_ => found = false,
		}
		found
	}
}
