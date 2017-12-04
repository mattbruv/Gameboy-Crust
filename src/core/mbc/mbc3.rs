use core::mbc::*;
use core::memory_map::*;

pub struct MBC3 {
    rom_bank: u8,
    ram_bank: u8,
    rtc_register: u8,
    ram_timer_enable: bool,
    select_ram_bank: bool,
    eram: Vec<u8>,
}

impl MBC3 {
    pub fn new() -> MBC3 {
        MBC3 {
            rom_bank: 0x01,
            ram_bank: 0x00,
            rtc_register: 0x00,
            ram_timer_enable: false,
            select_ram_bank: false,
            eram: vec![0; 0x8000],
        }
    }
}

impl MemoryController for MBC3 {
    
    fn read(&self, bytes: &Vec<u8>, address: u16) -> u8 {
        match address {
            ROM_START ... ROM_END => {
                bytes[address as usize]
            },
            ROM_BANK_START ... ROM_BANK_END => {
                let bank = self.rom_bank as usize * 0x4000;
                let offset = address - ROM_BANK_START;
                let index = bank + offset as usize; 
                bytes[index]
            },
            ERAM_START ... ERAM_END => {
                if self.ram_timer_enable {
                    let offset = address - ERAM_START;
                    let bank = self.ram_bank as usize * 0x2000;
                    let index = bank + offset as usize;
                    self.eram[index]
                } else { 0x00 } // May be 0xFF
            },
            _ => { unreachable!(); }
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000 ... 0x1FFF => {
                // RAM/Timer enable write
                self.ram_timer_enable = match data {
                    0x0A => true,
                    0x00 => false,
                    _ => unreachable!()
                };
            },
            0x2000 ... 0x3FFF => {
                // ROM Bank Number write
                let mut bank = data & 0x7F;
                if bank == 0 { bank = 1; }
                self.rom_bank = bank;
            },
            0x4000 ... 0x5FFF => {
                // RAM Bank number or RTC Register Select write
                // We only select the RAM bank for now
                if self.ram_timer_enable { // This may not be necessary
                    let bank = data & 0x3;

                    // We only care about the RAM bank at the moment
                    if bank <= 3 {
                        self.ram_bank = bank;
                    }
                }
            },
            0x6000 ... 0x7FFF => {
                // Latch Clock Data write   
            },
            ERAM_START ... ERAM_END => {
                if self.ram_timer_enable {
                    let offset = address - ERAM_START;
                    let bank = self.ram_bank as usize * 0x2000;
                    let index = bank + offset as usize;
                    self.eram[index] = data;
                }
            },
            _ => { unreachable!(); }
        }    
    }

}

