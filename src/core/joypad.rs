use core::helper::*;
use core::interrupt::*;

pub const BUTTON_A: u8      = 0b00000001;
pub const BUTTON_B: u8      = 0b00000010;
pub const BUTTON_SELECT: u8 = 0b00000100;
pub const BUTTON_START: u8  = 0b00001000;
pub const PAD_RIGHT: u8     = 0b00000001;
pub const PAD_LEFT: u8      = 0b00000010;
pub const PAD_UP: u8        = 0b00000100;
pub const PAD_DOWN: u8      = 0b00001000;

#[derive(Debug)]
enum ReadInput {
    Directional,
    Buttons,
}

pub struct Joypad {
    read_next: ReadInput,
    button_state: u8,
    directional_state: u8,
    register: MemoryRegister,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            read_next: ReadInput::Buttons,
            button_state: 0x00,
            directional_state: 0x00,
            register: MemoryRegister::new(0b00110000),
        }
    }

    pub fn set_button_pressed(&mut self, interrupt: &mut InterruptHandler, input: u8, is_pressed: bool) {
        match is_pressed {
            true  => {
                self.button_state = self.button_state | input;
                interrupt.request_interrupt(InterruptFlag::Joypad);
            },
            false => { self.button_state = self.button_state & !input; },
        }
    }

    pub fn set_direction_pressed(&mut self, interrupt: &mut InterruptHandler, input: u8, is_pressed: bool) {
        match is_pressed {
            true  => {
                self.directional_state = self.directional_state | input;
                interrupt.request_interrupt(InterruptFlag::Joypad);
            },
            false => { self.directional_state = self.directional_state & !input; },
        }
    }

    pub fn read(&self) -> u8 {
        let mut result = self.register.get() & 0x30;
        let pad = match self.read_next {
            ReadInput::Directional => !self.directional_state,
            ReadInput::Buttons => !self.button_state,
        };
        result | (pad & 0x0F)
    }

    // A write to the joypad by the game determines
    // which controls we return on the next read.
    pub fn write(&mut self, data: u8) {
        self.register.set(data);
        if !self.register.is_set(Bit::Bit4) {
            self.read_next = ReadInput::Directional;
        }
        if !self.register.is_set(Bit::Bit5) {
            self.read_next = ReadInput::Buttons;
        }
    }
}
