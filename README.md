# ![Gameboy Crust](https://i.imgur.com/Z1GJZMU.png)

Gameboy Crust is a work-in-progress emulator for the [Game Boy Color](https://en.wikipedia.org/wiki/Game_Boy_Color) written in the [Rust programming language](https://www.rust-lang.org/en-US/). This project serves as a complete re-write of [my first attempt](https://github.com/mattbruv/Gameboy-Emulator) at Game Boy emulation, as well as an opportunity to learn Rust. My goal is for the design of this emulator to be much more abstracted and polished than its predecessor, along with providing full GBC functionality.

The name is derived from the idea that `Color` + `Rust` = `Crust`. It's not the most endearing name, but I think it's perfect. Crust is the least appealing part of food, and you probably aren't going to eat it unless you are really hungry. Much in the same way, Gameboy Crust is a relatively unattractive choice compared to professional emulators for this system. But maybe you are that strange person who prefers eating crust. If so, this is the emulator for you!

## Progress

Gameboy Crust is quite trivial at the moment. Video/Hardware emulation is very basic but will improve quickly over time. Full DMG (original gameboy) emulation is a priority before fleshing out GBC emulation.

![Progress](https://i.imgur.com/3u0Y2ID.png)

## Build / Running

Building Gameboy Crust relies on having [Rust](https://www.rust-lang.org/en-US/install.html) installed. After cloning this repository into a folder, all that is needed is a simple: `cargo run [--release] <path to ROM>`. All dependencies will be gathered and built automatically.

Once the project is completed, official releases will be compiled and released [here](https://github.com/mattbruv/Gameboy-Crust/releases).

## Controls
| Function | Key |
| --- | --- |
| A | <kbd>A</kbd> |
| B | <kbd>S</kbd> |
| Start | <kbd>Z</kbd> |
| Select | <kbd>X</kbd> |
| D-Pad Up | <kbd>🡱</kbd> |
| D-Pad Down | <kbd>🡳</kbd> |
| D-Pad Left | <kbd>🡰</kbd> |
| D-Pad Right | <kbd>🡲</kbd> |
| View VRAM | <kbd>V</kbd> |
| Speed x10 | <kbd>Space</kbd> |

## Feature Checklist
A checklist of all the planned system components as I complete them. The entries with check marks have been started on. *Italic* entries still need work.

- [X] *CPU (Sharp LR35902)*
- [X] *Memory / Memory Map*
- [X] *Cartridge Memory Bank Controllers*
- [ ] Cartridge Battery backed SRAM
- [X] Interrupt Controller
- [X] *Frequency/Timing*
- [X] *Video Display*
- [ ] Full GBC Color Palettes
- [X] Joypad Input
- [ ] Audio Output
- [ ] Link Cable (via networking?)
- [X] CPU overclocking
- [ ] Hardware Save States
- [ ] SRAM Save States
- [ ] Gameshark/Genie Cheats
- [ ] Trivial Debugger/Dissassembler
- [X] VRAM Viewer
