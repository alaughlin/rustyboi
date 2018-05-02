mod mmu;
mod opcodes;

use std::io::prelude::*;
use std::fs::File;

#[derive(Default)]
pub struct Gameboy {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub mmu: mmu::MMU,
}

impl Gameboy {

    pub fn load_game(&mut self) {
        let mut f = File::open("/Users/adam/Projects/rustyboy/tetris.gb")
            // TODO: handle this
            .expect("file not found");

        let mut buffer = [0; 32768];

        f.read_exact(&mut buffer)
            // TODO: handle this
            .expect("wtf");

        self.mmu.rom = buffer.to_vec();
    }

    pub fn step(&mut self) {
        let opcode = self.mmu.read(self.pc);
        println!("opcode: {:x}", opcode);

        // potentially used values
        let bc = get_address(self.b, self.c);
        let de = get_address(self.d, self.e);
        let hl = get_address(self.h, self.l);
        let n = self.mmu.read(self.pc+1);

        match opcode {
            0x00 => {
                opcodes::nop(&mut self.pc);
            },
            0x02 => {
                opcodes::ld_rr_r(&mut self.pc, &mut self.mmu, bc, self.a);
            },
            0x06 => {
                opcodes::ld_r_n(&mut self.pc, &mut self.b, n);
            },
            0x0A => {
                opcodes::ld_r_rr(&mut self.pc, &mut self.mmu, bc, &mut self.a);
            },
            0x0E => {
                opcodes::ld_r_n(&mut self.pc, &mut self.c, n);
            },
            0x12 => {
                opcodes::ld_rr_r(&mut self.pc, &mut self.mmu, de, self.a);
            },
            0x16 => {
                opcodes::ld_r_n(&mut self.pc, &mut self.d, n);
            },
            0x1A => {
                opcodes::ld_r_rr(&mut self.pc, &mut self.mmu, de, &mut self.a);
            },
            0x1E => {
                opcodes::ld_r_n(&mut self.pc, &mut self.e, n);
            },
            0x21 => {
                let fb =  self.mmu.read(self.pc+1);
                let sb = self.mmu.read(self.pc+2);

                opcodes::ld_hl_nn(&mut self.pc, &mut self.mmu, hl, fb,sb);
            },
            0x26 => {
                opcodes::ld_r_n(&mut self.pc, &mut self.h, n);
            },
            0x2E => {
                opcodes::ld_r_n(&mut self.pc, &mut self.l, n);
            },
            0x36 => {
                opcodes::ld_hl_n(&mut self.pc, &mut self.mmu, hl, n);
            },
            0x40 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.b);
            },
            0x41 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.b, self.c);
            },
            0x42 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.b, self.d);
            },
            0x43 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.b, self.e);
            },
            0x44 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.b, self.h);
            },
            0x45 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.b, self.l);
            },
            0x46 => {
                opcodes::ld_r1_hl(&mut self.pc, &mut self.mmu, hl, &mut self.b);
            },
            0x47 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.b, self.a);
            },
            0x48 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.c, self.b);
            },
            0x49 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.c);
            },
            0x4A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.c, self.d);
            },
            0x4B => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.c, self.e);
            },
            0x4C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.c, self.h);
            },
            0x4D => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.c, self.l);
            },
            0x4E => {
                opcodes::ld_r1_hl(&mut self.pc, &mut self.mmu, hl, &mut self.c);
            },
            0x4F => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.c, self.a);
            },
            0x50 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.d, self.b);
            },
            0x51 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.d, self.c);
            },
            0x52 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.d);
            },
            0x53 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.d, self.e);
            },
            0x54 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.d, self.h);
            },
            0x55 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.d, self.l);
            },
            0x56 => {
                opcodes::ld_r1_hl(&mut self.pc, &mut self.mmu, hl, &mut self.d);
            },
            0x57 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.d, self.a);
            },
            0x58 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.e, self.b);
            },
            0x59 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.e, self.c);
            },
            0x5A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.e, self.d);
            },
            0x5B => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.e);
            },
            0x5C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.e, self.h);
            },
            0x5D => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.e, self.l);
            },
            0x5E => {
                opcodes::ld_r1_hl(&mut self.pc, &mut self.mmu, hl, &mut self.e);
            },
            0x5F => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.e, self.a);
            },
            0x60 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.h, self.b);
            },
            0x61 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.h, self.c);
            },
            0x62 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.h, self.d);
            },
            0x63 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.h, self.e);
            },
            0x64 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.h);
            },
            0x65 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.h, self.l);
            },
            0x66 => {
                opcodes::ld_r1_hl(&mut self.pc, &mut self.mmu, hl, &mut self.h);
            },
            0x67 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.h, self.a);
            },
            0x68 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.l, self.b);
            },
            0x69 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.l, self.c);
            },
            0x6A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.l, self.d);
            },
            0x6B => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.l, self.e);
            },
            0x6C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.l, self.h);
            },
            0x6D => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.l);
            },
            0x6E => {
                opcodes::ld_r1_hl(&mut self.pc, &mut self.mmu, hl, &mut self.l);
            },
            0x6F => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.l, self.a);
            },
            0x70 => {
                opcodes::ld_hl_r2(&mut self.pc, &mut self.mmu, hl, self.b);
            },
            0x71 => {
                opcodes::ld_hl_r2(&mut self.pc, &mut self.mmu, hl, self.c);
            },
            0x72 => {
                opcodes::ld_hl_r2(&mut self.pc, &mut self.mmu, hl, self.d);
            },
            0x73 => {
                opcodes::ld_hl_r2(&mut self.pc, &mut self.mmu, hl, self.e);
            },
            0x74 => {
                opcodes::ld_hl_r2(&mut self.pc, &mut self.mmu, hl, self.h);
            },
            0x75 => {
                opcodes::ld_hl_r2(&mut self.pc, &mut self.mmu, hl, self.l);
            },
            0x77 => {
                opcodes::ld_rr_r(&mut self.pc, &mut self.mmu, hl, self.a);
            },
            0x78 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.a, self.b);
            },
            0x79 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.a, self.c);
            },
            0x7A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.a, self.d);
            },
            0x7B => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.a, self.e);
            },
            0x7C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.a, self.h);
            },
            0x7D => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.a, self.l);
            },
            0x7E => {
                opcodes::ld_r_rr(&mut self.pc, &mut self.mmu, hl, &mut self.a);
            },
            0x7F => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.a);
            },
            0xAF => {
                opcodes::xor_a(&mut self.pc, self.a, &mut self.f);
            },
            0xC3 => {
                let nn = get_nn_little_endian(&mut self.pc, &mut self.mmu);
                opcodes::jp_nn(&mut self.pc, nn);
            },
            0xEA => {
                let nn = get_nn_little_endian(&mut self.pc, &mut self.mmu);
                opcodes::ld_nn_r(&mut self.pc, &mut self.mmu, nn, self.a);
            },
            0xFA => {
                let rr = get_nn_little_endian(&mut self.pc, &mut self.mmu);
                opcodes::ld_r_rr(&mut self.pc, &mut self.mmu, rr, &mut self.a);
            },
            0xFF => {
                opcodes::rst_38(&mut self.pc, &mut self.sp, &mut self.mmu);
            }
            _ => self.pc += 1,
        }
    }
}

fn get_address(r1: u8, r2: u8) -> u16 {
    let first_half = (r1 as u16) << 8;
    let second_half = r2 as u16;

    first_half + second_half
}

fn get_nn_little_endian(pc: &mut u16, mmu: &mut mmu::MMU) -> u16 {
    let low_byte = (mmu.read(*pc+2) as u16) << 8;
    let high_byte =  mmu.read(*pc+1) as u16;

    low_byte + high_byte
}