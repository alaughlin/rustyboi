use gameboy::{ mmu, opcodes };

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub m: u8,
    pub t: u8,
}

// f: flag register
// 7 6 5 4 3 2 1 0
// Z N H C 0 0 0 0

impl Default for Registers {

    fn default() -> Registers {
        Registers {
            a: 0x01,  // TODO: this value varies based on cart type
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
        }
    }
}

#[derive(Default)]
pub struct Clock {
    pub m: u8,
    pub t: u8,
}

pub struct CPU {
    pub registers: Registers,
    pub clock: Clock,
    pub pc: u16,
    pub sp: u16,
}

impl Default for CPU {

    fn default() -> CPU {
        CPU {
            registers: Registers { ..Default::default() },
            clock: Clock { ..Default::default() },
            pc: 0x100,
            sp: 0xFFFE,
        }
    }
}

impl CPU {

    pub fn execute(&mut self, mmu: &mut mmu::MMU) {
        let opcode = mmu.read(self.pc);
        println!("opcode: {:x}", opcode);

        match opcode {
            0x00 => {
                opcodes::nop(&mut self.pc);
            },
            0x01 => {
                let nn = get_nn(&mut self.pc, mmu, false);
                opcodes::ld_rr_nn(&mut self.pc, &mut self.registers.b, &mut self.registers.c, nn);
            },
            0x02 => {
                let bc = get_address(self.registers.b, self.registers.c);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, bc, self.registers.a);
            },
            0x06 => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_n(&mut self.pc, &mut self.registers.b, n);
            },
            0x08 => {
                let nn = get_nn(&mut self.pc, mmu, false);
                opcodes::ld_mem_nn_sp(&mut self.pc, mmu, nn, self.sp)
            },
            0x0A => {
                let bc = get_address(self.registers.b, self.registers.c);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, bc, &mut self.registers.a);
            },
            0x0E => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_n(&mut self.pc, &mut self.registers.c, n);
            },
            0x11 => {
                let nn = get_nn(&mut self.pc, mmu, false);
                opcodes::ld_rr_nn(&mut self.pc, &mut self.registers.d, &mut self.registers.e, nn);
            },
            0x12 => {
                let de = get_address(self.registers.d, self.registers.e);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, de, self.registers.a);
            },
            0x16 => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_n(&mut self.pc, &mut self.registers.d, n);
            },
            0x1A => {
                let de = get_address(self.registers.d, self.registers.e);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, de, &mut self.registers.a);
            },
            0x1E => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_n(&mut self.pc, &mut self.registers.e, n);
            },
            0x21 => {
                let nn = get_nn(&mut self.pc, mmu, false);
                opcodes::ld_rr_nn(&mut self.pc, &mut self.registers.h, &mut self.registers.l, nn);
            },
            0x22 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_inc_r(&mut self.pc, mmu, hl, self.registers.a);
            },
            0x26 => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_n(&mut self.pc, &mut self.registers.h, n);
            },
            0x2A => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr_inc(&mut self.pc, mmu, hl, &mut self.registers.a);
            },
            0x2E => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_n(&mut self.pc, &mut self.registers.l, n);
            },
            0x31 => {
                let nn = get_nn(&mut self.pc, mmu, false);
                opcodes::ld_sp_nn(&mut self.pc, &mut self.sp, nn);
            },
            0x32 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_dec_r(&mut self.pc, mmu, hl, self.registers.a);
            },
            0x3A => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr_dec(&mut self.pc, mmu, hl, &mut self.registers.a);
            },
            0x36 => {
                let hl = get_address(self.registers.h, self.registers.l);
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_mem_rr_n(&mut self.pc, mmu, hl, n);
            },
            0x40 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.b);
            },
            0x41 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.b, self.registers.c);
            },
            0x42 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.b, self.registers.d);
            },
            0x43 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.b, self.registers.e);
            },
            0x44 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.b, self.registers.h);
            },
            0x45 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.b, self.registers.l);
            },
            0x46 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.b);
            },
            0x47 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.b, self.registers.a);
            },
            0x48 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.c, self.registers.b);
            },
            0x49 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.c);
            },
            0x4A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.c, self.registers.d);
            },
            0x4B => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.c, self.registers.e);
            },
            0x4C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.c, self.registers.h);
            },
            0x4D => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.c, self.registers.l);
            },
            0x4E => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.c);
            },
            0x4F => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.c, self.registers.a);
            },
            0x50 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.d, self.registers.b);
            },
            0x51 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.d, self.registers.c);
            },
            0x52 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.d);
            },
            0x53 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.d, self.registers.e);
            },
            0x54 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.d, self.registers.h);
            },
            0x55 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.d, self.registers.l);
            },
            0x56 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.d);
            },
            0x57 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.d, self.registers.a);
            },
            0x58 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.e, self.registers.b);
            },
            0x59 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.e, self.registers.c);
            },
            0x5A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.e, self.registers.d);
            },
            0x5B => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.e);
            },
            0x5C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.e, self.registers.h);
            },
            0x5D => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.e, self.registers.l);
            },
            0x5E => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.e);
            },
            0x5F => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.e, self.registers.a);
            },
            0x60 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.h, self.registers.b);
            },
            0x61 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.h, self.registers.c);
            },
            0x62 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.h, self.registers.d);
            },
            0x63 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.h, self.registers.e);
            },
            0x64 => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.h);
            },
            0x65 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.h, self.registers.l);
            },
            0x66 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.h);
            },
            0x67 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.h, self.registers.a);
            },
            0x68 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.l, self.registers.b);
            },
            0x69 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.l, self.registers.c);
            },
            0x6A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.l, self.registers.d);
            },
            0x6B => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.l, self.registers.e);
            },
            0x6C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.l, self.registers.h);
            },
            0x6D => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.l);
            },
            0x6E => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.l);
            },
            0x6F => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.l, self.registers.a);
            },
            0x70 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.b);
            },
            0x71 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.c);
            },
            0x72 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.d);
            },
            0x73 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.e);
            },
            0x74 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.h);
            },
            0x75 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.l);
            },
            0x77 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_mem_rr_r(&mut self.pc, mmu, hl, self.registers.a);
            },
            0x78 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.a, self.registers.b);
            },
            0x79 => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.a, self.registers.c);
            },
            0x7A => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.a, self.registers.d);
            },
            0x7B => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.a, self.registers.e);
            },
            0x7C => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.a, self.registers.h);
            },
            0x7D => {
                opcodes::ld_r1_r2(&mut self.pc, &mut self.registers.a, self.registers.l);
            },
            0x7E => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, hl, &mut self.registers.a);
            },
            0x7F => {
                opcodes::ld_r1_r1(&mut self.pc, &mut self.registers.a);
            },
            0xAF => {
                opcodes::xor_a(&mut self.pc, self.registers.a, &mut self.registers.f);
            },
            0xC1 => {
                opcodes::pop_rr(&mut self.pc, mmu, &mut self.sp, &mut self.registers.b, & mut self.registers.c);
            },
            0xC3 => {
                let nn = get_nn(&mut self.pc, mmu, true);
                opcodes::jp_nn(&mut self.pc, nn);
            },
            0xC5 => {
                opcodes::push_rr(&mut self.pc, mmu, &mut self.sp, self.registers.b, self.registers.c);
            },
            0xD1 => {
                opcodes::pop_rr(&mut self.pc, mmu, &mut self.sp, &mut self.registers.d, &mut self.registers.e);
            },
            0xD5 => {
                opcodes::push_rr(&mut self.pc, mmu, &mut self.sp, self.registers.d, self.registers.e);
            },
            0xE0 => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_mem_n_r(&mut self.pc, mmu, 0xFF00+(n as u16), self.registers.a);
            },
            0xE1 => {
                opcodes::pop_rr(&mut self.pc, mmu, &mut self.sp, &mut self.registers.h, &mut self.registers.l);
            },
            0xE2 => {
                opcodes::ld_mem_r1_r2(&mut self.pc, mmu, self.registers.c, self.registers.a);
            },
            0xE5 => {
                opcodes::push_rr(&mut self.pc, mmu, &mut self.sp, self.registers.h, self.registers.l);
            },
            0xEA => {
                let nn = get_nn(&mut self.pc, mmu, true);
                opcodes::ld_mem_nn_r(&mut self.pc, mmu, nn, self.registers.a);
            },
            0xF0 => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_r_mem_n(&mut self.pc, mmu, n, &mut self.registers.a);
            },
            0xF1 => {
                opcodes::pop_rr(&mut self.pc, mmu, &mut self.sp, &mut self.registers.a, &mut self.registers.f);
            },
            0xF2 => {
                opcodes::ld_r1_mem_r2(&mut self.pc, mmu, &mut self.registers.c, self.registers.a);
            },
            0xF5 => {
                opcodes::push_rr(&mut self.pc, mmu, &mut self.sp, self.registers.a, self.registers.f);
            },
            0xF8 => {
                let n = get_n(&mut self.pc, mmu);
                opcodes::ld_rr_spn(&mut self.pc, &mut self.registers.h, &mut self.registers.l, self.sp, n, &mut self.registers.f);
            },
            0xF9 => {
                let hl = get_address(self.registers.h, self.registers.l);
                opcodes::ld_sp_hl(&mut self.pc, &mut self.sp, hl);
            },
            0xFA => {
                let rr = get_nn(&mut self.pc, mmu, true);
                opcodes::ld_r_mem_rr(&mut self.pc, mmu, rr, &mut self.registers.a);
            },
            0xFF => {
                opcodes::rst_38(&mut self.pc, &mut self.sp, mmu);
            }
            _ => self.pc += 1
        }
    }
}

fn get_address(r1: u8, r2: u8) -> u16 {
    let first_half = (r1 as u16) << 8;
    let second_half = r2 as u16;

    first_half + second_half
}

fn get_n(pc: &mut u16, mmu: &mut mmu::MMU) -> u8 {
    mmu.read(*pc+1)
}

fn get_nn(pc: &mut u16, mmu: &mut mmu::MMU, little_endian: bool) -> u16 {
    let high_byte =  mmu.read(*pc+1) as u16;
    let low_byte = (mmu.read(*pc+2) as u16) << 8;

    match little_endian {
        true => low_byte + high_byte,
        false => high_byte + low_byte
    }
}
