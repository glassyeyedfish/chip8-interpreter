use macroquad::prelude::*;
// use rfd::FileDialog;

use chip8-interpreter::Chip8;

#[macroquad::main("Chip8Emu")]
async fn main() {

    // Set up all the emulator data.
    let mut chip8 = Chip8::new();
    // let mut loop_counter = 0;

    // Load a rom form a file path.
    // TODO: Get path using more flexble method.
    chip8.load_rom("res/RPS.ch8").await;

    chip8.dump_data(0x200, 0x300);

    // main loop
    loop {

        // Fetch Instruction
        let instruction = chip8.fetch();

        // Decode / Execute
        match instruction & 0xF000 {
            0x0000 => match instruction {
                0x00E0 => chip8.cls_0(),
                0x00EE => chip8.ret_0(),
                _ => chip8.ignore(),
            },
            0x1000 => chip8.jp_1(instruction),
            0x2000 => chip8.call_2(instruction),
            0x3000 => chip8.se_3(instruction),
            0x4000 => chip8.sne_4(instruction),
            0x5000 => chip8.se_5(instruction),
            0x6000 => chip8.ld_6(instruction),
            0x7000 => chip8.add_7(instruction),
            0x8000 => match instruction & 0x000F {
                0x0 => chip8.ld_8(instruction),
                0x1 => chip8.or_8(instruction),
                0x2 => chip8.and_8(instruction),
                0x3 => chip8.xor_8(instruction),
                0x4 => chip8.add_8(instruction),
                0x5 => chip8.sub_8(instruction),
                0x6 => chip8.shr_8(instruction),
                0x7 => chip8.subn_8(instruction),
                0xE => chip8.shl_8(instruction),
                _ => chip8.unknown(),
            },
            0x9000 => chip8.sne_9(instruction),
            0xA000 => chip8.ld_a(instruction),
            0xB000 => chip8.jp_b(instruction),
            0xC000 => chip8.rnd_c(instruction),
            0xD000 => chip8.drw_d(instruction),
            0xE000 => match instruction & 0x00FF {
                0x9E => chip8.skp_e(instruction),
                0xA1 => chip8.sknp_e(instruction),
                _ => chip8.unknown(),
            },
            0xF000 => match instruction & 0x00FF {
                0x07 => chip8.ld_dt(instruction),
                0x0A => chip8.ld_k(instruction),
                0x1E => chip8.ld_i(instruction),
                0x29 => chip8.ld_f(instruction),
                0x33 => chip8.ld_b(instruction),
                0x55 => chip8.ld_arr_x(instruction),
                0x65 => chip8.ld_x_arr(instruction),
                _ => chip8.unknown(),
            },
            _ => chip8.unknown(),
        };

        // Increment Program Counter
        chip8.inc_pc();

        clear_background(VIOLET);
        // next_frame().await
    }
}
