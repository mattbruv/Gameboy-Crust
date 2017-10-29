use core::register::*;
use core::interconnect::*;

pub fn disassemble(reg: &Registers, mem: &Interconnect, opcode: u8) -> String {

	let n1 = mem.read(reg.pc); // opcode + 1
	let n2 = mem.read(reg.pc + 1); // opcode + 2
	let n3 = mem.read(reg.pc + 2); // opcode + 3
	let hl = mem.read(reg.hl());
	let bc = mem.read(reg.bc());
	let de = mem.read(reg.de());

	match opcode {
		0x7F => format!("LD A, A=${:02X}", reg.a),
		0x78 => format!("LD A, B=${:02X}", reg.b),
		0x79 => format!("LD A, C=${:02X}", reg.c),
		0x7A => format!("LD A, D=${:02X}", reg.d),
		0x7B => format!("LD A, E=${:02X}", reg.e),
		0x7C => format!("LD A, H=${:02X}", reg.h),
		0x7D => format!("LD A, L=${:02X}", reg.l),
		0x47 => format!("LD B, A=${:02X}", reg.a),
		0x40 => format!("LD B, B=${:02X}", reg.b),
		0x41 => format!("LD B, C=${:02X}", reg.c),
		0x42 => format!("LD B, D=${:02X}", reg.d),
		0x43 => format!("LD B, E=${:02X}", reg.e),
		0x44 => format!("LD B, H=${:02X}", reg.h),
		0x45 => format!("LD B, L=${:02X}", reg.l),
		0x4F => format!("LD C, A=${:02X}", reg.a),
		0x48 => format!("LD C, B=${:02X}", reg.b),
		0x49 => format!("LD C, C=${:02X}", reg.c),
		0x4A => format!("LD C, D=${:02X}", reg.d),
		0x4B => format!("LD C, E=${:02X}", reg.e),
		0x4C => format!("LD C, H=${:02X}", reg.h),
		0x4D => format!("LD C, L=${:02X}", reg.l),
		0x57 => format!("LD D, A=${:02X}", reg.a),
		0x50 => format!("LD D, B=${:02X}", reg.b),
		0x51 => format!("LD D, C=${:02X}", reg.c),
		0x52 => format!("LD D, D=${:02X}", reg.d),
		0x53 => format!("LD D, E=${:02X}", reg.e),
		0x54 => format!("LD D, H=${:02X}", reg.h),
		0x55 => format!("LD D, L=${:02X}", reg.l),
		0x5F => format!("LD E, A=${:02X}", reg.a),
		0x58 => format!("LD E, B=${:02X}", reg.b),
		0x59 => format!("LD E, C=${:02X}", reg.c),
		0x5A => format!("LD E, D=${:02X}", reg.d),
		0x5B => format!("LD E, E=${:02X}", reg.e),
		0x5C => format!("LD E, H=${:02X}", reg.h),
		0x5D => format!("LD E, L=${:02X}", reg.l),
		0x67 => format!("LD H, A=${:02X}", reg.a),
		0x60 => format!("LD H, B=${:02X}", reg.b),
		0x61 => format!("LD H, C=${:02X}", reg.c),
		0x62 => format!("LD H, D=${:02X}", reg.d),
		0x63 => format!("LD H, E=${:02X}", reg.e),
		0x64 => format!("LD H, H=${:02X}", reg.h),
		0x65 => format!("LD H, L=${:02X}", reg.l),
		0x6F => format!("LD L, A=${:02X}", reg.a),
		0x68 => format!("LD L, B=${:02X}", reg.b),
		0x69 => format!("LD L, C=${:02X}", reg.c),
		0x6A => format!("LD L, D=${:02X}", reg.d),
		0x6B => format!("LD L, E=${:02X}", reg.e),
		0x6D => format!("LD L, H=${:02X}", reg.h),
		0x6C => format!("LD L, L=${:02X}", reg.l),

		0x3E => format!("LD A, ${:02X}", n1),
		0x06 => format!("LD B, ${:02X}", n1),
		0x0E => format!("LD C, ${:02X}", n1),
		0x16 => format!("LD D, ${:02X}", n1),
		0x1E => format!("LD E, ${:02X}", n1),
		0x26 => format!("LD H, ${:02X}", n1),
		0x2E => format!("LD L, ${:02X}", n1),

		0x7E => format!("LD A, (HL)=${:02X}", hl),
		0x46 => format!("LD B, (HL)=${:02X}", hl),
		0x4E => format!("LD C, (HL)=${:02X}", hl),
		0x56 => format!("LD D, (HL)=${:02X}", hl),
		0x5E => format!("LD E, (HL)=${:02X}", hl),
		0x66 => format!("LD H, (HL)=${:02X}", hl),
		0x6E => format!("LD L, (HL)=${:02X}", hl),

		0x77 => format!("LD (HL), ${:02X}", reg.a),
		0x70 => format!("LD (HL), ${:02X}", reg.b),
		0x71 => format!("LD (HL), ${:02X}", reg.c),
		0x72 => format!("LD (HL), ${:02X}", reg.d),
		0x73 => format!("LD (HL), ${:02X}", reg.e),
		0x74 => format!("LD (HL), ${:02X}", reg.h),
		0x75 => format!("LD (HL), ${:02X}", reg.l),

		0x36 => format!("LD (HL), ${:02X}", n1),

		0x0A => format!("LD A, (BC)=${:02X}", bc),
		0x1A => format!("LD A, (DE)=${:02X}", de),
		0xF2 => format!("LD A, (${:04X})=${:02X}",
					mem.read((0xFF00 as u16).wrapping_add(reg.c as u16)), 
					reg.c),

//		// LD A, (C)
//		0xF2 => { self.regs.a = memory.read((0xFF00 as u16).wrapping_add(self.regs.c as u16)); 2 },
//		// LD (C), A
//		0xE2 => { memory.write((0xFF00 as u16).wrapping_add(self.regs.c as u16), self.regs.a); 2 }, 
//		// LD A, (n)
//		0xF0 => { self.regs.a = memory.read((0xFF00 as u16).wrapping_add(self.next_byte(memory) as u16)); 3 },
//		// LD (n), A
//		0xE0 => { let n = self.next_byte(memory); memory.write((0xFF00 as u16).wrapping_add(n as u16), self.regs.a); 3 },
//		// LD A, (nn)
//		0xFA => { self.regs.a = memory.read(self.next_pointer(memory)); 4 },
//		// LD (nn), A
//		0xEA => { let addr = self.next_pointer(memory); memory.write(addr, self.regs.a); 4 },
//		// LD A (HLI)			
//		0x2A => { self.regs.a = memory.read(self.regs.hl()); self.regs.hli(); 2 },
//		// LD A (HLD)
//		0x3A => { self.regs.a = memory.read(self.regs.hl()); self.regs.hld(); 2 },
//		// LD (BC), A
//		0x02 => { memory.write(self.regs.bc(), self.regs.a); 2 },
//		// LD (DE), A
//		0x12 => { memory.write(self.regs.de(), self.regs.a); 2 },
//		// LD (HLI), A
//		0x22 => { memory.write(self.regs.hl(), self.regs.a); self.regs.hli(); 2 },
//		// LD (HLD), A
//		0x32 => { memory.write(self.regs.hl(), self.regs.a); self.regs.hld(); 2 },
////		// 16-bit transfers
//		// LD dd, nn
//		0x01 => { let nn = self.next_pointer(memory); self.regs.set_bc(nn); 3 },
//		0x11 => { let nn = self.next_pointer(memory); self.regs.set_de(nn); 3 },
//		0x21 => { let nn = self.next_pointer(memory); self.regs.set_hl(nn); 3 },
//		0x31 => { let nn = self.next_pointer(memory); self.regs.sp = nn;    3 },
//		// LD SP, HL
//		0xF9 => { self.regs.sp = self.regs.hl(); 2 },
//		// PUSH qq
//		0xC5 => { let nn = self.regs.bc(); self.push(memory, nn); 4 },
//		0xD5 => { let nn = self.regs.de(); self.push(memory, nn); 4 },
//		0xE5 => { let nn = self.regs.hl(); self.push(memory, nn); 4 },
//		0xF5 => { let nn = self.regs.af(); self.push(memory, nn); 4 },
//		// POP qq
//		0xC1 => { let qq = self.pop(memory); self.regs.set_bc(qq); 3 },
//		0xD1 => { let qq = self.pop(memory); self.regs.set_de(qq); 3 },
//		0xE1 => { let qq = self.pop(memory); self.regs.set_hl(qq); 3 },
//		0xF1 => { let qq = self.pop(memory); self.regs.set_af(qq); 3 },
//		// LD SP, e (e = imm8 -128 to +127)
//		0xF8 => { unimplemented!(); },
//		// LD (nn), SP
//		0x08 => {
//			let nn = self.next_pointer(memory);
//			memory.write(nn, low!(self.regs.sp));
//			memory.write(nn.wrapping_add(1), high!(self.regs.sp));
//			5
//		},
//		// 8-bit Arithmetic
//		// ADD A, r
//		0x87 => { let r = self.regs.a; self.add_u8(r, false); 1 },
//		0x80 => { let r = self.regs.b; self.add_u8(r, false); 1 },
//		0x81 => { let r = self.regs.c; self.add_u8(r, false); 1 },
//		0x82 => { let r = self.regs.d; self.add_u8(r, false); 1 },
//		0x83 => { let r = self.regs.e; self.add_u8(r, false); 1 },
//		0x84 => { let r = self.regs.h; self.add_u8(r, false); 1 },
//		0x85 => { let r = self.regs.l; self.add_u8(r, false); 1 },
//		// ADD A, n
//		0xC6 => { let n = self.next_byte(memory); self.add_u8(n, false); 2 },
//		// ADD A, (HL)
//		0x86 => { let hl = memory.read(self.regs.hl()); self.add_u8(hl, false); 2 },
//		// ADC A, r
//		0x8F => { let r = self.regs.a; self.add_u8(r, true); 1 },
//		0x88 => { let r = self.regs.b; self.add_u8(r, true); 1 },
//		0x89 => { let r = self.regs.c; self.add_u8(r, true); 1 },
//		0x8A => { let r = self.regs.d; self.add_u8(r, true); 1 },
//		0x8B => { let r = self.regs.e; self.add_u8(r, true); 1 },
//		0x8C => { let r = self.regs.h; self.add_u8(r, true); 1 },
//		0x8D => { let r = self.regs.l; self.add_u8(r, true); 1 },			
//		// ADC A, n
//		0xCE => { let n = self.next_byte(memory); self.add_u8(n, true); 2 },
//		// ADC A, (HL)
//		0x8E => { let hl = memory.read(self.regs.hl()); self.add_u8(hl, true); 2 },
//		// SUB A, r
//		0x97 => { let r = self.regs.a; self.sub_u8(r, false); 1 },
//		0x90 => { let r = self.regs.b; self.sub_u8(r, false); 1 },
//		0x91 => { let r = self.regs.c; self.sub_u8(r, false); 1 },
//		0x92 => { let r = self.regs.d; self.sub_u8(r, false); 1 },
//		0x93 => { let r = self.regs.e; self.sub_u8(r, false); 1 },
//		0x94 => { let r = self.regs.h; self.sub_u8(r, false); 1 },
//		0x95 => { let r = self.regs.l; self.sub_u8(r, false); 1 },
//		// SUB A, n
//		0xD6 => { let n = self.next_byte(memory); self.sub_u8(n, false); 2 },
//		// SUB A, (HL)
//		0x96 => { let hl = memory.read(self.regs.hl()); self.sub_u8(hl, false); 2 },
//		// SBC A, r
//		0x9F => { let r = self.regs.a; self.sub_u8(r, true); 1 },
//		0x98 => { let r = self.regs.b; self.sub_u8(r, true); 1 },
//		0x99 => { let r = self.regs.c; self.sub_u8(r, true); 1 },
//		0x9A => { let r = self.regs.d; self.sub_u8(r, true); 1 },
//		0x9B => { let r = self.regs.e; self.sub_u8(r, true); 1 },
//		0x9C => { let r = self.regs.h; self.sub_u8(r, true); 1 },
//		0x9D => { let r = self.regs.l; self.sub_u8(r, true); 1 },			
//		// SBC A, n
//		0xDE => { let n = self.next_byte(memory); self.sub_u8(n, true); 2 },
//		// SBC A, (HL)
//		0x9E => { let hl = memory.read(self.regs.hl()); self.sub_u8(hl, true); 2 },
//		// AND A, r
//		0xA7 => { let r = self.regs.a; self.and_u8(r); 1 },
//		0xA0 => { let r = self.regs.b; self.and_u8(r); 1 },
//		0xA1 => { let r = self.regs.c; self.and_u8(r); 1 },
//		0xA2 => { let r = self.regs.d; self.and_u8(r); 1 },
//		0xA3 => { let r = self.regs.e; self.and_u8(r); 1 },
//		0xA4 => { let r = self.regs.h; self.and_u8(r); 1 },
//		0xA5 => { let r = self.regs.l; self.and_u8(r); 1 },
//		// AND A, n
//		0xE6 => { let n = self.next_byte(memory); self.and_u8(n); 2 },
//		// AND A, (HL)
//		0xA6 => { let hl = memory.read(self.regs.hl()); self.and_u8(hl); 2 },
//		// OR A, r
//		0xB7 => { let r = self.regs.a; self.or_u8(r); 1 },
//		0xB0 => { let r = self.regs.b; self.or_u8(r); 1 },
//		0xB1 => { let r = self.regs.c; self.or_u8(r); 1 },
//		0xB2 => { let r = self.regs.d; self.or_u8(r); 1 },
//		0xB3 => { let r = self.regs.e; self.or_u8(r); 1 },
//		0xB4 => { let r = self.regs.h; self.or_u8(r); 1 },
//		0xB5 => { let r = self.regs.l; self.or_u8(r); 1 },
//		// OR A, n
//		0xF6 => { let n = self.next_byte(memory); self.or_u8(n); 2 },
//		// OR A, (HL)
//		0xB6 => { let hl = memory.read(self.regs.hl()); self.or_u8(hl); 2 },
//		// XOR A, r
//		0xAF => { let r = self.regs.a; self.xor_u8(r); 1 },
//		0xA8 => { let r = self.regs.b; self.xor_u8(r); 1 },
//		0xA9 => { let r = self.regs.c; self.xor_u8(r); 1 },
//		0xAA => { let r = self.regs.d; self.xor_u8(r); 1 },
//		0xAB => { let r = self.regs.e; self.xor_u8(r); 1 },
//		0xAC => { let r = self.regs.h; self.xor_u8(r); 1 },
//		0xAD => { let r = self.regs.l; self.xor_u8(r); 1 },
//		// XOR A, n
//		0xEE => { let n = self.next_byte(memory); self.xor_u8(n); 2 },
//		// XOR A, (HL)
//		0xAE => { let hl = memory.read(self.regs.hl()); self.xor_u8(hl); 2 },
//		// CP A, r
//		0xBF => { let r = self.regs.a; self.cp_u8(r); 1 },
//		0xB8 => { let r = self.regs.b; self.cp_u8(r); 1 },
//		0xB9 => { let r = self.regs.c; self.cp_u8(r); 1 },
//		0xBA => { let r = self.regs.d; self.cp_u8(r); 1 },
//		0xBB => { let r = self.regs.e; self.cp_u8(r); 1 },
//		0xBC => { let r = self.regs.h; self.cp_u8(r); 1 },
//		0xBD => { let r = self.regs.l; self.cp_u8(r); 1 },
//		// CP A, n
//		0xFE => { let n = self.next_byte(memory); self.cp_u8(n); 2 },
//		// CP A, (HL)
//		0xBE => { let hl = memory.read(self.regs.hl()); self.cp_u8(hl); 2 },
//		// INC r
//		0x3C => { let r = self.regs.a; self.regs.a = self.inc_u8(r); 1 },
//		0x04 => { let r = self.regs.b; self.regs.b = self.inc_u8(r); 1 },
//		0x0C => { let r = self.regs.c; self.regs.c = self.inc_u8(r); 1 },
//		0x14 => { let r = self.regs.d; self.regs.d = self.inc_u8(r); 1 },
//		0x1C => { let r = self.regs.e; self.regs.e = self.inc_u8(r); 1 },
//		0x24 => { let r = self.regs.h; self.regs.h = self.inc_u8(r); 1 },
//		0x2C => { let r = self.regs.l; self.regs.l = self.inc_u8(r); 1 },
//		// INC (HL)
//		0x34 => { let n = memory.read(self.regs.hl()); memory.write(self.regs.hl(), self.inc_u8(n)); 3 },
//		// DEC r
//		0x3D => { let r = self.regs.a; self.regs.a = self.dec_u8(r); 1 },
//		0x05 => { let r = self.regs.b; self.regs.b = self.dec_u8(r); 1 },
//		0x0D => { let r = self.regs.c; self.regs.c = self.dec_u8(r); 1 },
//		0x15 => { let r = self.regs.d; self.regs.d = self.dec_u8(r); 1 },
//		0x1D => { let r = self.regs.e; self.regs.e = self.dec_u8(r); 1 },
//		0x25 => { let r = self.regs.h; self.regs.h = self.dec_u8(r); 1 },
//		0x2D => { let r = self.regs.l; self.regs.l = self.dec_u8(r); 1 },
//		// DEC (HL)
//		0x35 => { let n = memory.read(self.regs.hl()); memory.write(self.regs.hl(), self.dec_u8(n)); 3 },
//		// ADD HL, rr
//		0x09 => { let rr = self.regs.bc(); self.add_hl(rr); 2 }, 
//		0x19 => { let rr = self.regs.de(); self.add_hl(rr); 2 }, 
//		0x29 => { let rr = self.regs.hl(); self.add_hl(rr); 2 }, 
//		0x39 => { let rr = self.regs.sp; self.add_hl(rr); 2 },
//		// ADD SP, e
//		0xE8 => { unimplemented!(); }, 
//		// INC ss (no flags changed here)
//		0x03 => { let rr = self.regs.bc().wrapping_add(1); self.regs.set_bc(rr); 2 }, 
//		0x13 => { let rr = self.regs.de().wrapping_add(1); self.regs.set_de(rr); 2 }, 
//		0x23 => { let rr = self.regs.hl().wrapping_add(1); self.regs.set_hl(rr); 2 }, 
//		0x33 => { let rr = self.regs.sp.wrapping_add(1); self.regs.sp = rr; 2 },
//		// DEC ss (no flags changed here)
//		0x0B => { let rr = self.regs.bc().wrapping_sub(1); self.regs.set_bc(rr); 2 }, 
//		0x1B => { let rr = self.regs.de().wrapping_sub(1); self.regs.set_de(rr); 2 }, 
//		0x2B => { let rr = self.regs.hl().wrapping_sub(1); self.regs.set_hl(rr); 2 }, 
//		0x3B => { let rr = self.regs.sp.wrapping_sub(1); self.regs.sp = rr; 2 },
////		// JP nn
//		0xC3 => { let nn = self.next_pointer(memory); self.jump(nn); 4 },
//		// JP cc, nn
//		0xC2 => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::NotZero) },
//		0xCA => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::Zero) },
//		0xD2 => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::NotCarry) },
//		0xDA => { let nn = self.next_pointer(memory); self.jump_if(nn, Condition::Carry) },
//		
//		// JR e
//		0x18 => { let e = self.next_byte(memory) as i8; self.jump_rel(e); 3 },
//		// JR cc, e
//		0x20 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::NotZero) },
//		0x28 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::Zero) },
//		0x30 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::NotCarry) },
//		0x38 => { let e = self.next_byte(memory) as i8; self.jump_rel_if(e, Condition::Carry) },
//		// JP (HL)
//		0xE9 => { let hl = self.regs.hl(); self.jump(hl); 1 },
//		// CALL
//		0xCD => { let nn = self.next_pointer(memory); self.call(memory, nn); 6 },
//		// CALL cc
//		0xC4 => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::NotZero) },
//		0xCC => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::Zero) },
//		0xD4 => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::NotCarry) },
//		0xDC => { let nn = self.next_pointer(memory); self.call_if(memory, nn, Condition::Carry) },
//		// RET
//		0xC9 => { self.ret(memory); 4 },
//		// RET cc
//		0xC0 => { self.ret_if(memory, Condition::NotZero) },
//		0xC8 => { self.ret_if(memory, Condition::Zero) },
//		0xD0 => { self.ret_if(memory, Condition::NotCarry) },
//		0xD8 => { self.ret_if(memory, Condition::Carry) },
//		// RETI (return from interrupt)
//		0xD9 => { unimplemented!(); },
//		// RST t
//		0xC7 => { self.rst(memory, RstVector::Rst1); 4 },
//		0xCF => { self.rst(memory, RstVector::Rst2); 4 },
//		0xD7 => { self.rst(memory, RstVector::Rst3); 4 },
//		0xDF => { self.rst(memory, RstVector::Rst4); 4 },
//		0xE7 => { self.rst(memory, RstVector::Rst5); 4 },
//		0xEF => { self.rst(memory, RstVector::Rst6); 4 },
//		0xF7 => { self.rst(memory, RstVector::Rst7); 4 },
//		0xFF => { self.rst(memory, RstVector::Rst8); 4 },
////		// NOP
//		0x00 => { 1 }, // easiest opcode of my life
////		// GBCPUMAN
//		0xF3 => { memory.interrupt_handler.disable(); 1 }, // Disable interrupts
//		0xFB => { memory.interrupt_handler.enable();  1 }, // Enable interrupts
////		0xC3 => { self.regs.pc = self.next_pointer(memory); 4 },
//		_ => panic!("Unknown Opcode: ${:02X} | {}", opcode, opcode)

		_ => format!("Unknown: {:02X}", opcode)
	}
}