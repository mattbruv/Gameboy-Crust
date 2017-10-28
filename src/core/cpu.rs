use core::register::*;
use core::interconnect::*;

pub struct CPU {
	regs: Registers
}

impl CPU {

	// Initialize CPU state
	pub fn new() -> CPU {
		CPU {
			regs: Registers::new()
		}
	}

	// Reads the next byte and increments the program counter
	fn next_byte(&mut self, memory: &Interconnect) -> u8 {
		let byte = memory.read(self.regs.pc);
		self.regs.pc = self.regs.pc.wrapping_add(1);
		byte
	}

	// Returns the next word (pointer, little endian)
	fn next_pointer(&mut self, memory: &Interconnect) -> u16 {
		let low  = self.next_byte(memory);
		let high = self.next_byte(memory);
		combine!(high, low)
	}

	// Pushes 16 bit data onto the stack
	fn push(&mut self, memory: &mut Interconnect, data: u16) {
		self.regs.sp.wrapping_sub(1);
		memory.write(self.regs.sp, high!(data));
		self.regs.sp.wrapping_sub(1);
		memory.write(self.regs.sp, low!(data));
	}

	// Pops highest 16 bits from stack
	fn pop(&mut self, memory: &mut Interconnect) -> u16 {
		let low = memory.read(self.regs.sp);
		self.regs.sp.wrapping_add(1);
		let high = memory.read(self.regs.sp);
		self.regs.sp.wrapping_add(1);
		combine!(high, low)
	}

	fn add_u8(&mut self, n: u8, use_carry: bool) {
		let a = self.regs.a;
		let c = (use_carry && self.regs.is_flag_set(Flag::Carry)) as u8;
		let result = a.wrapping_add(n).wrapping_add(c);
		let carry = (a as i16 + n as i16 + c as i16) > 0xFF;
		let half_carry = (a & 0xF) + (n & 0xF) + c > 0xF;
		self.regs.set_flag(Flag::Carry, carry);
		self.regs.set_flag(Flag::HalfCarry, half_carry);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0));
		self.regs.a = result;
	}

	fn sub_u8(&mut self, n: u8, use_carry: bool) {
		let a = self.regs.a;
		let c = (use_carry && self.regs.is_flag_set(Flag::Carry)) as u8;
		let result = a.wrapping_sub(n).wrapping_sub(c);
		let carry = (a as i16 - n as i16 - c as i16) < 0;
		let half_carry = (a & 0xF) as i16 - (n & 0xF) as i16 - (c as i16) < 0;
		self.regs.set_flag(Flag::Carry, carry);
		self.regs.set_flag(Flag::HalfCarry, half_carry);
		self.regs.set_flag(Flag::Sub, true);
		self.regs.set_flag(Flag::Zero, (result == 0));
		self.regs.a = result;
	}

	// Perform one step of the fetch-decode-execute cycle 
	pub fn step(&mut self, memory: &mut Interconnect) -> usize {

		println!("PC: ${:04X}", self.regs.pc);
		let opcode = self.next_byte(memory);
		hex!(opcode);

		// decodes/excecutes each operation and returns cycles taken
		match opcode {
			// 8-bit transfers
			// LD r, r'
			0x7F => { 1 }, // LD A, A
			0x78 => { self.regs.a = self.regs.b; 1 },
			0x79 => { self.regs.a = self.regs.c; 1 },
			0x7A => { self.regs.a = self.regs.d; 1 },
			0x7B => { self.regs.a = self.regs.e; 1 },
			0x7C => { self.regs.a = self.regs.h; 1 },
			0x7D => { self.regs.a = self.regs.l; 1 },
			0x47 => { self.regs.b = self.regs.a; 1 },
			0x40 => { 1 }, // LD B, B
			0x41 => { self.regs.b = self.regs.c; 1 },
			0x42 => { self.regs.b = self.regs.d; 1 },
			0x43 => { self.regs.b = self.regs.e; 1 },
			0x44 => { self.regs.b = self.regs.h; 1 },
			0x45 => { self.regs.b = self.regs.l; 1 },
			0x4F => { self.regs.c = self.regs.a; 1 },
			0x48 => { self.regs.c = self.regs.b; 1 },
			0x49 => { 1 }, // LD C, C
			0x4A => { self.regs.c = self.regs.d; 1 },
			0x4B => { self.regs.c = self.regs.e; 1 },
			0x4C => { self.regs.c = self.regs.h; 1 },
			0x4D => { self.regs.c = self.regs.l; 1 },
			0x57 => { self.regs.d = self.regs.a; 1 },
			0x50 => { self.regs.d = self.regs.b; 1 },
			0x51 => { self.regs.d = self.regs.c; 1 },
			0x52 => { 1 }, // LD D, D
			0x53 => { self.regs.d = self.regs.e; 1 },
			0x54 => { self.regs.d = self.regs.h; 1 },
			0x55 => { self.regs.d = self.regs.l; 1 },
			0x5F => { self.regs.e = self.regs.a; 1 },
			0x58 => { self.regs.e = self.regs.b; 1 },
			0x59 => { self.regs.e = self.regs.c; 1 },
			0x5A => { self.regs.e = self.regs.d; 1 },
			0x5B => { 1 }, // LD E, E
			0x5C => { self.regs.e = self.regs.h; 1 },
			0x5D => { self.regs.e = self.regs.l; 1 },
			0x67 => { self.regs.h = self.regs.a; 1 },
			0x60 => { self.regs.h = self.regs.b; 1 },
			0x61 => { self.regs.h = self.regs.c; 1 },
			0x62 => { self.regs.h = self.regs.d; 1 },
			0x63 => { self.regs.h = self.regs.h; 1 },
			0x64 => { 1 }, // LD H, H
			0x65 => { self.regs.h = self.regs.l; 1 },
			0x6F => { self.regs.l = self.regs.a; 1 },
			0x68 => { self.regs.l = self.regs.b; 1 },
			0x69 => { self.regs.l = self.regs.c; 1 },
			0x6A => { self.regs.l = self.regs.d; 1 },
			0x6B => { self.regs.l = self.regs.h; 1 },
			0x6D => { self.regs.l = self.regs.l; 1 },
			0x6C => { 1 }, // LD L, L
			// LD r, n
			0x3E => { self.regs.a = self.next_byte(memory); 2 },
			0x06 => { self.regs.b = self.next_byte(memory); 2 },
			0x0E => { self.regs.c = self.next_byte(memory); 2 },
			0x16 => { self.regs.d = self.next_byte(memory); 2 },
			0x1E => { self.regs.e = self.next_byte(memory); 2 },
			0x26 => { self.regs.h = self.next_byte(memory); 2 },
			0x2E => { self.regs.l = self.next_byte(memory); 2 },
			// LD r, (HL)
			0x7E => { self.regs.a = memory.read(self.regs.hl()); 2 },
			0x46 => { self.regs.b = memory.read(self.regs.hl()); 2 },
			0x4E => { self.regs.c = memory.read(self.regs.hl()); 2 },
			0x56 => { self.regs.d = memory.read(self.regs.hl()); 2 },
			0x5E => { self.regs.e = memory.read(self.regs.hl()); 2 },
			0x66 => { self.regs.h = memory.read(self.regs.hl()); 2 },
			0x6E => { self.regs.l = memory.read(self.regs.hl()); 2 },
			// LD (HL), r
			0x77 => { memory.write(self.regs.hl(), self.regs.a); 2 },
			0x70 => { memory.write(self.regs.hl(), self.regs.b); 2 },
			0x71 => { memory.write(self.regs.hl(), self.regs.c); 2 },
			0x72 => { memory.write(self.regs.hl(), self.regs.d); 2 },
			0x73 => { memory.write(self.regs.hl(), self.regs.e); 2 },
			0x74 => { memory.write(self.regs.hl(), self.regs.h); 2 },
			0x75 => { memory.write(self.regs.hl(), self.regs.l); 2 },
			// LD (HL), n
			0x36 => { let n = self.next_byte(memory); memory.write(self.regs.hl(), n); 3 },
			// LD A, (BC)
			0x0A => { self.regs.a = memory.read(self.regs.bc()); 2 },
			// LD A, (DE)
			0x1A => { self.regs.a = memory.read(self.regs.de()); 2 },
			// LD A, (C)
			0xF2 => { self.regs.a = memory.read((0xFF00 as u16).wrapping_add(self.regs.c as u16)); 2 },
			// LD (C), A
			0xE2 => { memory.write((0xFF00 as u16).wrapping_add(self.regs.c as u16), self.regs.a); 2 }, 
			// LD A, (n)
			0xF0 => { self.regs.a = memory.read((0xFF00 as u16).wrapping_add(self.next_byte(memory) as u16)); 3 },
			// LD (n), A
			0xE0 => { let n = self.next_byte(memory); memory.write((0xFF00 as u16).wrapping_add(n as u16), self.regs.a); 3 },
			// LD A, (nn)
			0xFA => { self.regs.a = memory.read(self.next_pointer(memory)); 4 },
			// LD (nn), A
			0xEA => { let addr = self.next_pointer(memory); memory.write(addr, self.regs.a); 4 },
			// LD A (HLI)			
			0x2A => { self.regs.a = memory.read(self.regs.hl()); self.regs.hli(); 2 },
			// LD A (HLD)
			0x3A => { self.regs.a = memory.read(self.regs.hl()); self.regs.hld(); 2 },
			// LD (BC), A
			0x02 => { memory.write(self.regs.bc(), self.regs.a); 2 },
			// LD (DE), A
			0x12 => { memory.write(self.regs.de(), self.regs.a); 2 },
			// LD (HLI), A
			0x22 => { memory.write(self.regs.hl(), self.regs.a); self.regs.hli(); 2 },
			// LD (HLD), A
			0x32 => { memory.write(self.regs.hl(), self.regs.a); self.regs.hld(); 2 },

			// 16-bit transfers
			// LD dd, nn
			0x01 => { let nn = self.next_pointer(memory); self.regs.set_bc(nn); 3 },
			0x11 => { let nn = self.next_pointer(memory); self.regs.set_de(nn); 3 },
			0x21 => { let nn = self.next_pointer(memory); self.regs.set_hl(nn); 3 },
			0x31 => { let nn = self.next_pointer(memory); self.regs.sp = nn;    3 },
			// LD SP, HL
			0xF9 => { self.regs.sp = self.regs.hl(); 2 },
			// PUSH qq
			0xC5 => { let nn = self.regs.bc(); self.push(memory, nn); 4 },
			0xD5 => { let nn = self.regs.de(); self.push(memory, nn); 4 },
			0xE5 => { let nn = self.regs.hl(); self.push(memory, nn); 4 },
			0xF5 => { let nn = self.regs.af(); self.push(memory, nn); 4 },
			// POP qq
			0xC1 => { let qq = self.pop(memory); self.regs.set_bc(qq); 3 },
			0xD1 => { let qq = self.pop(memory); self.regs.set_de(qq); 3 },
			0xE1 => { let qq = self.pop(memory); self.regs.set_hl(qq); 3 },
			0xF1 => { let qq = self.pop(memory); self.regs.set_af(qq); 3 },
			// LD SP, e (e = imm8 -128 to +127)
			0xF8 => { unimplemented!(); },
			// LD (nn), SP
			0x08 => {
				let nn = self.next_pointer(memory);
				memory.write(nn, low!(self.regs.sp));
				memory.write(nn.wrapping_add(1), high!(self.regs.sp));
				5
			},
			// 8-bit Arithmetic
			// ADD A, r
			0x87 => { let r = self.regs.a; self.add_u8(r, false); 1 },
			0x80 => { let r = self.regs.b; self.add_u8(r, false); 1 },
			0x81 => { let r = self.regs.c; self.add_u8(r, false); 1 },
			0x82 => { let r = self.regs.d; self.add_u8(r, false); 1 },
			0x83 => { let r = self.regs.e; self.add_u8(r, false); 1 },
			0x84 => { let r = self.regs.h; self.add_u8(r, false); 1 },
			0x85 => { let r = self.regs.l; self.add_u8(r, false); 1 },
			// ADD A, n
			0xC6 => { let n = self.next_byte(memory); self.add_u8(n, false); 2 },
			// ADD A, (HL)
			0x86 => { let hl = memory.read(self.regs.hl()); self.add_u8(hl, false); 2 },
			// ADC A, r
			0x8F => { let r = self.regs.a; self.add_u8(r, true); 1 },
			0x88 => { let r = self.regs.b; self.add_u8(r, true); 1 },
			0x89 => { let r = self.regs.c; self.add_u8(r, true); 1 },
			0x8A => { let r = self.regs.d; self.add_u8(r, true); 1 },
			0x8B => { let r = self.regs.e; self.add_u8(r, true); 1 },
			0x8C => { let r = self.regs.h; self.add_u8(r, true); 1 },
			0x8D => { let r = self.regs.l; self.add_u8(r, true); 1 },			
			// ADC A, n
			0xCE => { let n = self.next_byte(memory); self.add_u8(n, true); 2 },
			// ADC A, (HL)
			0x8E => { let hl = memory.read(self.regs.hl()); self.add_u8(hl, true); 2 },
			// SUB A, r
			0x97 => { let r = self.regs.a; self.sub_u8(r, false); 1 },
			0x90 => { let r = self.regs.b; self.sub_u8(r, false); 1 },
			0x91 => { let r = self.regs.c; self.sub_u8(r, false); 1 },
			0x92 => { let r = self.regs.d; self.sub_u8(r, false); 1 },
			0x93 => { let r = self.regs.e; self.sub_u8(r, false); 1 },
			0x94 => { let r = self.regs.h; self.sub_u8(r, false); 1 },
			0x95 => { let r = self.regs.l; self.sub_u8(r, false); 1 },
			// SUB A, n
			0xD6 => { let n = self.next_byte(memory); self.sub_u8(n, false); 2 },
			// SUB A, (HL)
			0x96 => { let hl = memory.read(self.regs.hl()); self.sub_u8(hl, false); 2 },
			// SBC A, r
			0x9F => { let r = self.regs.a; self.sub_u8(r, true); 1 },
			0x98 => { let r = self.regs.b; self.sub_u8(r, true); 1 },
			0x99 => { let r = self.regs.c; self.sub_u8(r, true); 1 },
			0x9A => { let r = self.regs.d; self.sub_u8(r, true); 1 },
			0x9B => { let r = self.regs.e; self.sub_u8(r, true); 1 },
			0x9C => { let r = self.regs.h; self.sub_u8(r, true); 1 },
			0x9D => { let r = self.regs.l; self.sub_u8(r, true); 1 },			
			// SBC A, n
			0xDE => { let n = self.next_byte(memory); self.sub_u8(n, true); 2 },
			// SBC A, (HL)
			0x9E => { let hl = memory.read(self.regs.hl()); self.sub_u8(hl, true); 2 },




			0xC3 => { self.regs.pc = self.next_pointer(memory); 4 }
			_ => panic!("Unknown Opcode: ${:02X} | {}", opcode, opcode)
		}
	}

	pub fn debug(&mut self) {
		self.regs.a = 0x3b;
		self.regs.f = 0x00;
		self.regs.set_flag(Flag::Carry, true);
		println!("{}", self.regs);
		self.sub_u8(0x45 as u8, true);
		println!("{}", self.regs);
	}

}