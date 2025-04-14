#![feature(allocator_api)]

use ctru::{prelude::*, services::gfx::{Screen, Swap, TopScreen3D}};
use draw::draw_rectangle;
use color::Color;
use snake::Game;

mod snake;
mod draw;
mod color;

fn main() {
    let apt: Apt = Apt::new().unwrap();
    let mut hid: Hid = Hid::new().unwrap();
    let gfx = Gfx::new().unwrap();
    let console: Console = Console::new(gfx.bottom_screen.borrow_mut());

    let mut game: Game = Game::new();

    let mut top_screen = TopScreen3D::from(&gfx.top_screen);

    top_screen.set_double_buffering(true);

    while apt.main_loop() {
        let (mut top_left, mut top_right) = top_screen.split_mut();
        let top_left_buffer = top_left.raw_framebuffer();
        let top_right_buffer = top_right.raw_framebuffer();

        println!("\x1b[0;0HScore: {}", game.get_score());

        for buffer in [&top_left_buffer, &top_right_buffer] {
            draw_rectangle(buffer, 0, 0, 400, 240, color::BLACK);
            game.draw(buffer);
        }

        hid.scan_input();

        let keys = hid.keys_held();
        game.step(&keys);

        if keys.contains(KeyPad::B) {
            console.clear();
            game.restart();
        }

        drop(top_left);
        drop(top_right);

        top_screen.swap_buffers();
        gfx.wait_for_vblank();
    }
}

