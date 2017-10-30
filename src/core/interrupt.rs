use core::helper::*;

pub enum InterruptFlag {
	Vblank = 0b00000001,
	Lcdc   = 0b00000010,
	Timer  = 0b00000100,
	Serial = 0b00001000,
	Joypad = 0b00010000,
}

pub enum InterruptVector {
	VBlank = 0x0040,
	Lcdc   = 0x0048,
	Timer  = 0x0050,
	Serial = 0x0058,
	Joypad = 0x0060,
}

pub struct InterruptHandler {

	counter: u32, // counts the number of opcodes since interrupt status changed

	master_enable: bool,
	pub IE: MemoryRegister, // used to control intterupts
	pub IF: MemoryRegister, // indicates which type of interrupt is set 
}

impl InterruptHandler {
	pub fn new() -> InterruptHandler {
		InterruptHandler {
			counter: 0,
			master_enable: false,
			IE: MemoryRegister::new(0xFA),
			IF: MemoryRegister::new(0x00),
		}
	}

	pub fn enable(&mut self) {
		self.master_enable = true;
	}

	pub fn disable(&mut self) {
		self.master_enable = false;
	}

	// Requests an interrupt
	pub fn request_interrupt(flag: InterruptFlag) {
		unimplemented!();
	}
}