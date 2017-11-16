use core::register::*;
use core::interconnect::*;
use core::helper::*;
use core::disassembler::*;

enum Condition {
	NotZero  = 0b00,
	Zero     = 0b01,
	NotCarry = 0b10,
	Carry    = 0b11,
}

// static addresses for RST instruction
enum RstVector {
	Rst1 = 0x00,
	Rst2 = 0x08,
	Rst3 = 0x10,
	Rst4 = 0x18,
	Rst5 = 0x20,
	Rst6 = 0x28,
	Rst7 = 0x30,
	Rst8 = 0x38,
}

pub struct CPU {
	pub regs: Registers,
	halted: bool,
}

impl CPU {

	// Initialize CPU state
	pub fn new() -> CPU {
		CPU {
			regs: Registers::new(),
			halted: false,
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

	// Perform one step of the fetch-decode-execute cycle
	pub fn step(&mut self, memory: &mut Interconnect) -> usize {

		self.handle_interrupts(memory);

		if self.halted {
			return 1;
		}

		let old_pc = self.regs.pc;
		let opcode = self.next_byte(memory);
		//let command = disassemble(&self.regs, &memory, opcode);
		//println!("{}", command);

		//println!("PC: 0x{:04X}: ${:02X}", self.regs.pc - 1, opcode);

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
			0xF2 => {
				let offset = 0xFF00 | self.regs.c as u16;
				self.regs.a = memory.read(offset);
				2
			},
			// LD (C), A
			0xE2 => {
				let offset = 0xFF00 | self.regs.c as u16;
				memory.write(offset, self.regs.a);
				2
			},
			// LD A, (n)
			0xF0 => {
				let offset = 0xFF00 | self.next_byte(memory) as u16;
				self.regs.a = memory.read(offset);
				3
			},
			// LD (n), A
			0xE0 => {
				let offset = 0xFF00 | self.next_byte(memory) as u16;
				memory.write(offset, self.regs.a);
				3
			},
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
			0xF8 => { let e = self.next_byte(memory); self.ld_hl(e); 3 },
			// LD (nn), SP
			0x08 => {
				let nn = self.next_pointer(memory);
				memory.write(nn, low!(self.regs.sp));
				memory.write(nn + 1, high!(self.regs.sp));
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
			// AND A, r
			0xA7 => { let r = self.regs.a; self.and_u8(r); 1 },
			0xA0 => { let r = self.regs.b; self.and_u8(r); 1 },
			0xA1 => { let r = self.regs.c; self.and_u8(r); 1 },
			0xA2 => { let r = self.regs.d; self.and_u8(r); 1 },
			0xA3 => { let r = self.regs.e; self.and_u8(r); 1 },
			0xA4 => { let r = self.regs.h; self.and_u8(r); 1 },
			0xA5 => { let r = self.regs.l; self.and_u8(r); 1 },
			// AND A, n
			0xE6 => { let n = self.next_byte(memory); self.and_u8(n); 2 },
			// AND A, (HL)
			0xA6 => { let hl = memory.read(self.regs.hl()); self.and_u8(hl); 2 },
			// OR A, r
			0xB7 => { let r = self.regs.a; self.or_u8(r); 1 },
			0xB0 => { let r = self.regs.b; self.or_u8(r); 1 },
			0xB1 => { let r = self.regs.c; self.or_u8(r); 1 },
			0xB2 => { let r = self.regs.d; self.or_u8(r); 1 },
			0xB3 => { let r = self.regs.e; self.or_u8(r); 1 },
			0xB4 => { let r = self.regs.h; self.or_u8(r); 1 },
			0xB5 => { let r = self.regs.l; self.or_u8(r); 1 },
			// OR A, n
			0xF6 => { let n = self.next_byte(memory); self.or_u8(n); 2 },
			// OR A, (HL)
			0xB6 => { let hl = memory.read(self.regs.hl()); self.or_u8(hl); 2 },
			// XOR A, r
			0xAF => { let r = self.regs.a; self.xor_u8(r); 1 },
			0xA8 => { let r = self.regs.b; self.xor_u8(r); 1 },
			0xA9 => { let r = self.regs.c; self.xor_u8(r); 1 },
			0xAA => { let r = self.regs.d; self.xor_u8(r); 1 },
			0xAB => { let r = self.regs.e; self.xor_u8(r); 1 },
			0xAC => { let r = self.regs.h; self.xor_u8(r); 1 },
			0xAD => { let r = self.regs.l; self.xor_u8(r); 1 },
			// XOR A, n
			0xEE => { let n = self.next_byte(memory); self.xor_u8(n); 2 },
			// XOR A, (HL)
			0xAE => { let hl = memory.read(self.regs.hl()); self.xor_u8(hl); 2 },
			// CP A, r
			0xBF => { let r = self.regs.a; self.cp_u8(r); 1 },
			0xB8 => { let r = self.regs.b; self.cp_u8(r); 1 },
			0xB9 => { let r = self.regs.c; self.cp_u8(r); 1 },
			0xBA => { let r = self.regs.d; self.cp_u8(r); 1 },
			0xBB => { let r = self.regs.e; self.cp_u8(r); 1 },
			0xBC => { let r = self.regs.h; self.cp_u8(r); 1 },
			0xBD => { let r = self.regs.l; self.cp_u8(r); 1 },
			// CP A, n
			0xFE => { let n = self.next_byte(memory); self.cp_u8(n); 2 },
			// CP A, (HL)
			0xBE => { let hl = memory.read(self.regs.hl()); self.cp_u8(hl); 2 },
			// INC r
			0x3C => { let r = self.regs.a; self.regs.a = self.inc_u8(r); 1 },
			0x04 => { let r = self.regs.b; self.regs.b = self.inc_u8(r); 1 },
			0x0C => { let r = self.regs.c; self.regs.c = self.inc_u8(r); 1 },
			0x14 => { let r = self.regs.d; self.regs.d = self.inc_u8(r); 1 },
			0x1C => { let r = self.regs.e; self.regs.e = self.inc_u8(r); 1 },
			0x24 => { let r = self.regs.h; self.regs.h = self.inc_u8(r); 1 },
			0x2C => { let r = self.regs.l; self.regs.l = self.inc_u8(r); 1 },
			// INC (HL)
			0x34 => { let n = memory.read(self.regs.hl()); memory.write(self.regs.hl(), self.inc_u8(n)); 3 },
			// DEC r
			0x3D => { let r = self.regs.a; self.regs.a = self.dec_u8(r); 1 },
			0x05 => { let r = self.regs.b; self.regs.b = self.dec_u8(r); 1 },
			0x0D => { let r = self.regs.c; self.regs.c = self.dec_u8(r); 1 },
			0x15 => { let r = self.regs.d; self.regs.d = self.dec_u8(r); 1 },
			0x1D => { let r = self.regs.e; self.regs.e = self.dec_u8(r); 1 },
			0x25 => { let r = self.regs.h; self.regs.h = self.dec_u8(r); 1 },
			0x2D => { let r = self.regs.l; self.regs.l = self.dec_u8(r); 1 },
			// DEC (HL)
			0x35 => { let n = memory.read(self.regs.hl()); memory.write(self.regs.hl(), self.dec_u8(n)); 3 },
			// ADD HL, rr
			0x09 => { let rr = self.regs.bc(); self.add_hl(rr); 2 },
			0x19 => { let rr = self.regs.de(); self.add_hl(rr); 2 },
			0x29 => { let rr = self.regs.hl(); self.add_hl(rr); 2 },
			0x39 => { let rr = self.regs.sp;   self.add_hl(rr); 2 },
			// ADD SP, e
			0xE8 => { unimplemented!(); },
			// INC ss (no flags changed here)
			0x03 => { let rr = self.regs.bc().wrapping_add(1); self.regs.set_bc(rr); 2 },
			0x13 => { let rr = self.regs.de().wrapping_add(1); self.regs.set_de(rr); 2 },
			0x23 => { let rr = self.regs.hl().wrapping_add(1); self.regs.set_hl(rr); 2 },
			0x33 => { let rr = self.regs.sp.wrapping_add(1);   self.regs.sp = rr;    2 },
			// DEC ss (no flags changed here)
			0x0B => { let rr = self.regs.bc().wrapping_sub(1); self.regs.set_bc(rr); 2 },
			0x1B => { let rr = self.regs.de().wrapping_sub(1); self.regs.set_de(rr); 2 },
			0x2B => { let rr = self.regs.hl().wrapping_sub(1); self.regs.set_hl(rr); 2 },
			0x3B => { let rr = self.regs.sp.wrapping_sub(1);   self.regs.sp = rr; 2 },

			// JP nn
			0xC3 => { let nn = self.next_pointer(memory); self.jump(nn); 4 },
			// JP cc, nn
			0xC2 => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::NotZero) },
			0xCA => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::Zero) },
			0xD2 => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::NotCarry) },
			0xDA => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::Carry) },

			// JR e
			0x18 => { let e = self.next_byte(memory) as i8; self.jump_rel(e); 3 },
			// JR cc, e
			0x20 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::NotZero) },
			0x28 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::Zero) },
			0x30 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::NotCarry) },
			0x38 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::Carry) },
			// JP (HL)
			0xE9 => { let hl = self.regs.hl(); self.jump(hl); 1 },
			// CALL
			0xCD => { let nn = self.next_pointer(memory); self.call(memory, nn); 6 },
			// CALL cc
			0xC4 => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::NotZero) },
			0xCC => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::Zero) },
			0xD4 => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::NotCarry) },
			0xDC => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::Carry) },
			// RET
			0xC9 => { self.ret(memory); 4 },
			// RET cc
			0xC0 => { self.ret_if(memory, Condition::NotZero) },
			0xC8 => { self.ret_if(memory, Condition::Zero) },
			0xD0 => { self.ret_if(memory, Condition::NotCarry) },
			0xD8 => { self.ret_if(memory, Condition::Carry) },
			// RETI (return from interrupt)
			0xD9 => { memory.interrupt.enable(); self.ret(memory); 4 },
			// RST t
			0xC7 => { self.rst(memory, RstVector::Rst1); 4 },
			0xCF => { self.rst(memory, RstVector::Rst2); 4 },
			0xD7 => { self.rst(memory, RstVector::Rst3); 4 },
			0xDF => { self.rst(memory, RstVector::Rst4); 4 },
			0xE7 => { self.rst(memory, RstVector::Rst5); 4 },
			0xEF => { self.rst(memory, RstVector::Rst6); 4 },
			0xF7 => { self.rst(memory, RstVector::Rst7); 4 },
			0xFF => { self.rst(memory, RstVector::Rst8); 4 },
			// Rotates
			0x07 => { let a = self.regs.a; self.regs.a = self.rotate_left(a, false, false); 1 },
			0x17 => { let a = self.regs.a; self.regs.a = self.rotate_left(a, true, false); 1 },
			0x0F => { let a = self.regs.a; self.regs.a = self.rotate_right(a, false, false); 1 },
			0x1F => { let a = self.regs.a; self.regs.a = self.rotate_right(a, true, false); 1 },
			// Sub-Operations
			0xCB => { self.sub_op(memory) },
			// Decimal Adjust Accumulator
			0x27 => { self.daa(); 1 },
			// CPL
			0x2F => { self.cpl(); 1 },
			// scf
			0x37 => { self.scf(); 1 },
			// ccf
			0x3F => { self.ccf(); 1 },
			// HALT
			0x76 => { self.halted = true; 1 },
			// NOP
			0x00 => { 1 }, // easiest opcode of my life

			// GBCPUMAN
			0xF3 => { memory.interrupt.disable(); 1 }, // Disable interrupts
			0xFB => { memory.interrupt.enable();  1 }, // Enable interrupts

			0xC3 => { self.regs.pc = self.next_pointer(memory); 4 },
			_ => panic!("Unknown Opcode: ${:02X} @ ${:04X} dec: {}", opcode, old_pc, opcode)
		}
	}

	// Evaluates all sub-operations under CPU instr 0xCB
	// Returns number of cycles of operation
	fn sub_op(&mut self, memory: &mut Interconnect) -> usize {
		let pc = self.regs.pc;
		let opcode = self.next_byte(memory);
		let regs = self.regs;

		match opcode {
			// Rotate Left
			0x07 => { self.regs.a = self.rotate_left(regs.a, false, true); 2 },
			0x00 => { self.regs.b = self.rotate_left(regs.b, false, true); 2 },
			0x01 => { self.regs.c = self.rotate_left(regs.c, false, true); 2 },
			0x02 => { self.regs.d = self.rotate_left(regs.d, false, true); 2 },
			0x03 => { self.regs.e = self.rotate_left(regs.e, false, true); 2 },
			0x04 => { self.regs.h = self.rotate_left(regs.h, false, true); 2 },
			0x05 => { self.regs.l = self.rotate_left(regs.l, false, true); 2 },
			0x06 => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.rotate_left(n, false, false));
			4 },
			0x17 => { self.regs.a = self.rotate_left(regs.a, true, true); 2 },
			0x10 => { self.regs.b = self.rotate_left(regs.b, true, true); 2 },
			0x11 => { self.regs.c = self.rotate_left(regs.c, true, true); 2 },
			0x12 => { self.regs.d = self.rotate_left(regs.d, true, true); 2 },
			0x13 => { self.regs.e = self.rotate_left(regs.e, true, true); 2 },
			0x14 => { self.regs.h = self.rotate_left(regs.h, true, true); 2 },
			0x15 => { self.regs.l = self.rotate_left(regs.l, true, true); 2 },
			0x16 => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.rotate_left(n, true, false));
			4 },
			// Rotate Right
			0x0F => { self.regs.a = self.rotate_right(regs.a, false, true); 2 },
			0x08 => { self.regs.b = self.rotate_right(regs.b, false, true); 2 },
			0x09 => { self.regs.c = self.rotate_right(regs.c, false, true); 2 },
			0x0A => { self.regs.d = self.rotate_right(regs.d, false, true); 2 },
			0x0B => { self.regs.e = self.rotate_right(regs.e, false, true); 2 },
			0x0C => { self.regs.h = self.rotate_right(regs.h, false, true); 2 },
			0x0D => { self.regs.l = self.rotate_right(regs.l, false, true); 2 },
			0x0E => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.rotate_right(n, false, false));
			4 },
			0x1F => { self.regs.a = self.rotate_right(regs.a, true, true); 2 },
			0x18 => { self.regs.b = self.rotate_right(regs.b, true, true); 2 },
			0x19 => { self.regs.c = self.rotate_right(regs.c, true, true); 2 },
			0x1A => { self.regs.d = self.rotate_right(regs.d, true, true); 2 },
			0x1B => { self.regs.e = self.rotate_right(regs.e, true, true); 2 },
			0x1C => { self.regs.h = self.rotate_right(regs.h, true, true); 2 },
			0x1D => { self.regs.l = self.rotate_right(regs.l, true, true); 2 },
			0x1E => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.rotate_right(n, true, false));
			4 },
			// Shift Left
			0x27 => { self.regs.a = self.shift_left(regs.a); 2 },
			0x20 => { self.regs.b = self.shift_left(regs.b); 2 },
			0x21 => { self.regs.c = self.shift_left(regs.c); 2 },
			0x22 => { self.regs.d = self.shift_left(regs.d); 2 },
			0x23 => { self.regs.e = self.shift_left(regs.e); 2 },
			0x24 => { self.regs.h = self.shift_left(regs.h); 2 },
			0x25 => { self.regs.l = self.shift_left(regs.l); 2 },
			0x26 => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.shift_left(n));
			4 },
			// Shift Right
			0x2F => { self.regs.a = self.shift_right(regs.a, true); 2 },
			0x28 => { self.regs.b = self.shift_right(regs.b, true); 2 },
			0x29 => { self.regs.c = self.shift_right(regs.c, true); 2 },
			0x2A => { self.regs.d = self.shift_right(regs.d, true); 2 },
			0x2B => { self.regs.e = self.shift_right(regs.e, true); 2 },
			0x2C => { self.regs.h = self.shift_right(regs.h, true); 2 },
			0x2D => { self.regs.l = self.shift_right(regs.l, true); 2 },
			0x2E => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.shift_right(n, true));
			4 },
			0x3F => { self.regs.a = self.shift_right(regs.a, false); 2 },
			0x38 => { self.regs.b = self.shift_right(regs.b, false); 2 },
			0x39 => { self.regs.c = self.shift_right(regs.c, false); 2 },
			0x3A => { self.regs.d = self.shift_right(regs.d, false); 2 },
			0x3B => { self.regs.e = self.shift_right(regs.e, false); 2 },
			0x3C => { self.regs.h = self.shift_right(regs.h, false); 2 },
			0x3D => { self.regs.l = self.shift_right(regs.l, false); 2 },
			0x3E => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.shift_right(n, false));
			4 },
			// Swap
			0x37 => { self.regs.a = self.swap(regs.a); 2 },
			0x30 => { self.regs.b = self.swap(regs.b); 2 },
			0x31 => { self.regs.c = self.swap(regs.c); 2 },
			0x32 => { self.regs.d = self.swap(regs.d); 2 },
			0x33 => { self.regs.e = self.swap(regs.e); 2 },
			0x34 => { self.regs.h = self.swap(regs.h); 2 },
			0x35 => { self.regs.l = self.swap(regs.l); 2 },
			0x36 => {
				let n = memory.read(regs.hl());
				memory.write(regs.hl(), self.swap(n));
			4 },
			// Bit
			0x47 => { self.bit(regs.a, Bit::Bit0); 2 },
			0x4F => { self.bit(regs.a, Bit::Bit1); 2 },
			0x57 => { self.bit(regs.a, Bit::Bit2); 2 },
			0x5F => { self.bit(regs.a, Bit::Bit3); 2 },
			0x67 => { self.bit(regs.a, Bit::Bit4); 2 },
			0x6F => { self.bit(regs.a, Bit::Bit5); 2 },
			0x77 => { self.bit(regs.a, Bit::Bit6); 2 },
			0x7F => { self.bit(regs.a, Bit::Bit7); 2 },
			0x40 => { self.bit(regs.b, Bit::Bit0); 2 },
			0x48 => { self.bit(regs.b, Bit::Bit1); 2 },
			0x50 => { self.bit(regs.b, Bit::Bit2); 2 },
			0x58 => { self.bit(regs.b, Bit::Bit3); 2 },
			0x60 => { self.bit(regs.b, Bit::Bit4); 2 },
			0x68 => { self.bit(regs.b, Bit::Bit5); 2 },
			0x70 => { self.bit(regs.b, Bit::Bit6); 2 },
			0x78 => { self.bit(regs.b, Bit::Bit7); 2 },
			0x41 => { self.bit(regs.c, Bit::Bit0); 2 },
			0x49 => { self.bit(regs.c, Bit::Bit1); 2 },
			0x51 => { self.bit(regs.c, Bit::Bit2); 2 },
			0x59 => { self.bit(regs.c, Bit::Bit3); 2 },
			0x61 => { self.bit(regs.c, Bit::Bit4); 2 },
			0x69 => { self.bit(regs.c, Bit::Bit5); 2 },
			0x71 => { self.bit(regs.c, Bit::Bit6); 2 },
			0x79 => { self.bit(regs.c, Bit::Bit7); 2 },
			0x42 => { self.bit(regs.d, Bit::Bit0); 2 },
			0x4A => { self.bit(regs.d, Bit::Bit1); 2 },
			0x52 => { self.bit(regs.d, Bit::Bit2); 2 },
			0x5A => { self.bit(regs.d, Bit::Bit3); 2 },
			0x62 => { self.bit(regs.d, Bit::Bit4); 2 },
			0x6A => { self.bit(regs.d, Bit::Bit5); 2 },
			0x72 => { self.bit(regs.d, Bit::Bit6); 2 },
			0x7A => { self.bit(regs.d, Bit::Bit7); 2 },
			0x43 => { self.bit(regs.e, Bit::Bit0); 2 },
			0x4B => { self.bit(regs.e, Bit::Bit1); 2 },
			0x53 => { self.bit(regs.e, Bit::Bit2); 2 },
			0x5B => { self.bit(regs.e, Bit::Bit3); 2 },
			0x63 => { self.bit(regs.e, Bit::Bit4); 2 },
			0x6B => { self.bit(regs.e, Bit::Bit5); 2 },
			0x73 => { self.bit(regs.e, Bit::Bit6); 2 },
			0x7B => { self.bit(regs.e, Bit::Bit7); 2 },
			0x44 => { self.bit(regs.h, Bit::Bit0); 2 },
			0x4C => { self.bit(regs.h, Bit::Bit1); 2 },
			0x54 => { self.bit(regs.h, Bit::Bit2); 2 },
			0x5C => { self.bit(regs.h, Bit::Bit3); 2 },
			0x64 => { self.bit(regs.h, Bit::Bit4); 2 },
			0x6C => { self.bit(regs.h, Bit::Bit5); 2 },
			0x74 => { self.bit(regs.h, Bit::Bit6); 2 },
			0x7C => { self.bit(regs.h, Bit::Bit7); 2 },
			0x45 => { self.bit(regs.l, Bit::Bit0); 2 },
			0x4D => { self.bit(regs.l, Bit::Bit1); 2 },
			0x55 => { self.bit(regs.l, Bit::Bit2); 2 },
			0x5D => { self.bit(regs.l, Bit::Bit3); 2 },
			0x65 => { self.bit(regs.l, Bit::Bit4); 2 },
			0x6D => { self.bit(regs.l, Bit::Bit5); 2 },
			0x75 => { self.bit(regs.l, Bit::Bit6); 2 },
			0x7D => { self.bit(regs.l, Bit::Bit7); 2 },
			0x46 => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit0); 3 },
			0x4E => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit1); 3 },
			0x56 => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit2); 3 },
			0x5E => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit3); 3 },
			0x66 => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit4); 3 },
			0x6E => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit5); 3 },
			0x76 => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit6); 3 },
			0x7E => { let n = memory.read(regs.hl()); self.bit(n, Bit::Bit7); 3 },
			// Set
			0xC7 => { self.regs.a = self.set(regs.a, Bit::Bit0); 2 },
			0xCF => { self.regs.a = self.set(regs.a, Bit::Bit1); 2 },
			0xD7 => { self.regs.a = self.set(regs.a, Bit::Bit2); 2 },
			0xDF => { self.regs.a = self.set(regs.a, Bit::Bit3); 2 },
			0xE7 => { self.regs.a = self.set(regs.a, Bit::Bit4); 2 },
			0xEF => { self.regs.a = self.set(regs.a, Bit::Bit5); 2 },
			0xF7 => { self.regs.a = self.set(regs.a, Bit::Bit6); 2 },
			0xFF => { self.regs.a = self.set(regs.a, Bit::Bit7); 2 },
			0xC0 => { self.regs.b = self.set(regs.b, Bit::Bit0); 2 },
			0xC8 => { self.regs.b = self.set(regs.b, Bit::Bit1); 2 },
			0xD0 => { self.regs.b = self.set(regs.b, Bit::Bit2); 2 },
			0xD8 => { self.regs.b = self.set(regs.b, Bit::Bit3); 2 },
			0xE0 => { self.regs.b = self.set(regs.b, Bit::Bit4); 2 },
			0xE8 => { self.regs.b = self.set(regs.b, Bit::Bit5); 2 },
			0xF0 => { self.regs.b = self.set(regs.b, Bit::Bit6); 2 },
			0xF8 => { self.regs.b = self.set(regs.b, Bit::Bit7); 2 },
			0xC1 => { self.regs.c = self.set(regs.c, Bit::Bit0); 2 },
			0xC9 => { self.regs.c = self.set(regs.c, Bit::Bit1); 2 },
			0xD1 => { self.regs.c = self.set(regs.c, Bit::Bit2); 2 },
			0xD9 => { self.regs.c = self.set(regs.c, Bit::Bit3); 2 },
			0xE1 => { self.regs.c = self.set(regs.c, Bit::Bit4); 2 },
			0xE9 => { self.regs.c = self.set(regs.c, Bit::Bit5); 2 },
			0xF1 => { self.regs.c = self.set(regs.c, Bit::Bit6); 2 },
			0xF9 => { self.regs.c = self.set(regs.c, Bit::Bit7); 2 },
			0xC2 => { self.regs.d = self.set(regs.d, Bit::Bit0); 2 },
			0xCA => { self.regs.d = self.set(regs.d, Bit::Bit1); 2 },
			0xD2 => { self.regs.d = self.set(regs.d, Bit::Bit2); 2 },
			0xDA => { self.regs.d = self.set(regs.d, Bit::Bit3); 2 },
			0xE2 => { self.regs.d = self.set(regs.d, Bit::Bit4); 2 },
			0xEA => { self.regs.d = self.set(regs.d, Bit::Bit5); 2 },
			0xF2 => { self.regs.d = self.set(regs.d, Bit::Bit6); 2 },
			0xFA => { self.regs.d = self.set(regs.d, Bit::Bit7); 2 },
			0xC3 => { self.regs.e = self.set(regs.e, Bit::Bit0); 2 },
			0xCB => { self.regs.e = self.set(regs.e, Bit::Bit1); 2 },
			0xD3 => { self.regs.e = self.set(regs.e, Bit::Bit2); 2 },
			0xDB => { self.regs.e = self.set(regs.e, Bit::Bit3); 2 },
			0xE3 => { self.regs.e = self.set(regs.e, Bit::Bit4); 2 },
			0xEB => { self.regs.e = self.set(regs.e, Bit::Bit5); 2 },
			0xF3 => { self.regs.e = self.set(regs.e, Bit::Bit6); 2 },
			0xFB => { self.regs.e = self.set(regs.e, Bit::Bit7); 2 },
			0xC4 => { self.regs.h = self.set(regs.h, Bit::Bit0); 2 },
			0xCC => { self.regs.h = self.set(regs.h, Bit::Bit1); 2 },
			0xD4 => { self.regs.h = self.set(regs.h, Bit::Bit2); 2 },
			0xDC => { self.regs.h = self.set(regs.h, Bit::Bit3); 2 },
			0xE4 => { self.regs.h = self.set(regs.h, Bit::Bit4); 2 },
			0xEC => { self.regs.h = self.set(regs.h, Bit::Bit5); 2 },
			0xF4 => { self.regs.h = self.set(regs.h, Bit::Bit6); 2 },
			0xFC => { self.regs.h = self.set(regs.h, Bit::Bit7); 2 },
			0xC5 => { self.regs.l = self.set(regs.l, Bit::Bit0); 2 },
			0xCD => { self.regs.l = self.set(regs.l, Bit::Bit1); 2 },
			0xD5 => { self.regs.l = self.set(regs.l, Bit::Bit2); 2 },
			0xDD => { self.regs.l = self.set(regs.l, Bit::Bit3); 2 },
			0xE5 => { self.regs.l = self.set(regs.l, Bit::Bit4); 2 },
			0xED => { self.regs.l = self.set(regs.l, Bit::Bit5); 2 },
			0xF5 => { self.regs.l = self.set(regs.l, Bit::Bit6); 2 },
			0xFD => { self.regs.l = self.set(regs.l, Bit::Bit7); 2 },
			0xC6 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit0)); 4 },
			0xCE => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit1)); 4 },
			0xD6 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit2)); 4 },
			0xDE => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit3)); 4 },
			0xE6 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit4)); 4 },
			0xEE => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit5)); 4 },
			0xF6 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit6)); 4 },
			0xFE => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.set(n, Bit::Bit7)); 4 },
			// Reset
			0x87 => { self.regs.a = self.res(regs.a, Bit::Bit0); 2 },
			0x8F => { self.regs.a = self.res(regs.a, Bit::Bit1); 2 },
			0x97 => { self.regs.a = self.res(regs.a, Bit::Bit2); 2 },
			0x9F => { self.regs.a = self.res(regs.a, Bit::Bit3); 2 },
			0xA7 => { self.regs.a = self.res(regs.a, Bit::Bit4); 2 },
			0xAF => { self.regs.a = self.res(regs.a, Bit::Bit5); 2 },
			0xB7 => { self.regs.a = self.res(regs.a, Bit::Bit6); 2 },
			0xBF => { self.regs.a = self.res(regs.a, Bit::Bit7); 2 },
			0x80 => { self.regs.b = self.res(regs.b, Bit::Bit0); 2 },
			0x88 => { self.regs.b = self.res(regs.b, Bit::Bit1); 2 },
			0x90 => { self.regs.b = self.res(regs.b, Bit::Bit2); 2 },
			0x98 => { self.regs.b = self.res(regs.b, Bit::Bit3); 2 },
			0xA0 => { self.regs.b = self.res(regs.b, Bit::Bit4); 2 },
			0xA8 => { self.regs.b = self.res(regs.b, Bit::Bit5); 2 },
			0xB0 => { self.regs.b = self.res(regs.b, Bit::Bit6); 2 },
			0xB8 => { self.regs.b = self.res(regs.b, Bit::Bit7); 2 },
			0x81 => { self.regs.c = self.res(regs.c, Bit::Bit0); 2 },
			0x89 => { self.regs.c = self.res(regs.c, Bit::Bit1); 2 },
			0x91 => { self.regs.c = self.res(regs.c, Bit::Bit2); 2 },
			0x99 => { self.regs.c = self.res(regs.c, Bit::Bit3); 2 },
			0xA1 => { self.regs.c = self.res(regs.c, Bit::Bit4); 2 },
			0xA9 => { self.regs.c = self.res(regs.c, Bit::Bit5); 2 },
			0xB1 => { self.regs.c = self.res(regs.c, Bit::Bit6); 2 },
			0xB9 => { self.regs.c = self.res(regs.c, Bit::Bit7); 2 },
			0x82 => { self.regs.d = self.res(regs.d, Bit::Bit0); 2 },
			0x8A => { self.regs.d = self.res(regs.d, Bit::Bit1); 2 },
			0x92 => { self.regs.d = self.res(regs.d, Bit::Bit2); 2 },
			0x9A => { self.regs.d = self.res(regs.d, Bit::Bit3); 2 },
			0xA2 => { self.regs.d = self.res(regs.d, Bit::Bit4); 2 },
			0xAA => { self.regs.d = self.res(regs.d, Bit::Bit5); 2 },
			0xB2 => { self.regs.d = self.res(regs.d, Bit::Bit6); 2 },
			0xBA => { self.regs.d = self.res(regs.d, Bit::Bit7); 2 },
			0x83 => { self.regs.e = self.res(regs.e, Bit::Bit0); 2 },
			0x8B => { self.regs.e = self.res(regs.e, Bit::Bit1); 2 },
			0x93 => { self.regs.e = self.res(regs.e, Bit::Bit2); 2 },
			0x9B => { self.regs.e = self.res(regs.e, Bit::Bit3); 2 },
			0xA3 => { self.regs.e = self.res(regs.e, Bit::Bit4); 2 },
			0xAB => { self.regs.e = self.res(regs.e, Bit::Bit5); 2 },
			0xB3 => { self.regs.e = self.res(regs.e, Bit::Bit6); 2 },
			0xBB => { self.regs.e = self.res(regs.e, Bit::Bit7); 2 },
			0x84 => { self.regs.h = self.res(regs.h, Bit::Bit0); 2 },
			0x8C => { self.regs.h = self.res(regs.h, Bit::Bit1); 2 },
			0x94 => { self.regs.h = self.res(regs.h, Bit::Bit2); 2 },
			0x9C => { self.regs.h = self.res(regs.h, Bit::Bit3); 2 },
			0xA4 => { self.regs.h = self.res(regs.h, Bit::Bit4); 2 },
			0xAC => { self.regs.h = self.res(regs.h, Bit::Bit5); 2 },
			0xB4 => { self.regs.h = self.res(regs.h, Bit::Bit6); 2 },
			0xBC => { self.regs.h = self.res(regs.h, Bit::Bit7); 2 },
			0x85 => { self.regs.l = self.res(regs.l, Bit::Bit0); 2 },
			0x8D => { self.regs.l = self.res(regs.l, Bit::Bit1); 2 },
			0x95 => { self.regs.l = self.res(regs.l, Bit::Bit2); 2 },
			0x9D => { self.regs.l = self.res(regs.l, Bit::Bit3); 2 },
			0xA5 => { self.regs.l = self.res(regs.l, Bit::Bit4); 2 },
			0xAD => { self.regs.l = self.res(regs.l, Bit::Bit5); 2 },
			0xB5 => { self.regs.l = self.res(regs.l, Bit::Bit6); 2 },
			0xBD => { self.regs.l = self.res(regs.l, Bit::Bit7); 2 },
			0x86 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit0)); 4 },
			0x8E => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit1)); 4 },
			0x96 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit2)); 4 },
			0x9E => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit3)); 4 },
			0xA6 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit4)); 4 },
			0xAE => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit5)); 4 },
			0xB6 => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit6)); 4 },
			0xBE => { let n = memory.read(regs.hl()); memory.write(regs.hl(), self.res(n, Bit::Bit7)); 4 },
			_ => panic!("Unknown Sub-Op: ${:02X} @ ${:04X} dec: {}", opcode, pc, opcode)
		}
	}

		// Pushes 16 bit data onto the stack
	fn push(&mut self, memory: &mut Interconnect, data: u16) {
		self.regs.sp = self.regs.sp.wrapping_sub(1);
		memory.write(self.regs.sp, high!(data));
		self.regs.sp = self.regs.sp.wrapping_sub(1);
		memory.write(self.regs.sp, low!(data));
	}

	// Pops highest 16 bits from stack
	fn pop(&mut self, memory: &mut Interconnect) -> u16 {
		let low = memory.read(self.regs.sp);
		self.regs.sp = self.regs.sp.wrapping_add(1);
		let high = memory.read(self.regs.sp);
		self.regs.sp = self.regs.sp.wrapping_add(1);
		combine!(high, low)
	}

	/* 8-bit operations */

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

	fn and_u8(&mut self, n: u8) {
		let result = self.regs.a & n;
		self.regs.set_flag(Flag::Carry, false);
		self.regs.set_flag(Flag::HalfCarry, true);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0));
		self.regs.a = result;
	}

	fn or_u8(&mut self, n: u8) {
		let result = self.regs.a | n;
		self.regs.set_flag(Flag::Carry, false);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0));
		self.regs.a = result;
	}

	fn xor_u8(&mut self, n: u8) {
		let result = self.regs.a ^ n;
		self.regs.set_flag(Flag::Carry, false);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0));
		self.regs.a = result;
	}

	// Compare A with n. This is basically a A - n subtraction but the results are thrown away
	fn cp_u8(&mut self, n: u8) {
		let a = self.regs.a;
		self.sub_u8(n, false);
		self.regs.a = a;
	}

	fn inc_u8(&mut self, n: u8) -> u8 {
		let result = n.wrapping_add(1);
		let half_carry = (n & 0xF) + 1 > 0xF;
		self.regs.set_flag(Flag::HalfCarry, half_carry);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0));
		result
	}

	fn dec_u8(&mut self, n: u8) -> u8 {
		let result = n.wrapping_sub(1);
		let half_carry = ((n & 0xF) as i16) - 1 < 0;
		self.regs.set_flag(Flag::HalfCarry, half_carry);
		self.regs.set_flag(Flag::Sub, true);
		self.regs.set_flag(Flag::Zero, (result == 0));
		result
	}

	fn cpl(&mut self) {
		self.regs.a = !self.regs.a;
		self.regs.set_flag(Flag::HalfCarry, true);
		self.regs.set_flag(Flag::Sub, true);
	}

	/* 8-bit Rotate/Shift operations */

	fn rotate_left(&mut self, n: u8, include_carry: bool, update_zero: bool) -> u8 {
		let bit7 = n >> 7;
		let result = match include_carry {
			true =>  {
				self.regs.set_flag(Flag::Carry, (bit7 == 1));
				(n << 1) | (self.regs.is_flag_set(Flag::Carry) as u8)
			},
			false => n.rotate_left(1),
		};
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0 && update_zero));
		result
	}

	fn rotate_right(&mut self, n: u8, include_carry: bool, update_zero: bool) -> u8 {
		let bit1 = n & 1;
		let result = match include_carry {
			true =>  {
				self.regs.set_flag(Flag::Carry, (bit1 == 1));
				(n >> 1) | ((self.regs.is_flag_set(Flag::Carry) as u8) << 7)
			},
			false => n.rotate_right(1),
		};
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, (result == 0 && update_zero));
		result
	}

	fn shift_left(&mut self, n: u8) -> u8 {
		let result = n << 1;
		let bit7 = n >> 7;
		self.regs.set_flag(Flag::Carry, bit7 == 1);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, result == 0);
		result
	}

	fn shift_right(&mut self, n: u8, keep_bit7: bool) -> u8 {
		let result = match keep_bit7 {
			true => (n >> 1) | (n & 0x80),
			false => n >> 1,
		};
		self.regs.set_flag(Flag::Carry, (n & 1) == 1);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, result == 0);
		result
	}

	fn swap(&mut self, n: u8) -> u8 {
		let high = n >> 4;
		let low  = n << 4;
		let result = low | high;
		self.regs.set_flag(Flag::Carry, false);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::Zero, result == 0);
		result
	}

	fn bit(&mut self, n: u8, bit: Bit) {
		self.regs.set_flag(Flag::Zero, ((bit as u8) & !n) != 0);
		self.regs.set_flag(Flag::HalfCarry, true);
		self.regs.set_flag(Flag::Sub, false);
	}

	fn set(&mut self, n: u8, bit: Bit) -> u8 {
		n | bit as u8
	}

	fn res(&mut self, n: u8, bit: Bit) -> u8 {
		(!(bit as u8) & n)
	}

	fn daa(&mut self) {
		let mut carry = false;
		if !self.regs.is_flag_set(Flag::Sub) {
			if self.regs.is_flag_set(Flag::Carry) || self.regs.a > 0x99 {
				self.regs.a = self.regs.a.wrapping_add(0x60);
				carry = true;
			}
			if self.regs.is_flag_set(Flag::HalfCarry) || self.regs.a & 0x0F > 0x09 {
				self.regs.a = self.regs.a.wrapping_add(0x06);
			}
		} else if self.regs.is_flag_set(Flag::Carry) {
			carry = true;
			self.regs.a = self.regs.a.wrapping_add(
				match self.regs.is_flag_set(Flag::HalfCarry) {
					true =>  0x9A,
					false => 0xA0,
				}
			);
		} else if self.regs.is_flag_set(Flag::HalfCarry) {
			self.regs.a = self.regs.a.wrapping_add(0xFA);
		}
		let a = self.regs.a;
		let sub = self.regs.is_flag_set(Flag::Sub);
		self.regs.set_flag(Flag::Zero, (a == 0));
		self.regs.set_flag(Flag::Sub, sub);
		self.regs.set_flag(Flag::Carry, carry);
	}

	/* 16-bit operations */

	fn add_hl(&mut self, n: u16) {
		let hl = self.regs.hl();
		let result = hl.wrapping_add(n);
		//((((target & 0xFFF) + (value & 0xFFF)) & 0x1000) != 0))
		let half_carry = (((hl & 0xFFF) + (n & 0xFFF)) & 0x1000) != 0;
		self.regs.set_flag(Flag::Carry, hl > 0xFFFF - n);
		self.regs.set_flag(Flag::HalfCarry, half_carry);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_hl(result);
	}

	fn ld_hl(&mut self, n: u8) {
		let signed_val = (n as i8) as i16;
		let sp = self.regs.sp;
		let result = ((sp as i16) + signed_val) as u16;
		self.regs.set_flag(Flag::Carry, (result & 0xFF) < (sp & 0xFF));
		self.regs.set_flag(Flag::HalfCarry, (result & 0xF) < (sp & 0xF));
		self.regs.set_flag(Flag::Zero, false);
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_hl(result);
	}

	/* Jump Instructions */

	fn jump(&mut self, source: u16) {
		self.regs.pc = source;
	}

	// Jumps relative to a signed i8
	fn jump_rel(&mut self, source: i8) {
		let result = (self.regs.pc as i16).wrapping_add(source as i16);
		self.jump(result as u16);
	}

	// Determines whether or not to jump based on passed argument
	// returns number of cycles (varied)
	fn jump_if(&mut self, source: u16, condition: Condition) -> usize {
		match condition {
			Condition::NotZero => { // JP NZ
				if !self.regs.is_flag_set(Flag::Zero) { self.jump(source); 4 } else { 3 }
			},
			Condition::Zero => { // JP Z
				if self.regs.is_flag_set(Flag::Zero) { self.jump(source); 4 } else { 3 }
			},
			Condition::NotCarry => { // JP NC
				if !self.regs.is_flag_set(Flag::Carry) { self.jump(source); 4 } else { 3 }
			},
			Condition::Carry => { // JP C
				if self.regs.is_flag_set(Flag::Carry) { self.jump(source); 4 } else { 3 }
			}, _ => unreachable!()
		}
	}

	fn jump_rel_if(&mut self, source: i8, condition: Condition) -> usize {
		let result = (self.regs.pc as i16).wrapping_add(source as i16);
		self.jump_if(result as u16, condition) - 1
	}

	/* Call and Return Instructions */
	fn call(&mut self, memory: &mut Interconnect, source: u16) {
		let pc = self.regs.pc;
		self.push(memory, pc);
		self.regs.pc = source;
	}

	// Calls based on condition. Returns cycles (varied)
	fn call_if(&mut self, memory: &mut Interconnect, source: u16, condition: Condition) -> usize {
		match condition {
			Condition::NotZero => { // CALL NZ
				if !self.regs.is_flag_set(Flag::Zero) { self.call(memory, source); 6 } else { 3 }
			},
			Condition::Zero => { // CALL Z
				if self.regs.is_flag_set(Flag::Zero) { self.call(memory, source); 6 } else { 3 }
			},
			Condition::NotCarry => { // CALL NC
				if !self.regs.is_flag_set(Flag::Carry) { self.call(memory, source); 6 } else { 3 }
			},
			Condition::Carry => { // CALL C
				if self.regs.is_flag_set(Flag::Carry) { self.call(memory, source); 6 } else { 3 }
			}, _ => unreachable!()
		}
	}

	fn ret(&mut self, memory: &mut Interconnect) {
		let pc = self.pop(memory);
		self.regs.pc = pc;
	}

	// Returns based on condition. Returns cycles (varied)
	fn ret_if(&mut self, memory: &mut Interconnect, condition: Condition) -> usize {
		match condition {
			Condition::NotZero => { // CALL NZ
				if !self.regs.is_flag_set(Flag::Zero) { self.ret(memory); 5 } else { 2 }
			},
			Condition::Zero => { // CALL Z
				if self.regs.is_flag_set(Flag::Zero) { self.ret(memory); 5 } else { 2 }
			},
			Condition::NotCarry => { // CALL NC
				if !self.regs.is_flag_set(Flag::Carry) { self.ret(memory); 5 } else { 2 }
			},
			Condition::Carry => { // CALL C
				if self.regs.is_flag_set(Flag::Carry) { self.ret(memory); 5 } else { 2 }
			}, _ => unreachable!()
		}
	}

	fn rst(&mut self, memory: &mut Interconnect, vector: RstVector) {
		let source = vector as u16;
		self.call(memory, source);
	}

	// scf 4 -001 cy=1
	fn scf(&mut self) {
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Carry, true);
	}
	// ccf 4 -00c cy=cy xor 1
	fn ccf(&mut self) {
		let bit = match self.regs.is_flag_set(Flag::Carry) {
			true => 1,
			false => 0,
		};
		self.regs.set_flag(Flag::Sub, false);
		self.regs.set_flag(Flag::HalfCarry, false);
		self.regs.set_flag(Flag::Carry, (bit ^ 1) == 1);
	}

	fn handle_interrupts(&mut self, memory: &mut Interconnect) {
		if let Some(vector) = memory.interrupt.execute_next() {
			self.halted = false;
			self.call(memory, vector as u16);
		}
	}

	pub fn debug(&mut self) {
		// self.regs.set_bc(0x8500);
		// self.regs.set_flag(Flag::Carry, false);
		println!("{}", self.regs);
	}
}
