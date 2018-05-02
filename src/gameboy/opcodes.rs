use gameboy::mmu;

///// 8 bit loads /////

// 0x06, 0x0E, 0x16, 0x1E, 0x26, 0x2E
// loads value n into register r
pub fn ld_r_n(pc: &mut u16, r: &mut u8, n: u8) {
    *r = n;
    *pc += 2;
}

// 0x41, 0x42, 0x43, 0x44, 0x45, 0x47, 0x48, 0x4A, 0x4B, 0x4C, 0x4D, 0x4F
// 0x50, 0x51, 0x53, 0x54, 0x55, 0x57, 0x58, 0x59, 0x5A, 0x5C, 0x5D, 0x5F
// 0x60, 0x61, 0x62, 0x63, 0x65, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6F
// 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D
// loads value in register r2 into register r1
pub fn ld_r1_r2(pc: &mut u16, r1: &mut u8, r2: u8) {
    *r1 = r2;
    *pc += 1;
}

// 0x46, 0x4E, 0x56, 0x5E, 0x66, 0x6E, 0x7E
// loads value at (hl) into register r1
pub fn ld_r1_hl(pc: &mut u16, mmu: &mut mmu::MMU, hl: u16, r1: &mut u8) {
    *r1 = mmu.read(hl);
    *pc += 1;
}

// 0x36
// loads value n into (hl)
pub fn ld_hl_n(pc: &mut u16, mmu: &mut mmu::MMU, hl: u16, n: u8) {
    mmu.write(hl, n);
    *pc += 2;
}

// 0x70, 0x71, 0x72, 0x73, 0x74, 0x75
// loads value in register r2 into (hl)
pub fn ld_hl_r2(pc: &mut u16, mmu: &mut mmu::MMU, hl: u16, r2: u8) {
    mmu.write(hl, r2);
    *pc += 1;
}

// 0x40, 0x49, 0x52, 0x5B, 0x64, 0x6D, 0x7F
// loads value in register r1 into register r1
pub fn ld_r1_r1(pc: &mut u16, r1: &mut u8) {
    *r1 = *r1;
    *pc += 1;
}

// 0x0A, 0x1A, 0x7E
// loads value at (rr) into register r
pub fn ld_r_rr(pc: &mut u16, mmu: &mut mmu::MMU, rr: u16, r: &mut u8) {
    *r = mmu.read(rr);
    *pc += 1;
}

// 0x02, 0x12, 0x77
// loads value in register r into (rr)
pub fn ld_rr_r(pc: &mut u16, mmu: &mut mmu::MMU, rr: u16, r: u8) {
    mmu.write(rr, r);
    *pc += 1;
}

// 0xEA
// loads value in register r into (nn)
pub fn ld_nn_r(pc: &mut u16, mmu: &mut mmu::MMU, nn: u16, r: u8) {
    mmu.write(nn, r);
    *pc += 3;
}

///// 16 bit loads /////

// 0x21
pub fn ld_hl_nn(pc: &mut u16, mmu: &mut mmu::MMU, addr: u16, n1: u8, n2: u8) {
    mmu.write(addr, n1);
    mmu.write(addr+8, n2);

    *pc += 3;
}

///// 8 bit ALU /////

// 0xAF
pub fn xor_a(pc: &mut u16, a: u8, f: &mut u8) {
    if a ^ a == 0 {
        *f |= 0b10000000;
    }

    *f &= 0b10000000;

    *pc += 1;
}

///// jumps /////

// JP nn
// 0xC3
pub fn jp_nn(pc: &mut u16, addr: u16) {
    *pc = addr;
}

///// restarts /////

// 0xFF
pub fn rst_38(pc: &mut u16, sp: &mut u16, mmu: &mut mmu::MMU) {
    mmu.push(sp, *pc as u8);
    mmu.push(sp, (*pc >> 8) as u8);

    // TODO: unclear if this is what's supposed to happen
    *pc += 0x38;
}

///// misc /////

// 0x00
pub fn nop(pc: &mut u16) {
    *pc += 1;
}

// TODO: skipped these, address them later
// 0x3E