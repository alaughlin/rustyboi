mod cpu;
mod mmu;
mod opcodes;

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::str;

pub struct Gameboy {
    pub cpu: cpu::CPU,
    pub mmu: mmu::MMU,
}

const ROM_BANK_SIZE: u16 = 16384;

impl Default for Gameboy {

    fn default() -> Gameboy {
        Gameboy {
            cpu: cpu::CPU { ..Default::default() },
            mmu: mmu::MMU { ..Default::default() },
        }
    }
}

impl Gameboy {

    // TODO: take a path to a rom
    pub fn load_game(&mut self) {
        let mut f = File::open("/Users/adam/Projects/rustyboi/tetris.gb")
            // TODO: handle this
            .expect("file not found");

        let bank_0 = get_rom_bank_vec(&mut f, 0);
        let bank_1 = get_rom_bank_vec(&mut f, 1);

        self.mmu.load_game(bank_0, bank_1);
    }

    pub fn get_game_title(&mut self) -> &str {
        let buffer = &self.mmu.rom_bank_0[0x134..0x143];

        let title = match str::from_utf8(buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence for game title: {}", e),
        };

        &title
    }

    pub fn power_on(&mut self) {
        self.mmu.init_io();
    }

    pub fn step(&mut self) {
        self.cpu.execute(&mut self.mmu);

    }
}

fn get_rom_bank_vec(file: &mut File, bank_number: u16) -> Vec<u8> {
    let mut buffer = [0; ROM_BANK_SIZE as usize];

    if bank_number > 0 {
        file.seek(SeekFrom::Start((bank_number * ROM_BANK_SIZE) as u64))
            // TODO: handle this
            .expect("wtf");
    }

    file.read_exact(&mut buffer)
        // TODO: handle this
        .expect("wtf");

    buffer.to_vec()
}
