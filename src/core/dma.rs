use memory_map::*;
use interconnect::*;

// Direct Memory Access

pub struct OamDma {
    pub active: bool,
    source: u16,
    destination: u16,
    cycles: i32,
}

impl OamDma {
    pub fn new() -> OamDma {
        OamDma {
            active: false,
            source: 0x0000,
            destination: 0x0000,
            // 2 cycles to transfer 16 bytes
            // 1 cycle per 8 bytes (40 * 4bytes per obj) = 160 bytes / 8 bytes per cycle = 20
            cycles: 20,
        }
    }

    pub fn request(&mut self, source: u8) {
        self.active = true;
        self.source = (source as u16) * 0x100;
        self.destination = OAM_START;
    }

    // Returns how many bytes to transfer based on number of cycles passed
    pub fn cycles(&mut self, cycles: usize) -> (u16, u16, u8) {
        let from = self.source;
        let to = self.destination;
        let mut adjusted_cycles = cycles as i32;
        if (self.cycles - adjusted_cycles < 0) {
            adjusted_cycles = self.cycles;
        }
        let bytes = (adjusted_cycles * 8) as u8;
        self.source = self.source.wrapping_add(bytes as u16);
        self.destination = self.destination.wrapping_add(bytes as u16);
        self.cycles -= adjusted_cycles;
        if self.cycles == 0 {
            self.active = false;
            self.cycles = 20;
        }
        (from, to, bytes)
    }
}
