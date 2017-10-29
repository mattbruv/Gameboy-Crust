pub enum InterruptFlag {
	Vblank = 0b00000001,
	Lcdc   = 0b00000010,
	Timer  = 0b00000100,
	Serial = 0b00001000,
	Joypad = 0b00010000,
}

pub struct InterruptHandler {

	counter: u32, // counts the number of opcodes since interrupt status changed

	master_enable: bool,
	reg_ie: u8, // used to control intterupts
	reg_if: u8, // indicates which type of interrupt is set 
}

impl InterruptHandler {
	pub fn new() -> InterruptHandler {
		InterruptHandler {
			counter: 0,
			master_enable: false,
			reg_ie: 0x00,
			reg_if: 0x00,
		}
	}

	pub fn enable(&mut self) {
		self.master_enable = true;
	}

	pub fn disable(&mut self) {
		self.master_enable = false;
	}
}