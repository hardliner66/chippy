use macroquad::*;

fn get_key() -> Option<u8> {
    for (i, key) in [
        KeyCode::J,
        KeyCode::Q,
        KeyCode::W,
        KeyCode::E,
        KeyCode::A,
        KeyCode::S,
        KeyCode::D,
        KeyCode::Y,
        KeyCode::X,
        KeyCode::C,
        KeyCode::H,
        KeyCode::K,
        KeyCode::R,
        KeyCode::F,
        KeyCode::V,
        KeyCode::L,
    ].iter().enumerate() {
        if is_key_down(*key) {
            return Some(i as u8);
        }
    }
    None
}

#[macroquad::main("InputKeys")]
async fn main() {
    let mut emu = chippy::Emulator::new(get_key);
    emu.load(include_bytes!("../roms/chip-8/TETRIS.bin")).unwrap();

    loop {
        clear_background(BLACK);
        
        draw_text(&format!("{:X}", emu.instruction_pointer()), 20.0, 20.0, 20.0, DARKGRAY);
        emu.clock().unwrap();
        next_frame().await
    }
}
