extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2::gfx::primitives::DrawRenderer;

mod gameboy;

use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

fn main() {
    let mut gb = gameboy::Gameboy::new();
    gb.power_on();
    gb.load_game();

//    let title = gb.get_game_title();
//    let sdl_context = sdl2::init().unwrap();
//
//    let mut canvas = get_canvas(&sdl_context, title).unwrap();
//
//    canvas.clear();
//    canvas.present();
//
//    let mut events = sdl_context.event_pump().unwrap();
//
//    let mut screen_data = Vec::new();
//    for i in 0..23041 {
//        screen_data.push(0xFFFFFFFF);
//    }
//
//    let mut x = 0;
//    let mut y = 0;
//    for pixel in screen_data {
//        canvas.pixel(x as i16, y as i16, pixel as u32).unwrap();
//        if x > 159 {
//            x = 0;
//            y += 1;
//            continue
//        }
//
//        x += 1;
//    }
//
//    canvas.present();
//
//    'main: loop {
//        for event in events.poll_iter() {
//
//            match event {
//                Event::Quit {..} => {
//                    break 'main
//                },
//                Event::KeyDown {keycode: Some(keycode), ..} => {
//                    if keycode == Keycode::Escape {
//                        break 'main
//                    }
//                },
//                _ => {}
//            }
//        }
//    }

    for _x in 0..10 {
        gb.step();
        sleep(Duration::new(1, 0));
    }

    gb.print_registers();
}

fn get_canvas(context: &sdl2::Sdl, title: &str) -> Result<sdl2::render::WindowCanvas, sdl2::IntegerOrSdlError> {
    let video_subsys = context.video().unwrap();
    let window = video_subsys.window(title, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    window.into_canvas().build()
}
