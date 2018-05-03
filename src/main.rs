mod gameboy;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut gb = gameboy::Gameboy { pc: 0x100, sp: 0xFFFE, ..Default::default() };
    gb.load_game();

    for _x in 0..7 {
        gb.step();
        sleep(Duration::new(1, 0));
    }
}
