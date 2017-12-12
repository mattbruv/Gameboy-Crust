use core::mbc::*;
use core::memory_map::*;
use core::helper::*;

pub struct MBC5 {
    title: String,
    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
    eram: Vec<u8>,
}

impl MBC5 {
    pub fn new() -> MBC5 {
        MBC5 {
            title: "".to_owned(),
            rom_bank: 0x10,
            ram_bank: 0x00,
            ram_enabled: false,
            eram: vec![0; 0xFFFFF],
        }
    }
}

impl MemoryController for MBC5 {

    fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
        match address {
            ROM_START ... ROM_END => {
                bytes[address as usize]
            },
            ROM_BANK_START ... ROM_BANK_END => {
                let index = self.rom_bank * 0x4000 | ((address as usize) & 0x3FFF);
                bytes[index]
            },
            ERAM_START ... ERAM_END => {
                if !self.ram_enabled { return 0xFF; }
                let index = address - ERAM_START;
                let offset = (0x2000 as u32 * self.ram_bank as u32) + index as u32;
                self.eram[offset as usize]
            },
            _ => { unreachable!(); }
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000 ... 0x1FFF => {
                self.ram_enabled = data == 0x0A;
            },
            0x2000 ... 0x2FFF => {
                let lower_bits = data as usize;
                self.rom_bank = (self.rom_bank & 0x100) | lower_bits;
            },
            0x3000 ... 0x3FFF => {
                let ninth_bit: usize = (((data & 0x1) as usize) << 8);
                self.rom_bank = (self.rom_bank & 0x0FF) | ninth_bit;
            },
            0x4000 ... 0x5FFF => {
                self.ram_bank = (data & 0x0F) as usize;
            },
            0x6000 ... 0x7FFF => {},
            // ERAM writes
            ERAM_START ... ERAM_END => {
                if !self.ram_enabled { return; }
                let index = address - ERAM_START;
                let offset = (0x2000 as u32 * self.ram_bank as u32) + index as u32;
                self.eram[offset as usize] = data;
            },
            _ => { unreachable!(); }
        }
    }

    fn set_title(&mut self, name: String) {
        self.title = name;
    }

    fn load(&mut self) {
        let mut title = self.title.clone();
        title.push_str(".sav");
        load(title, &mut self.eram);
    }
}

impl Drop for MBC5 {
    fn drop(&mut self) {
        let mut filename = self.title.clone();
        filename.push_str(".sav");
        dump(&filename, &self.eram);
    }
}
