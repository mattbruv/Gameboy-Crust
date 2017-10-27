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

	// Perform one step of the fetch-decode-execute cycle 
	pub fn step(&mut self, memory: &mut Interconnect) -> usize {

		println!("PC: ${:04X}", self.regs.pc);
		let opcode = self.next_byte(memory);
		hex!(opcode);

		// decodes/excecutes each operation and returns cycles taken
		match opcode {

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

			0xC3 => { self.regs.pc = self.next_pointer(memory); 4 }
			_ => panic!("Unknown Opcode: ${:02X} | {}", opcode, opcode)
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

	pub fn debug(&mut self) {
		println!("{}", self.regs);
	}

}