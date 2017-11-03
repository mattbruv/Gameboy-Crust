use core::helper::*;

pub enum InterruptFlag {
	VBlank = 0b00000001,
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

	// Returns an interrupt vector if there is an interrupt to be handled
	pub fn execute_next(&mut self) -> Option<InterruptVector> {
		let mut address = None;
		if self.master_enable {
			let IF = self.IF.get(); // Interrupt Flag
			let IE = self.IE.get(); // Interrupt Enable
			for bit in 0..5 {
				if IF & bit > 0 {
					if IE & bit > 0 {
						self.IF.set(IF & !bit);
						address = match bit {
							0 => Some(InterruptVector::VBlank),
							1 => Some(InterruptVector::Lcdc),
							2 => Some(InterruptVector::Timer),
							3 => Some(InterruptVector::Serial),
							4 => Some(InterruptVector::Joypad),
							_ => unreachable!(),
						};
						break;
					}
				}
			}
		}
		address
	}

	// Requests an interrupt
	pub fn request_interrupt(&mut self, flag: InterruptFlag) {
		let mut register = self.IF.get();
		register |= flag as u8;
		self.IF.set(register);
	}
}
