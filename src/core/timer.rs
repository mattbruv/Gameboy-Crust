use core::helper::*;
use core::interrupt::*;

// TODO: update clock speed to reflect emulated speed
const CLOCK_SPEED: i32 = 4194304;

#[derive(Eq, PartialEq, Copy, Clone)]
enum TimerFrequency {
    Mode0 = 4096, // Hz..
    Mode1 = 262144,
    Mode2 = 65536,
    Mode3 = 16384,
}

pub struct Timer {
    DIV:  MemoryRegister,
    TIMA: MemoryRegister,
    TMA:  MemoryRegister,
    TAC:  MemoryRegister,
    divider_counter: i32,
    timer_counter: i32,
    frequency: TimerFrequency,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            DIV:  MemoryRegister::new(0x00),
            TIMA: MemoryRegister::new(0x00),
            TMA:  MemoryRegister::new(0x00),
            TAC:  MemoryRegister::new(0x00),
            divider_counter: 0,
            timer_counter: 0,
            frequency: TimerFrequency::Mode0
        }
    }

    pub fn cycles(&mut self, cycles: usize, interrupt: &mut InterruptHandler) {
        self.update_divider(cycles);

        let new_freq = self.get_frequency();

        if self.frequency != new_freq {
            self.set_frequency(new_freq);
        }

        if self.timer_enabled() {
            self.timer_counter -= cycles as i32;

            if self.timer_counter <= 0 {
                let timer_value = self.TIMA.get();
                self.set_frequency(new_freq);
                println!("{}", timer_value);

                // If TIMA overflows
                if (timer_value == 255) {
                    self.TIMA.set(self.TMA.get());
                    interrupt.request_interrupt(InterruptFlag::Timer);
                } else {
                    self.TIMA.set(timer_value + 1);
                }
            }
        }
    }

    fn set_frequency(&mut self, frequency: TimerFrequency) {
        self.frequency = frequency;
        self.timer_counter = CLOCK_SPEED / (frequency as i32);
    }

    fn get_frequency(&self) -> TimerFrequency {
        let mode = self.TAC.get() & 3;
        match mode {
            0 => TimerFrequency::Mode0,
            1 => TimerFrequency::Mode1,
            2 => TimerFrequency::Mode2,
            3 => TimerFrequency::Mode3,
            _ => unreachable!(),
        }
    }

    // Divider register is incremented at 16384Hz
    // Cycles per update = Clock speed / 16384 = 256
    // TODO: register is updated at 32768Hz in CGB mode
    fn update_divider(&mut self, cycles: usize) {
        self.divider_counter += cycles as i32;

        if self.divider_counter >= 256 {
            self.DIV.add(1);
            self.divider_counter = 0;
        }
    }

    fn timer_enabled(&self) -> bool {
        self.TAC.is_set(Bit::Bit2)
    }

    pub fn read_div(&self) -> u8 {
        self.DIV.get()
    }

    pub fn write_div(&mut self, data: u8) {
        // writing to div resets it
        self.DIV.clear();
    }

    pub fn read_counter(&self) -> u8 {
        self.TIMA.get()
    }

    pub fn write_counter(&mut self, data: u8) {
        self.TIMA.set(data);
    }

    pub fn read_modulo(&self) -> u8 {
        self.TMA.get()
    }

    pub fn write_modulo(&mut self, data: u8) {
        self.TMA.set(data);
    }

    pub fn read_control(&self) -> u8 {
        self.TAC.get()
    }

    pub fn write_control(&mut self, data: u8) {
        self.TAC.set(data);
    }
}
