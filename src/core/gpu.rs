use core::helper::*;
use core::sink::*;
use core::memory_map::*;
use core::interrupt::*;

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
	HBlank      = 0b00001000,
	VBlank      = 0b00010000,
	Oam         = 0b00100000,
	Coincidence = 0b01000000,
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

// Entry for the sprite table
#[derive(Clone, Debug)]
struct SpriteEntry {
	y_pos: u8,
	x_pos: u8,
	tile_id: u8,
	behind_background: bool,
	x_flip: bool,
	y_flip: bool,
	palette_zero: bool,
}

impl SpriteEntry {
	pub fn new() -> SpriteEntry {
		SpriteEntry {
			y_pos: 0,
			x_pos: 0,
			tile_id: 0,
			behind_background: false,
			x_flip: false,
			y_flip: false,
			palette_zero: false,
		}
	}
}

pub struct Gpu {
	// Memory
	Vram: Vec<u8>,
	Oam: Vec<u8>,
	// Tile Cache
	tile_cache: Vec<TileEntry>, // cache rules everything around me
	// Sprite Table
	sprite_table: Vec<SpriteEntry>,
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
			sprite_table: vec![SpriteEntry::new(); 40],
			frame_buffer: vec![0xFF00FF; FRAME_WIDTH * FRAME_HEIGHT],
			LCDC: MemoryRegister::new(0x91),
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
			0xEEEEEE, // 0 White
			0x999999, // 1 Light Gray
			0x666666, // 2 Dark Gray
			0x222222, // 3 Black
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
			let low_byte = &self.read_raw(offset + (y * 2));
			let high_byte = &self.read_raw(offset + (y * 2) + 1);
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

	pub fn cycles(&mut self, cycles: usize, interrupt: &mut InterruptHandler, video_sink: &mut VideoSink) {

		if !self.display_enabled() {
			return;
		}

		let old_mode = self.get_mode();
		let mut new_mode: StatusMode;

		self.scanline_cycles += cycles;
		self.frame_cycles += cycles;

		// we are in vblank
		if self.frame_cycles > FRAME_PERIOD {

			// We have just entered the Vblank period
			if old_mode != StatusMode::VBlank {
				self.set_mode(StatusMode::VBlank);
				self.set_stat(StatusInterrupt::VBlank);
				// Call the appropriate interrupt
				interrupt.request_interrupt(InterruptFlag::VBlank);
				interrupt.request_interrupt(InterruptFlag::Lcdc);
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
						self.set_stat(StatusInterrupt::Oam);
						interrupt.request_interrupt(InterruptFlag::Lcdc);
					}
				},
				OAM_PERIOD ... TRANSFER_PERIOD => { // Transfer
					if old_mode != StatusMode::Transfer {
						self.set_mode(StatusMode::Transfer);
						// The LCD controller is now transferring data from VRAM to screen.
						// Udpate the internal framebuffer at the current scanline to mimic this.
						self.update_scanline();
					}
				},
				TRANSFER_PERIOD ... HBLANK_PERIOD => { // OAM
					// We have just entered H-Blank
					if old_mode != StatusMode::HBlank {
						self.set_mode(StatusMode::HBlank);
						self.set_stat(StatusInterrupt::HBlank);
						interrupt.request_interrupt(InterruptFlag::Lcdc);
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

		// LY == LYC Coincidence flag
		if self.LY.get() == self.LYC.get() {
			self.STAT.set_bit(Bit::Bit2);
			self.set_stat(StatusInterrupt::Coincidence);
			interrupt.request_interrupt(InterruptFlag::Lcdc);
		} else {
			self.STAT.clear_bit(Bit::Bit2);
		}
	}

	// Draw the current scanline on the internal framebuffer
	fn update_scanline(&mut self) {

		// If BG enabled, draw it
		if self.LCDC.is_set(Bit::Bit0) {
			self.draw_background();
		}

		// If sprites are enabled, draw them
		if self.LCDC.is_set(Bit::Bit1) {
			self.draw_sprites();
		}
	}

	#[inline]
	fn draw_background(&mut self) {
		// BG Tile Map Display Select
		let tile_map_location = match self.LCDC.is_set(Bit::Bit3) {
			true  => 0x9C00,
			false => 0x9800,
		};

		let tile_data_location = match self.LCDC.is_set(Bit::Bit4) {
			false => 0x9000,
			true => 0x8000,
		};

		let y = self.LY.get();
		let map_y = y / 8;
		// Loop through the 20 tiles on this scanline where L = LY
		for map_x in 0..20 {

			let tile_map_index = (map_y as u16 * 32) + map_x;

			// Finding true tile ID ********
			let lookup = tile_map_location + tile_map_index;
			let tile_pattern = self.read_raw(lookup);

			let vram_location = match self.LCDC.is_set(Bit::Bit4) {
				false => {
					let adjusted = ((tile_pattern as i8) as i16) * 16;
					let path = (tile_data_location as i16) + adjusted;
					path as u16
				}, // $8800-97FF (signed, so we start in the middle)
				true  => {
					(tile_pattern as u16 * 16) + tile_data_location
				}, // $8800-97FF (unsigned)
			};

			let tile_id = self.address_to_tile_id(vram_location);

			// Refresh the tile if it has been overwritten in VRAM
			if self.tile_cache[tile_id].dirty {
				self.refresh_tile(tile_id);
			}

			let tile_y = y % 8;
			for tile_x in 0..8 {
				let pixel = self.tile_cache[tile_id].pixels[((tile_y * 8) + tile_x) as usize];
				let buffer_offset = (y as u16 * 160) + (map_x as u16 * 8) + tile_x as u16;
				self.frame_buffer[buffer_offset as usize] = pixel;
			}
		}
	}

	#[inline]
	fn draw_sprites(&mut self) {

		// Only 10 sprites can be displayed per scanline
		let scanline_y = self.LY.get();

		// Get all the sprites with a Y range that intersects with the current scanline
		// Limit the first 10, and draw reversed. Lower indexed sprites have higher priority
		let mut iter = self.sprite_table.clone().into_iter().filter(|sprite| {
			scanline_y >= sprite.y_pos && scanline_y <= sprite.y_pos + 7
		}).rev().take(10);

		// Draw the damn thing
		for sprite in iter {
			let sprite_x = sprite.x_pos;
			let sprite_y = sprite.y_pos;

			if self.tile_cache[sprite.tile_id as usize].dirty {
				self.refresh_tile(sprite.tile_id as usize);
			}

			let tile = &self.tile_cache[sprite.tile_id as usize];

			for pixel_x in 0..8 {

				let pixel_y = scanline_y - sprite_y;

				// Flip the X/Y rendering if necessary
				let lookup_x = match sprite.x_flip {
					true  => ((pixel_x as i8 - 7) * -1) as u8,
					false => pixel_x
				};
				let lookup_y = match sprite.y_flip {
					true  => ((pixel_y as i8 - 7) * -1) as u8,
					false => pixel_y
				};

				let pixel = tile.pixels[((lookup_y * 8) + lookup_x) as usize];
				let offset_x = sprite_x as i32 + pixel_x as i32;
				let offset_y = scanline_y as i32 * FRAME_WIDTH as i32;
				let offset = offset_y + offset_x;
				self.frame_buffer[offset as usize] = pixel;
			}
		}
	}

	// Translates a location in VRAM to the relevant tile cache ID
	#[inline]
	fn address_to_tile_id(&self, address: u16) -> usize {
		((address - VRAM_START) / 16) as usize
	}

	#[inline]
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

	#[inline]
	fn set_mode(&mut self, mode: StatusMode) {
		let mut stat = self.STAT.get() & !(0x3);
		stat |= mode as u8;
		self.STAT.set(stat);
	}

	// sets the interrupt type on the status register
	// so programmers can check the reason the machine interrupted
	fn set_stat(&mut self, mode: StatusInterrupt) {
		let mut stat = self.STAT.get();
		stat |= mode as u8;
		self.STAT.set(stat);
	}

	// Reads raw data directly from VRAM
	// This is necessary to bypass the memory access restrictions
	// that are imposed on the CPU depending on LCD STAT register
	#[inline]
	fn read_raw(&self, address: u16) -> u8 {
		self.Vram[(address - VRAM_START) as usize]
	}

	pub fn read(&self, address: u16) -> u8 {
		match address {
			VRAM_START ... VRAM_END => {
				match self.get_mode() {
					// Cannot access VRAM in Transfer Mode
					StatusMode::Transfer => 0xFF,
					_ => {
						self.Vram[(address - VRAM_START) as usize]
					},
				}
			},
			OAM_START  ... OAM_END  => {
				match self.get_mode() {
					// Cannot access OAM in the following modes:
					StatusMode::Transfer | StatusMode::Oam => 0xFF,
					_ => {
						self.Oam[(address - OAM_START) as usize]
					},
				}
			},
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, address: u16, data: u8) {
		let stat = self.LCDC.get();
		match address {
			BGP  => { self.BGP.set(data); },
			OBP0 => { self.OBP0.set(data); },
			OBP1 => { self.OBP1.set(data); },
			LCDC => { self.update_lcdc(data); },
			STAT => {
				// Bits 0-2 are read only
				self.STAT.set(data & 0xF8);
			},
			LYC => { self.LYC.set(data); },
			LY => { self.LY.set(data); },

			VRAM_START ... VRAM_END => {
				// Disallow writes to VRAM depending on the mode
				if self.get_mode() == StatusMode::Transfer {
					return;
				}
				let index = address - VRAM_START;
				self.Vram[index as usize] = data;
				// Mark this data as dirty so the tile cache updates
				if address <= TILE_RAM_END {
					let tile_id = index / 16;
					self.tile_cache[tile_id as usize].dirty = true;
				}
			},

			OAM_START ... OAM_END => {
				match self.get_mode() {
					StatusMode::Oam | StatusMode::Transfer => { return; },
					_ => {
						self.Oam[(address - OAM_START) as usize] = data;
						self.update_sprite(address, data);
					}
				};
			},
			_ => unreachable!(),
		}
	}

	// Update the sprite table with the relevant new information
	fn update_sprite(&mut self, address: u16, data: u8) {
		let sprite_id = (address - OAM_START) / 4; // 4 bytes of information per sprite
		let sprite = &mut self.sprite_table[sprite_id as usize];
		let data_type = address % 4;
		match data_type {
			0 => sprite.y_pos = data.wrapping_sub(16),
			1 => sprite.x_pos = data.wrapping_sub(8),
			2 => sprite.tile_id = data,
			3 => {
				sprite.behind_background = (data & Bit::Bit7 as u8) > 0;
				sprite.y_flip = (data & Bit::Bit6 as u8) > 0;
				sprite.x_flip = (data & Bit::Bit5 as u8) > 0;
				sprite.palette_zero = (data & Bit::Bit4 as u8) > 0;
			},
			_ => unreachable!()
		};
	}

	fn update_lcdc(&mut self, data: u8) {
		let new = MemoryRegister::new(data);

		if !new.is_set(Bit::Bit7) && self.display_enabled() {
			if self.get_mode() != StatusMode::VBlank {
				panic!("LCD off, but not in VBlank");
			}
			self.LY.clear();
			// Set stat mode to 0 to let game know it is safe to write to RAM
			self.set_mode(StatusMode::HBlank);
		}
		self.LCDC.set(data);
	}

	#[inline]
	fn display_enabled(&self) -> bool {
		self.LCDC.is_set(Bit::Bit7)
	}

	pub fn dump(&self) {
		println!("DUMPING VRAM");
		dump("vram.bin", &self.Vram);
		dump("oam.bin", &self.Oam);
	}
}
