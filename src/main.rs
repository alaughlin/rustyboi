mod gameboy;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut gb = gameboy::Gameboy { ..Default::default() };
    gb.power_on();
//    gb.load_game();

//    for _x in 0..20 {
//        gb.step();
//        sleep(Duration::new(1, 0));
//    }
}
