use core::helper::*;
use core::memory_map::*;

pub enum InterruptFlag {
	VBlank = 0b00000001,
	Lcdc   = 0b00000010,
	Timer  = 0b00000100,
	Serial = 0b00001000,
	Joypad = 0b00010000,
}

#[derive(Debug)]
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

	pub fn read(&mut self, address: u16) -> u8 {
		match address {
			IE => self.IE.get(),
			IF => self.IF.get(),
			_ => panic!("Unknown read from InterruptHandler")
		}
	}

	pub fn write(&mut self, address: u16, data: u8) {
		match address {
			IE => self.IE.set(data),
			IF => self.IF.set(data),
			_ => panic!("Unknown read from InterruptHandler")
		}
	}

	// Returns an interrupt vector if there is an interrupt to be handled
	pub fn execute_next(&mut self) -> Option<InterruptVector> {
		let mut address = None;
		if self.master_enable {
			let interrupt_flag = self.IF.get(); // Interrupt Flag
			let interrupt_enable = self.IE.get(); // Interrupt Enable
			for index in 0..5 {
				let bit = 1 << index;
				if interrupt_enable & bit > 0 {
					if interrupt_flag & bit > 0 {
						self.IF.set(interrupt_flag & !bit);
						address = match bit {
							0b00000001 => Some(InterruptVector::VBlank),
							0b00000010 => Some(InterruptVector::Lcdc),
							0b00000100 => Some(InterruptVector::Timer),
							0b00001000 => Some(InterruptVector::Serial),
							0b00010000 => Some(InterruptVector::Joypad),
							_ => unreachable!("BIT: {:08b}", bit),
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
