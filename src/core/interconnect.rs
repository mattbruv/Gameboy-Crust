use rom::*;

pub struct Interconnect {
	rom: Rom
}

impl Interconnect {
	pub fn new(_rom: Rom) -> Interconnect {
		Interconnect {
			rom: _rom
		}
	}
}