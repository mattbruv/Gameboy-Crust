use core::helper::*;
use core::sink::*;
use core::memory_map::*;
use core::interrupt::*;

const FRAME_WIDTH: usize = 160;
const FRAME_HEIGHT: usize = 144;

const TILE_RAM_END: u16 = 0x97FF;

const VRAM_SIZE: usize = 8192; // 8Kb Bank, 16KB for GBC
const OAM_SIZE: usize = 160; // 160byte OAM memory
const PALETTE_RAM_SIZE: usize = 64; // 8 bytes per obj palette X 8 obj palettes

// time in cycles for each mode to complete
// Read -> Transfer -> Hblank (reapeat...) until Vblank
const TOTAL_SCANLINE_CYCLES: isize = 456; // 456 cycles to draw a scanline
const OAM_CUTOFF: isize = TOTAL_SCANLINE_CYCLES - 80; // 77-83 cycles, 80 average
const TRANSFER_CUTOFF: isize = OAM_CUTOFF - 172; // 169-175 cycles, 172 average

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
	pixels: Vec<u8>,
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
	y_pos: i32,
	x_pos: i32,
	tile_id: u8,
	behind_background: bool,
	x_flip: bool,
	y_flip: bool,
	use_palette_one: bool,
	bank: u8,
	palette: u8,
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
			use_palette_one: false,
			bank: 0,
			palette: 0,
		}
	}
}

pub struct Gpu {

	// Color Palette RAM
	cgb_pal_ram: Vec<u8>,
	sprite_pal_ram: Vec<u8>,
	BGPI: MemoryRegister,
	OBPI: MemoryRegister,

	// Memory
	vram: Vec<u8>,
	cgb_vram: Vec<u8>,
	oam: Vec<u8>,
	// Tile Cache
	tile_cache: Vec<TileEntry>, // cache rules everything around me
	cgb_tile_cache: Vec<TileEntry>,
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
	pub SCY: MemoryRegister,
	pub SCX: MemoryRegister,
	pub WY: MemoryRegister,
	pub WX: MemoryRegister,
	scanline_counter: isize,
    vram_bank: u8,
}

impl Gpu {
	pub fn new() -> Gpu {
		Gpu {
			cgb_pal_ram: vec![0; PALETTE_RAM_SIZE],
			sprite_pal_ram: vec![0; PALETTE_RAM_SIZE],
			BGPI: MemoryRegister::new(0x00),
			OBPI: MemoryRegister::new(0x00),
			vram: vec![0; VRAM_SIZE],
			cgb_vram: vec![0; VRAM_SIZE],
			oam:  vec![0; OAM_SIZE],
			tile_cache: vec![TileEntry::new(); 384],
			cgb_tile_cache: vec![TileEntry::new(); 384],
			sprite_table: vec![SpriteEntry::new(); 40],
			frame_buffer: vec![0xFF00FF; FRAME_WIDTH * FRAME_HEIGHT],
			LCDC: MemoryRegister::new(0x91),
			STAT: MemoryRegister::new(0x00),
			LYC: MemoryRegister::new(0x00),
			LY: MemoryRegister::new(0x00),
			BGP: MemoryRegister::new(0xFC),
			OBP0: MemoryRegister::new(0x00),
			OBP1: MemoryRegister::new(0x00),
			SCY: MemoryRegister::new(0x00),
			SCX: MemoryRegister::new(0x00),
			WY: MemoryRegister::new(0x00),
			WX: MemoryRegister::new(0x00),
			scanline_counter: TOTAL_SCANLINE_CYCLES,
            vram_bank: 0,
		}
	}

	// Converts a 0-3 shade to the appropriate 32bit palette color
	fn colorize(&self, shade: u8, palette: u8) -> u32 {
		let color_values = [
			0xEEEEEE, // 0 White
			0x999999, // 1 Light Gray
			0x666666, // 2 Dark Gray
			0x222222, // 3 Black
		];
		let real_shade = match shade {
			0 =>  palette & 0b00000011,
			1 => (palette & 0b00001100) >> 2,
			2 => (palette & 0b00110000) >> 4,
			3 => (palette & 0b11000000) >> 6,
			_ => panic!("Invalid Palette Shade!")
		};
		color_values[real_shade as usize]
	}

	// Returns a 128x192px display for entire tile cache for debugging
	// Tile cache is 384 tiles, entire VRAM is turned into a tile cache
	// Even though we only use certain areas, it makes it easier to cache
	// Entire VRAM as if all data were tiles.
	pub fn get_tiles(&mut self) -> Vec<u32> {
		let width = 128;
		let height = 192;
		let mut display = vec![0xFF00FF; width * height];
		let palette = self.BGP.get();
		// Loop entire VRAM as tiles
		for index in 0..384 {

			if self.cgb_tile_cache[index].dirty {
				self.refresh_tile(index, 1); // we dont really care about this any more
			}

			for y in 0..8 {
				for x in 0..8 {
					let raw_pixel = self.cgb_tile_cache[index].pixels[(y * 8) + x];
					let color = self.colorize(raw_pixel, palette);
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

	pub fn get_tile(&self, id: usize, bank: u8) -> TileEntry {
		match bank {
			0 => self.tile_cache[id].clone(),
			1 => self.cgb_tile_cache[id].clone(),
			_ => unreachable!()
		}
	}

	// Updates the tile cache with the current data in VRAM for that tile
	pub fn refresh_tile(&mut self, id: usize, bank: u8) {
		//let entry = &mut self.tile_cache[id];

		let offset = VRAM_START + (id * 16) as u16;
		//println!("OFFSET ${:04X}", offset);

		let mut tile = vec![0; 64];

		for y in 0..8 {
			let low_byte  = &self.read_raw(offset + (y * 2), bank);
			let high_byte = &self.read_raw(offset + (y * 2) + 1, bank);
			let mut x: i8 = 7;
			// Loop through all the pixels in a y value
			while x >= 0 {
				let x_flip = (x - 7) * -1;
				// 7
				let low_bit = (low_byte >> x) & 1;
				let high_bit = (high_byte >> x) & 1;

				let combined = (high_bit << 1) | low_bit;

				tile[((y * 8) + x_flip as u16) as usize] = combined;

				x -= 1;
			}
		}

		match bank {
			0 => {
				self.tile_cache[id].dirty = false;
				self.tile_cache[id].pixels = tile;
			},
			1 => {
				self.cgb_tile_cache[id].dirty = false;
				self.cgb_tile_cache[id].pixels = tile;
			},
			_ => unreachable!()
		};
	}

	pub fn cycles(&mut self, cycles: usize, interrupt: &mut InterruptHandler, video_sink: &mut VideoSink) {

        self.set_lcd_status(interrupt);

        if !self.display_enabled() {
            return;
        }

        self.scanline_counter -= cycles as isize;

        if self.scanline_counter < 0
        {
            // Reset our counter
            self.scanline_counter = TOTAL_SCANLINE_CYCLES;

            let scanline = self.LY.get();

            if scanline == 144 {
                // Request V-blank interrupt
                interrupt.request_interrupt(InterruptFlag::VBlank);
                // we have also reached the end of the frame, output it
                video_sink.append(self.frame_buffer.clone());
            }
            else if scanline > 153 {
                self.LY.clear();
            }
            else if scanline < 144 {
                self.update_scanline();
            }

            self.LY.add(1);
        }
	}

    fn set_lcd_status(&mut self, interrupt: &mut InterruptHandler) {
        // if the LCD is disabled, reset our internal counters
        if !self.display_enabled() {
            self.scanline_counter = TOTAL_SCANLINE_CYCLES;
            self.LY.clear(); // reset scanline counter
            self.set_mode(StatusMode::VBlank);
            return;
        }

        let current_line = self.LY.get();
        let current_mode = self.get_mode();
        let mut new_mode: StatusMode;

        let new_mode: StatusMode;
        let mut request_interrupt = false;

        // We are in VBlank
        if current_line >= 144 {
            new_mode = StatusMode::VBlank;
            request_interrupt = self.STAT.is_set(Bit::Bit4);
        }
        else {
            // Are we in Mode 2? (OAM search)
            if self.scanline_counter >= OAM_CUTOFF {
                new_mode = StatusMode::Oam;
                request_interrupt = self.STAT.is_set(Bit::Bit5);
            }
            // Are we in Mode 3? (transferring VRAM/OAM)
            else if self.scanline_counter >= TRANSFER_CUTOFF {
                new_mode = StatusMode::Transfer;
                // No interrupt is requested for this mode
            }
            // If nothing else, we are in Mode 0. (H-Blank)
            else {
                new_mode = StatusMode::HBlank;
                request_interrupt = self.STAT.is_set(Bit::Bit3);
            }
        }

        // Request interrupt if we have entered a new mode and the game requests one
        if request_interrupt && (current_mode != new_mode) {
            interrupt.request_interrupt(InterruptFlag::Lcdc);
        }

        // Check coincidence flag
        self.line_compare(interrupt);

        // Assign new mode
        self.set_mode(new_mode);
    }

    fn line_compare(&mut self, interrupt: &mut InterruptHandler) {
		// LY == LYC Coincidence flag
        let enabled = self.STAT.is_set(Bit::Bit6);
		if enabled && self.LY.get() == self.LYC.get() {
			self.STAT.set_bit(Bit::Bit2); // make sure this is being called?
			interrupt.request_interrupt(InterruptFlag::Lcdc);
		} else {
			self.STAT.clear_bit(Bit::Bit2);
		}
    }
	// Draw the current scanline on the internal framebuffer
	fn update_scanline(&mut self) {
		// A helper vector to determine sprite priority relative to bg
		// set to true if bg pixel = any color but zero
		let mut bg_priority = vec![false; FRAME_WIDTH];
		// If BG enabled, draw it
		if self.LCDC.is_set(Bit::Bit0) {
			self.draw_background(&mut bg_priority);
		}

		if self.LCDC.is_set(Bit::Bit5) {
			self.draw_window(&mut bg_priority);
		}

		// If sprites are enabled, draw them
		if self.LCDC.is_set(Bit::Bit1) {
			self.draw_sprites(&mut bg_priority);
		}
	}

	#[inline]
	fn draw_background(&mut self, bg_priority: &mut Vec<bool>) {
		let palette = self.BGP.get();
		// BG Tile Map Display Select
		let tile_map_location = match self.LCDC.is_set(Bit::Bit3) {
			true  => 0x9C00,
			false => 0x9800,
		};

		let tile_data_location = match self.LCDC.is_set(Bit::Bit4) {
			false => 0x9000,
			true => 0x8000,
		};

		let display_y = self.LY.get();
		let y = display_y.wrapping_add(self.SCY.get());
		let row = (y / 8);
		let buffer_start = display_y as usize * FRAME_WIDTH;

		for i in 0..FRAME_WIDTH {
			let x = (i as u8).wrapping_add(self.SCX.get());
			let column = (x / 8);
			let tile_map_index = (row as u16 * 32) + column as u16;
			let lookup = tile_map_location + tile_map_index;
			let tile_pattern = self.read_raw(lookup, 0);

			let color_attributes = self.read_raw(lookup, 1);
			let bank = (color_attributes & Bit::Bit3 as u8) >> 3;
			let h_flip = color_attributes & Bit::Bit5 as u8 > 0;
			let v_flip = color_attributes & Bit::Bit6 as u8 > 0;
			let palette_id = color_attributes & 0x7; // bits 0-2

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
			let tile = self.get_tile(tile_id, bank);

			// Refresh the tile if it has been overwritten in VRAM
			if tile.dirty {
				self.refresh_tile(tile_id, bank);
			}

			let mut pixel_x = x % 8;
			if h_flip { pixel_x = ((pixel_x as i8 - 7) * -1) as u8; }
			let mut pixel_y = y % 8;
			if v_flip { pixel_y = ((pixel_y as i8 - 7) * -1) as u8; }
			let pixel = tile.pixels[((pixel_y * 8) + pixel_x) as usize];
			//let color = self.colorize(pixel, palette);
			let color = self.cgb_colorize(pixel, palette_id, palette);
			let offset = buffer_start + i;
			if pixel != 0 { bg_priority[i] = true; }
			self.frame_buffer[offset as usize] = color;
		}
	}

	#[inline]
	fn draw_window(&mut self, bg_priority: &mut Vec<bool>) {
		let window_y = self.WY.get();
		let window_x = self.WX.get().wrapping_sub(7);
		let y = self.LY.get();
		let palette = self.BGP.get();

		if y < window_y { return; }

		let tile_map_location = match self.LCDC.is_set(Bit::Bit6) {
			true  => 0x9C00,
			false => 0x9800
		};

		let tile_data_location = match self.LCDC.is_set(Bit::Bit4) {
			false => 0x9000,
			true => 0x8000,
		};

		let pixel_y = y % 8;
		let buffer_start = y as usize * FRAME_WIDTH;

		let row = (y - window_y) / 8;

		let debug_line_color = ((y - window_y) as f32 * 1.77) as u8;
		let mut debug_color: u32 = (debug_line_color as u32) << 16;
		//debug_color |= ((debug_line_color as u32) << 8);
		debug_color |= (debug_line_color as u32);

		for i in 0..FRAME_WIDTH {
			let display_x = (i as u8).wrapping_add(window_x);
			let column = i as u8 / 8;
			let tile_map_index = (row as u16 * 32) + column as u16;
			let offset = tile_map_location + tile_map_index;
			let tile_pattern = self.read_raw(offset, 0);

			let color_attributes = self.read_raw(offset, 1);
			let bank = (color_attributes & Bit::Bit3 as u8) >> 3;
			let h_flip = color_attributes & Bit::Bit5 as u8 > 0;
			let v_flip = color_attributes & Bit::Bit6 as u8 > 0;
			let palette_id = color_attributes & 0x7; // bits 0-2

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
			let tile = self.get_tile(tile_id, bank);

			// Refresh the tile if it has been overwritten in VRAM
			if tile.dirty {
				self.refresh_tile(tile_id, bank);
			}

			let pixel_x = i % 8;
			let pixel = tile.pixels[((pixel_y * 8) + pixel_x as u8) as usize];
			let color = self.cgb_colorize(pixel, palette_id, palette);
			let buffer_offset = buffer_start + i;
			if pixel != 0 { bg_priority[i] = true; }
			self.frame_buffer[buffer_offset as usize] = color;

		}
	}

	#[inline]
	fn draw_sprites(&mut self, bg_priority: &mut Vec<bool>) {

		// Only 10 sprites can be displayed per scanline
		let scanline_y = self.LY.get();
		let tall_sprite_mode = self.LCDC.is_set(Bit::Bit2);
		let sprite_y_max = match tall_sprite_mode {
			true => 15, // 0-15 y pixels for 8x16 sprites
			false => 7 // 0-7 y pixels for 8x8 sprites
		};

		// Get all the sprites with a Y range that intersects with the current scanline
		// Limit the first 10, and draw reversed. Lower indexed sprites have higher priority
		let mut iter = self.sprite_table.clone().into_iter().filter(|sprite| {
			scanline_y as i32 >= sprite.y_pos && scanline_y as i32 <= sprite.y_pos + sprite_y_max as i32
			&& sprite.x_pos + 8 >= 0 && sprite.x_pos < FRAME_WIDTH as i32
		}).rev().take(10);

		// Draw the damn thing
		for sprite in iter {
			let sprite_x = sprite.x_pos;
			let sprite_y = sprite.y_pos as u8;

			let pixel_y = (scanline_y.wrapping_sub(sprite_y)) % 8;
			let lookup_y = match sprite.y_flip {
				true  => { ((pixel_y as i8 - 7) * -1) as u8 },
				false => pixel_y
			};

			let tile_id = match tall_sprite_mode {
				true => {
					// Are we displaying the top half or bottom half?
					if (scanline_y.wrapping_sub(sprite_y) < 8) { // top half
						if sprite.y_flip { sprite.tile_id | 0x01 }
						else { sprite.tile_id & 0xFE }
					} else { // bottom half
						if sprite.y_flip { sprite.tile_id & 0xFE }
						else { sprite.tile_id | 0x01 }
					}
				},
				false => sprite.tile_id,
			};

			let bank = self.vram_bank; //sprite.bank;
			let tile = self.get_tile(tile_id as usize, bank);
            let palette_id = sprite.palette;

			if tile.dirty {
				self.refresh_tile(tile_id as usize, bank);
			}
            
			/*let palette = match sprite.use_palette_one {
				false => self.OBP0.get(),
				true  => self.OBP1.get(),
			};*/

			for pixel_x in 0..8 {
				let adjusted_x = (sprite_x + pixel_x as i32) as u8;

                // Do not draw out of bounds sprites
                if adjusted_x >= 160 { continue; };

				// Flip the X/Y rendering if necessary
				let lookup_x = match sprite.x_flip {
					true  => ((pixel_x as i8 - 7) * -1) as u8,
					false => pixel_x
				};

				let pixel = tile.pixels[((lookup_y * 8) + lookup_x) as usize];
				if pixel == 0 { continue; } // Color zero is ignored when drawing sprites
				// Do not draw over background priority
				if sprite.behind_background {
					if bg_priority[adjusted_x as usize] {
						continue;
					}
				}
                let palette = 0;
				let color = self.sprite_colorize(pixel, palette_id, palette);
				let offset_x = adjusted_x as i32;
				let offset_y = scanline_y as i32 * FRAME_WIDTH as i32;
				let offset = offset_y + offset_x;
				self.frame_buffer[offset as usize] = color;
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
	fn read_raw(&self, address: u16, bank: u8) -> u8 {
        self.read(address, true, bank)
	}

	pub fn read(&self, address: u16, raw: bool, bank: u8) -> u8 {
		match address {
			VRAM_START ... VRAM_END => {
                let mode = self.get_mode();
                if mode == StatusMode::Transfer && !raw {
                    0xFF
                } else {
                    let offset = address - VRAM_START;
                    match bank {
                    	0 => self.vram[offset as usize],
                    	1 => self.cgb_vram[offset as usize],
                    	_ => unreachable!(),
                    }
                }
			},
			OAM_START  ... OAM_END  => {
                let mode = self.get_mode();
                if (mode == StatusMode::Transfer || mode == StatusMode::Oam) && !raw {
                    0xFF
                } else {
                    self.oam[(address - OAM_START) as usize]
                }
			},
			_ => unreachable!(),
		}
	}

	pub fn write(&mut self, address: u16, data: u8, bank: u8) {
		match address {
			BGP  => { self.BGP.set(data); },
			OBP0 => { self.OBP0.set(data); },
			OBP1 => { self.OBP1.set(data); },
			LCDC => { self.update_lcdc(data); },
			STAT => {
				let stat = self.STAT.get();
				let high = data & 0xF8;
				let low = stat & 0x7; // Bits 0-2 are read only
				self.STAT.set(high | low);
			},
			LYC => { self.LYC.set(data); },
			LY => { self.LY.clear(); }, // writing resets counter
			SCY => { self.SCY.set(data); },
			SCX => { self.SCX.set(data); },
			WY => { self.WY.set(data); },
			WX => { self.WX.set(data); },

			VRAM_START ... VRAM_END => {
				// Disallow writes to VRAM depending on the mode
				if self.get_mode() == StatusMode::Transfer {
					return;
				}

                let offset = address - VRAM_START;
                match bank {
                	0 => self.vram[offset as usize] = data,
                	1 => self.cgb_vram[offset as usize] = data,
                	_ => unreachable!(),
                };

				// Mark this data as dirty so the tile cache updates
				if address <= TILE_RAM_END {
					let tile_id = offset / 16;

					match bank {
	                	0 => self.tile_cache[tile_id as usize].dirty = true,
	                	1 => self.cgb_tile_cache[tile_id as usize].dirty = true,
	                	_ => unreachable!(),
	                };

				}
			},

			OAM_START ... OAM_END => {
				match self.get_mode() {
					StatusMode::Oam | StatusMode::Transfer => { return; },
					_ => {
						self.oam[(address - OAM_START) as usize] = data;
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
			0 => sprite.y_pos = data as i32 - 16,
			1 => sprite.x_pos = data as i32 - 8,
			2 => sprite.tile_id = data,
			3 => {
				sprite.behind_background = (data & Bit::Bit7 as u8) > 0;
				sprite.y_flip = (data & Bit::Bit6 as u8) > 0;
				sprite.x_flip = (data & Bit::Bit5 as u8) > 0;
				sprite.use_palette_one = (data & Bit::Bit4 as u8) > 0;
				sprite.bank = (data & Bit::Bit3 as u8) >> 3;
				sprite.palette = data & 0x7;
			},
			_ => unreachable!()
		};
	}

	fn update_lcdc(&mut self, data: u8) {
		let new = MemoryRegister::new(data);

		if !new.is_set(Bit::Bit7) && self.display_enabled() {
			if self.get_mode() != StatusMode::VBlank {
				//panic!("LCD off, but not in VBlank");
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

    pub fn get_vram_bank(&self) -> u8 {
        self.vram_bank
    }

    pub fn set_vram_bank(&mut self, data: u8) {
        self.vram_bank = data & 0x1;
    }

    /***** SPRITE PALETTE DATA *****/

    pub fn get_sprite_pal_index(&self) -> u8 {
    	self.OBPI.get()
    }

	pub fn set_sprite_pal_index(&mut self, data: u8) {
		self.OBPI.set(data);
	}

	pub fn get_sprite_pal_data(&self) -> u8 {
		let pal_index = self.get_sprite_pal_index();
		let index = pal_index & 0x3F;
		self.sprite_pal_ram[index as usize]
	}

	pub fn set_sprite_pal_data(&mut self, data: u8) {

		let pal_index = self.get_sprite_pal_index();
		let index = pal_index & 0x3F;
		let auto_increment = pal_index & Bit::Bit7 as u8;

		self.sprite_pal_ram[index as usize] = data;

		if auto_increment > 0 {
			let mut new_index = index + 1;
			if new_index > 63 {
				new_index = 0;
			}
			let adjusted = auto_increment | new_index;
			self.OBPI.set(adjusted);
		}
		println!("{:02x} written to palette RAM ID: {}", data, index);
	}

	fn get_sprite_pal_color(&self, palette_id: u8, color_id: u8) -> u32 {
		let offset = palette_id * 8;
		let index = offset + (color_id * 2);
		let first_byte  = self.sprite_pal_ram[index as usize];
		let second_byte = self.sprite_pal_ram[(index + 1) as usize];

		let combined = ((second_byte as u16) << 8) | first_byte as u16;

		let red = combined & 0x1F;
		let green = (combined >> 5) & 0x1F;
		let blue = (combined >> 10) & 0x1F;

		//FLOOR(256/32,1)*K4
		let r = (256 / 32) * red;
		let g = (256 / 32) * green;
		let b = (256 / 32) * blue;

		let rgb = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;

		rgb
	}

    pub fn get_bg_pal_index(&self) -> u8 {
    	self.BGPI.get()
    }

	pub fn set_bg_pal_index(&mut self, data: u8) {
		self.BGPI.set(data);
	}

	pub fn get_bg_pal_data(&self) -> u8 {
		let pal_index = self.get_bg_pal_index();
		let index = pal_index & 0x3F;
		self.cgb_pal_ram[index as usize]
	}

	pub fn set_bg_pal_data(&mut self, data: u8) {

		//if self.get_mode() == StatusMode::Transfer { return; }

		let pal_index = self.get_bg_pal_index();
		let index = pal_index & 0x3F;
		let auto_increment = pal_index & Bit::Bit7 as u8;

		self.cgb_pal_ram[index as usize] = data;

		if auto_increment > 0 {
			let mut new_index = index + 1;
			if new_index > 63 {
				new_index = 0;
			}
			let adjusted = auto_increment | new_index;
			self.BGPI.set(adjusted);
		}
		//println!("{:02x} written to {}", data, index);
		//println!("{:?}", self.cgb_pal_ram);
	}

	fn get_cgb_pal_color(&self, palette_id: u8, color_id: u8) -> u32 {
		let offset = palette_id * 8;
		let index = offset + (color_id * 2);
		let first_byte  = self.cgb_pal_ram[index as usize];
		let second_byte = self.cgb_pal_ram[(index + 1) as usize];

		let combined = ((second_byte as u16) << 8) | first_byte as u16;

		let red = combined & 0x1F;
		let green = (combined >> 5) & 0x1F;
		let blue = (combined >> 10) & 0x1F;

		//FLOOR(256/32,1)*K4
		let r = (256 / 32) * red;
		let g = (256 / 32) * green;
		let b = (256 / 32) * blue;

		let rgb = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;

		rgb
	}

	// Using color gameboy palette data
	// Converts a 0-3 shade to the appropriate 32bit palette color
	fn cgb_colorize(&self, shade: u8, palette_id: u8, palette: u8) -> u32 {
		let color_values = [
			self.get_cgb_pal_color(palette_id, 0), // 0
			self.get_cgb_pal_color(palette_id, 1), // 1
			self.get_cgb_pal_color(palette_id, 2), // 2
			self.get_cgb_pal_color(palette_id, 3), // 3
		];
		color_values[shade as usize]
	}

	// Using color gameboy palette data
	// Converts a 0-3 shade to the appropriate 32bit palette color for sprites
	fn sprite_colorize(&self, shade: u8, palette_id: u8, palette: u8) -> u32 {
		let color_values = [
			self.get_sprite_pal_color(palette_id, 0), // 0
			self.get_sprite_pal_color(palette_id, 1), // 1
			self.get_sprite_pal_color(palette_id, 2), // 2
			self.get_sprite_pal_color(palette_id, 3), // 3
		];
		color_values[shade as usize]
	}

	pub fn dump(&self) {
		println!("DUMPING VRAM");
		dump("vram.bin", &self.vram);
		dump("oam.bin", &self.oam);
	}
}
