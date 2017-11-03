use core::helper::*;
use core::sink::*;
use core::memory_map::*;

const FRAME_WIDTH: usize = 160;
const FRAME_HEIGHT: usize = 144;

const TILE_RAM_END: u16 = 0x97FF;

const VRAM_SIZE: usize = 8192; // 8Kb Bank
const OAM_SIZE: usize = 160; // 160byte OAM memory

const LCD_WIDTH: usize  = 160;
const LCD_HEIGHT: usize = 144;

// time in cycles for each mode to complete
// Read -> Transfer -> Hblank (reapeat...) until Vblank
const OAM_PERIOD: usize        = 80; // 77-83 cycles, 80 average
const TRANSFER_PERIOD: usize   = OAM_PERIOD + 172; // 169-175 cycles, 172 average
const HBLANK_PERIOD: usize     = 456; // 456 cycles

// time in cycles for rendering full screen and vblank
const FRAME_PERIOD: usize      = HBLANK_PERIOD * LCD_HEIGHT; // 65,664 cycles for full frame
const VBLANK_PERIOD: usize     = FRAME_PERIOD + 4560; // 4,560 cycles for vblank

// Status of the LCD controller
#[derive(Debug, PartialEq)]
enum StatusMode {
	HBlank   = 0,
	VBlank   = 1,
	Oam      = 2,
	Transfer = 3,
}

enum StatusInterrupt {
	HBlank,
	VBlank,
	Oam,
	Coincidence,
}

// Entry for the tile cache
#[derive(Clone, Debug)]
struct TileEntry {
	dirty: bool,
	pixels: Vec<u32>,
}

impl TileEntry {
	pub fn new() -> TileEntry {
		TileEntry {
			dirty: true,
			pixels: vec![0; 64],
		}
	}
}

pub struct Gpu {
	// Memory
	Vram: Vec<u8>,
	Oam:  Vec<u8>,
	// Tile Cache
	tile_cache: Vec<TileEntry>, // cache rules everything around me
	// Frame Buffer
	frame_buffer: Vec<u32>,
	// Registers
	pub LCDC: MemoryRegister,
	pub STAT: MemoryRegister,
	pub LYC: MemoryRegister,
	pub LY: MemoryRegister,
	pub BGP: MemoryRegister,
	pub OBP0: MemoryRegister,
	pub OBP1: MemoryRegister,
	scanline_cycles: usize,
	frame_cycles: usize,
}

impl Gpu {
	pub fn new() -> Gpu {
		Gpu {
			Vram: vec![0; VRAM_SIZE],
			Oam:  vec![0; OAM_SIZE],
			tile_cache: vec![TileEntry::new(); 384],
			frame_buffer: vec![0xFF00FF; FRAME_WIDTH * FRAME_HEIGHT],
			LCDC: MemoryRegister::new(0x00),
			STAT: MemoryRegister::new(0x02),
			LYC: MemoryRegister::new(0x00),
			LY: MemoryRegister::new(0x00),
			BGP: MemoryRegister::new(0x00),
			OBP0: MemoryRegister::new(0x00),
			OBP1: MemoryRegister::new(0x00),
			scanline_cycles: 0,
			frame_cycles: 0,
		}
	}

	// Converts a 0-3 shade to the appropriate 32bit palette color
	fn colorize(&mut self, shade: u8) -> u32 {
		let palette = [
			0xFFFFFF, // 0 White
			0x999999, // 1 Light Gray
			0x333333, // 2 Dark Gray
			0x000000, // 3 Black
		];
		let pal_data = self.BGP.get();
		let real_shade = match shade {
			0 =>  pal_data & 0b00000011,
			1 => (pal_data & 0b00001100) >> 2,
			2 => (pal_data & 0b00110000) >> 4,
			3 => (pal_data & 0b11000000) >> 6,
			_ => panic!("Invalid Palette Shade!")
		};
		palette[real_shade as usize]
	}

	// Returns a 128x192px display for entire tile cache for debugging
	// Tile cache is 384 tiles, entire VRAM is turned into a tile cache
	// Even though we only use certain areas, it makes it easier to cache
	// Entire VRAM as if all data were tiles.
	pub fn get_tiles(&mut self) -> Vec<u32> {
		let width = 128;
		let height = 192;
		let mut display = vec![0xFF00FF; width * height];
		// Loop entire VRAM as tiles
		for index in 0..384 {

			if self.tile_cache[index].dirty {
				self.refresh_tile(index);
			}

			let entry = &self.tile_cache[index];

			for y in 0..8 {
				for x in 0..8 {
					let color = entry.pixels[(y * 8) + x];
					let column = index % 16;
					let row = index / 16;
					let width_offset = (column * 8) + x;
					let height_offset = ((row * 8) + y) * width;
					let vec_offset = width_offset + height_offset;
					display[vec_offset] = color;
				}
			}
		}
		display
	}

	// Updates the tile cache with the current data in VRAM for that tile
	pub fn refresh_tile(&mut self, id: usize) {
		//let entry = &mut self.tile_cache[id];

		let offset = VRAM_START + (id * 16) as u16;
		//println!("OFFSET ${:04X}", offset);

		let mut tile = vec![0; 64];

		for y in 0..8 {
			let low_byte = &self.read(offset + (y * 2));
			let high_byte = &self.read(offset + (y * 2) + 1);
			let mut x: i8 = 7;
			// Loop through all the pixels in a y value
			while x >= 0 {
				let x_flip = (x - 7) * -1;
				// 7
				let low_bit = (low_byte >> x) & 1;
				let high_bit = (high_byte >> x) & 1;

				let combined = (high_bit << 1) | low_bit;

				tile[((y * 8) + x_flip as u16) as usize] = self.colorize(combined);

				x -= 1;
			}
		}

		self.tile_cache[id].dirty = false;
		self.tile_cache[id].pixels = tile;
	}

	pub fn cycles(&mut self, cycles: usize, video_sink: &mut VideoSink) {

		let old_mode = self.get_mode();
		let mut new_mode: StatusMode;

		self.scanline_cycles += cycles;
		self.frame_cycles += cycles;

		// we are in vblank
		if self.frame_cycles > FRAME_PERIOD {

			if old_mode != StatusMode::VBlank {
				self.set_mode(StatusMode::VBlank);
			}

			// we have completed vblank period, reset everything, update sink
			if self.frame_cycles > VBLANK_PERIOD {
				self.scanline_cycles = 0;
				self.frame_cycles = 0;
				self.LY.clear();
				self.set_mode(StatusMode::Oam);
				video_sink.append(self.frame_buffer.clone());
			}

		} else {
			// Update the scanline state
			match self.scanline_cycles {
				0 ... OAM_PERIOD => { // OAM
					if old_mode != StatusMode::Oam {
						self.set_mode(StatusMode::Oam);
					}
				},
				OAM_PERIOD ... TRANSFER_PERIOD => { // Transfer
					if old_mode != StatusMode::Transfer {
						self.set_mode(StatusMode::Transfer);
					}
				},
				TRANSFER_PERIOD ... HBLANK_PERIOD => { // OAM
					// We have just entered H-Blank
					if old_mode != StatusMode::HBlank {
						self.set_mode(StatusMode::HBlank);
					}
				},
				_ => {},
			}
		}

		// If we have finished the H-Blank period, we are on a new line
		// LY is updated even if we are in V-blank
		if self.scanline_cycles > HBLANK_PERIOD {
			self.LY.add(1);
			self.scanline_cycles = 0;
		}
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

	// sets the interrupt type on the status register
	// so programmers can check the reason the machine interrupted
	fn set_interrupt(&mut self, mode: StatusInterrupt, value: bool) {

		let stat = self.STAT.get();

		match mode {
			StatusInterrupt::HBlank => {},
			StatusInterrupt::VBlank => {},
			StatusInterrupt::Oam => {},
			StatusInterrupt::Coincidence => {},
		}

	}

	pub fn read(&self, address: u16) -> u8 {
		match address {
			VRAM_START ... VRAM_END => {
				self.Vram[(address - VRAM_START) as usize]
			},
			OAM_START  ... OAM_END  => {
				self.Oam[(address - OAM_START) as usize]
			},
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, address: u16, data: u8) {
		match address {
			BGP  => { self.BGP.set(data); },
			OBP0 => { self.OBP0.set(data); },
			OBP1 => { self.OBP1.set(data); },
			VRAM_START ... VRAM_END => {
				let index = address - VRAM_START;
				self.Vram[index as usize] = data;
				// Mark this data as dirty so the tile cache updates

				if address <= TILE_RAM_END {
					let tile_id = index / 16;
					self.tile_cache[tile_id as usize].dirty = true;
				}
			},
			OAM_START ... OAM_END => {
				self.Oam[(address - OAM_START) as usize] = data;
			},
			_ => unreachable!(),
		}
	}

	pub fn debug(&mut self) {
		//self.tile_cache[1].pixels[8] = 3;
		//println!("{:?}", self.tile_cache);
	}

	pub fn dump(&self) {
		println!("DUMPING VRAM");
		dump("vram.bin", &self.Vram);
		dump("oam.bin", &self.Oam);
	}
}
