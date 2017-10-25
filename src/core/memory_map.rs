// 16 KB ROM Bank 00 (in cartridge, fixed at bank 00)
pub const ROM_BANK_0_START: u16 = 0x0000;
pub const ROM_BANK_0_END: u16   = 0x3FFF;

// 165KB ROM Bank 01..XX (in cartridge, switchable bank number)
pub const ROM_BANK_X_START: u16 = 0x4000;
pub const ROM_BANK_X_END: u16   = 0x7FFF;

// 8KB Video RAM (VRAM)
pub const VRAM_START: u16 = 0x4000;
pub const VRAM_END: u16   = 0x9FFF;

// 8KB External RAM (in cartridge, switchable bank, if any)
pub const EXTERNAL_RAM_START: u16 = 0xA000;
pub const EXTERNAL_RAM_END: u16   = 0xBFFF;

// 4KB Work RAM Bank 0 (WRAM)
pub const WRAM_BANK_0_START: u16 = 0xC000;
pub const WRAM_BANK_0_END: u16   = 0xCFFF;

// 4KB Work RAM Bank 1 (switchable bank 1-7 in CGB Mode)
pub const WRAM_BANK_X_START: u16 = 0xD000;
pub const WRAM_BANK_X_END: u16   = 0xDFFF;

// Same as C000-DDFF (ECHO or Mirror RAM) (Typically not used)
pub const ECHO_RAM_START: u16 = 0xE000;
pub const ECHO_RAM_END: u16   = 0xFDFF;

// 160 byte Sprite Attribute Table (OAM)
pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16   = 0xFE9F;

// 352 bytes of High RAM
pub const HRAM_START: u16 = 0xFEA0;
pub const HRAM_END: u16   = 0xFFFF;