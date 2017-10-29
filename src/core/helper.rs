pub enum Bit {
	Bit0 = 0b00000001,
	Bit1 = 0b00000010,
	Bit2 = 0b00000100,
	Bit3 = 0b00001000,
	Bit4 = 0b00010000,
	Bit5 = 0b00100000,
	Bit6 = 0b01000000,
	Bit7 = 0b10000000,
}

// "It is considered poor style to implement methods on such primitive types, even though it is possible."
// - Rust Documentation

// ...I'm going to do it anyway. ¯\_(ツ)_/¯
pub trait Helper<T> {
	fn set_bit(&mut self, b: Bit) -> T;
	fn clear_bit(&mut self, b: Bit) -> T;
	fn is_set(&self, b: Bit) -> bool;
}

impl Helper<u8> for u8 {
	fn set_bit(&mut self, b: Bit) -> u8 {
		*self | b as u8
	}

	fn clear_bit(&mut self, b: Bit) -> u8 {
		*self & !(b as u8)
	}

	fn is_set(&self, b: Bit) -> bool {
		*self & b as u8 > 0
	}
}

// combine two u8s into a u16
macro_rules! combine {
	($h:expr, $l:expr) => (
		(($h as u16) << 8) | $l as u16
	)
}

// return high byte from u16
macro_rules! high {
	($word:expr) => (
		($word >> 8) as u8
	)
}

// return low byte from u16
macro_rules! low {
	($word:expr) => (
		($word & 0xFF) as u8
	)
}

// print number as hexidecimal
macro_rules! hex {
	($val:expr) => {{
		println!("${:02X}", $val);
	}}
}