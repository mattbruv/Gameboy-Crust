use core::helper::*;

const LCD_WIDTH: usize  = 160;
const LCD_HEIGHT: usize = 144;

// time in cycles for each mode to complete
// Read -> Transfer -> Hblank (reapeat...) until Vblank
const SCANLINE_PERIOD: usize   = 456; // 456 cycles
const OAM_PERIOD: usize        = 80; // 77-83 cycles, 80 average
const TRANSFER_PERIOD: usize   = OAM_PERIOD + 172; // 169-175 cycles, 172 average
const HBLANK_PERIOD: usize     = SCANLINE_PERIOD - TRANSFER_PERIOD; // = 204 average, 201-207

// time in cycles for rendering full screen and vblank
const FRAME_PERIOD: usize      = SCANLINE_PERIOD * LCD_HEIGHT; // 65,664 cycles for full frame
const VBLANK_PERIOD: usize     = FRAME_PERIOD + 4560; // 4,560 cycles for vblank

// Status of the LCD controller
#[derive(Debug, PartialEq)]
enum StatusMode {
	HBlank   = 0,
	VBlank   = 1,
	Oam      = 2,
	Transfer = 3,
}

pub struct Gpu {
	pub LCDC: MemoryRegister,
	pub STAT: MemoryRegister,
	pub LYC: MemoryRegister,
	pub LY: MemoryRegister,
	scanline_cycles: usize,
	frame_cycles: usize, 
}

impl Gpu {
	pub fn new() -> Gpu {
		Gpu {
			LCDC: MemoryRegister::new(0x00),
			STAT: MemoryRegister::new(0x00),
			LYC: MemoryRegister::new(0x00),
			LY: MemoryRegister::new(0x00),
			scanline_cycles: 0,
			frame_cycles: 0,
		}
	}

	pub fn cycles(&mut self, cycles: usize) {
		self.scanline_cycles += cycles;
		let line_cycles = self.scanline_cycles;
		let mode = self.get_mode();
	}

	fn get_mode(&self) -> StatusMode {
		let mode = self.STAT.get() & 0x3;
		match mode {
			0 => StatusMode::HBlank,
			1 => StatusMode::VBlank,
			2 => StatusMode::Oam,
			3 => StatusMode::Transfer,
			_ => unreachable!(),
		}
	}

	fn set_mode(&mut self, mode: StatusMode) {
		let mut stat = self.STAT.get() & !(0x3);
		stat |= mode as u8;
		self.STAT.set(stat);
	}
}