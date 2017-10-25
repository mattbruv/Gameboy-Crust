use rom::*;
use vram::*;
use wram::*;
use oam::*;
use hram::*;

pub struct Interconnect {
	rom: Rom,
	vram: Vram,
	wram: Wram,
	oam: Oam,
	hram: Hram
}

impl Interconnect {
	pub fn new(_rom: Rom) -> Interconnect {
		Interconnect {
			rom: _rom,
			vram: Vram::new(),
			wram: Wram::new(),
			oam: Oam::new(),
			hram: Hram::new()
		}
	}

	pub fn test(&self) {
		println!("size: {}", self.wram.size());
	}
}