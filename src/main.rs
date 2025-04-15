#![feature(allocator_api)]

use ctru::{prelude::*, services::gfx::{Flush, Screen, Swap, TopScreen3D}};
use draw::{draw_circle, draw_pixel, draw_rectangle};
use color::Color;
use snake::Game;

mod snake;
mod draw;
mod color;
mod util;

fn main() {
    let apt: Apt = Apt::new().unwrap();
    let mut hid: Hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    //let console: Console = Console::new(gfx.bottom_screen.borrow_mut());

    let mut game: Game = Game::new();

    let mut top_screen = TopScreen3D::from(&gfx.top_screen);

    let mut bottom_screen = gfx.bottom_screen.borrow_mut();

    let font = draw::Font::create_default_font();

    top_screen.set_double_buffering(true);
    bottom_screen.set_double_buffering(true);

    while apt.main_loop() {
        let (mut top_left, mut top_right) = top_screen.split_mut();
        let top_left_buffer = top_left.raw_framebuffer();
        let top_right_buffer = top_right.raw_framebuffer();

        let bottom_buffer = bottom_screen.raw_framebuffer();

        println!("\x1b[0;0HScore: {}", game.get_score());

        for buffer in [&top_left_buffer, &top_right_buffer] {
            draw_rectangle(buffer, 0, 0, 400, 240, color::BLACK);
            game.draw(buffer);
        }


        for x in 0..bottom_buffer.width {
            for y in 0..bottom_buffer.height {
                draw_pixel(&bottom_buffer, x as isize, y as isize, Color{r:y as u8, g:y as u8, b:y as u8});
            }
        }

        draw_circle(&bottom_buffer, 160, 120, 30, color::WHITE);
        font.draw_text(&bottom_buffer, format!("{}", game.get_score()).as_str(), 10, 10, 30);

        hid.scan_input();

        let keys = hid.keys_held();
        game.step(&keys);

        if keys.contains(KeyPad::B) {
            //console.clear();
            game.restart();
        }

        drop(top_left);
        drop(top_right);

        top_screen.flush_buffers();
        top_screen.swap_buffers();

        bottom_screen.flush_buffers();
        bottom_screen.swap_buffers();

        gfx.wait_for_vblank();
    }
}

