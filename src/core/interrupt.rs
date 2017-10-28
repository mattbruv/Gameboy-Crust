

pub struct InterruptHandler {
	pub enabled: bool,
	counter: u32,
}

impl InterruptHandler {
	pub fn new() -> InterruptHandler {
		InterruptHandler {
			enabled: true,
			counter: 0,
		}
	}
}