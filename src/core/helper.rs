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