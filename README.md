# ![Gameboy Crust](https://i.imgur.com/Z1GJZMU.png)

Gameboy Crust is a work-in-progress emulator for the [Game Boy Color](https://en.wikipedia.org/wiki/Game_Boy_Color) written in the [Rust programming language](https://www.rust-lang.org/en-US/). This project serves as a complete re-write of [my first attempt](https://github.com/mattbruv/Gameboy-Emulator) at Game Boy emulation, as well as an opportunity to learn Rust. My goal is for the design of this emulator to be much more abstracted and polished than its predecessor, along with providing full GBC functionality.

The name is derived from the idea that `Color` + `Rust` = `Crust`. It's not the most endearing name, but I think it's perfect. Crust is the most unappealing part of food and you probably aren't going to eat it unless you are really hungry. Much in the same way, Gameboy Crust is probably one of the least appealing ways to play your classic gameboy games since the system is 20+ years old and many better emulators exist for it. Maybe you are that weird person who eats nothing but crust. In that case, this is the emulator for you!

## Features
A checklist of all the planned components and features: 
- [ ] CPU *(SHARP LR35902)*
- [ ] Memory Map
- [ ] Cartridge Memory Bank Controllers
- [ ] Cartridge Battery backed SRAM
- [ ] Frequency/Timing
- [ ] Video Display
- [ ] Full GBC Color Palettes
- [ ] Joypad Input
- [ ] Audio Output
- [ ] Link Cable (via networking?)
- [ ] CPU overclocking
- [ ] Gameshark/Genie Cheats
- [ ] Trivial Debugger/Dissassembler
