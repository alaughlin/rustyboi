use std::str;

// Memory Layout:
// 0000-3FFF   16KB ROM Bank 00            (ROM)  (in cartridge, fixed at bank 00)
// 4000-7FFF   16KB ROM Bank 01..NN        (ROM)  (in cartridge, switchable bank number)
// 8000-9FFF   8KB Video RAM               (VRAM) (switchable bank 0-1 in CGB Mode)
// A000-BFFF   8KB External RAM            (in cartridge, switchable bank, if any)
// C000-CFFF   4KB Work RAM Bank 0         (WRAM)
// D000-DFFF   4KB Work RAM Bank 1         (WRAM) (switchable bank 1-7 in CGB Mode)
// E000-FDFF   Same as C000-DDFF           (ECHO) (typically not used)
// FE00-FE9F   Sprite Attribute Table      (OAM)
// FEA0-FEFF   Not Usable
// FF00-FF7F   I/O Ports
// FF80-FFFE   High RAM                    (ZRAM)
// FFFF        Interrupt Enable Register

pub struct MMU {
    rom_bank_0: Vec<u8>,
    rom_bank_nn: Vec<u8>,
    vram: Vec<u8>,
    eram: Vec<u8>,
    wram: Vec<u8>,
    io: Vec<u8>,
    zram: Vec<u8>,
}

impl Default for MMU {
    fn default() -> MMU {
        MMU {
            rom_bank_0: vec![0; 16384],
            rom_bank_nn: vec![0; 16384],
            vram: vec![0; 8192],
            eram: vec![0; 8192],
            wram: vec![0; 8192],
            io: vec![0; 128],
            zram: vec![0; 128],
        }
    }
}

impl MMU {

    pub fn new() -> MMU {
        Default::default()
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let (memory_slice, offset) = self.get_memory_slice(address);
        let idx = (address - offset) as usize;

        memory_slice[idx] = data;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        let (memory_slice, offset) = self.get_memory_slice(address);
        let idx = (address - offset) as usize;

        memory_slice[idx]
    }

    pub fn incr(&mut self, address: u16) {
        let (memory_slice, offset) = self.get_memory_slice(address);
        let idx = (address - offset) as usize;

        memory_slice[idx] += 1;
    }

    pub fn decr(&mut self, address: u16) {
        let (memory_slice, offset) = self.get_memory_slice(address);
        let idx = (address - offset) as usize;

        memory_slice[idx] -= 1;
    }

    pub fn push(&mut self, sp: &mut u16, data: u8) {
        *sp -= 8;
        self.write(*sp, data);
    }

    pub fn pop(&mut self, sp: &mut u16) -> u8 {
        let val = self.read(*sp);
        self.write(*sp, 0);
        *sp += 8;

        val
    }

    pub fn load_game(&mut self, bank_0: Vec<u8>, bank_1: Vec<u8>) {
        self.rom_bank_0 = bank_0;
        self.rom_bank_nn = bank_1;
    }

    pub fn get_game_title(&mut self) -> &str {
        let buffer = &self.rom_bank_0[0x134..0x144];

        match str::from_utf8(buffer) {
            Ok(v) => v.trim_right_matches(char::from(0)),
            Err(e) => panic!("Invalid UTF-8 sequence for game title: {}", e)
        }
    }

    pub fn init_io(&mut self) {
        self.write(0xFF10, 0x80);
        self.write(0xFF11, 0xBF);
        self.write(0xFF12, 0xF3);
        self.write(0xFF14, 0xBF);
        self.write(0xFF16, 0x3F);
        self.write(0xFF19, 0xBF);
        self.write(0xFF1A, 0x7F);
        self.write(0xFF1B, 0xFF);
        self.write(0xFF1C, 0x9F);
        self.write(0xFF1E, 0xBF);
        self.write(0xFF20, 0xFF);
        self.write(0xFF23, 0xBF);
        self.write(0xFF24, 0x77);
        self.write(0xFF25, 0xF3);
        self.write(0xFF26, 0xF1);  // TODO: this value varies based on cart type
        self.write(0xFF40, 0x91);
        self.write(0xFF47, 0xFC);
        self.write(0xFF48, 0xFF);
        self.write(0xFF49, 0xFF);
    }

    fn get_memory_slice(&mut self, address: u16) -> (&mut Vec<u8>, u16) {
        if address < 0x4000 {
            return (&mut self.rom_bank_0, 0x0000);
        } else if address < 0x8000 {
            return (&mut self.rom_bank_nn, 0x4000);
        } else if address < 0xA000 {
            return (&mut self.vram, 0x8000);
        } else if address < 0xC000 {
            return (&mut self.eram, 0xA000);
        } else if address < 0xE000 {
            return (&mut self.wram, 0xC000);
        } else if address < 0xFF80 {
            return (&mut self.io, 0xFF00);
        } else if address >= 0xFF80 && address <= 0xFFFE {
            return (&mut self.zram, 0xFF80);
        }

        return (&mut self.zram, 0xFF80);
    }
}