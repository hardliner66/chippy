#![allow(dead_code)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut emu = chippy::Emulator::default();
    emu.load(include_bytes!("../roms/chip-8/TETRIS.bin"))?;

    loop {
        println!("{:X}", emu.instruction_pointer());
        emu.clock()?;
    }
}
