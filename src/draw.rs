use std::{collections::HashMap, ops::Index};

use crate::{util::distance_squared, Color};
use ctru::services::gfx::RawFrameBuffer;

pub fn draw_rectangle(
    fb: &RawFrameBuffer,
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    color: Color,
) {
    for cx in x..x+w {
        for cy in y..y+h {
            draw_pixel(fb, cx, cy, color);
        }
    }
}

pub fn draw_circle(fb: &RawFrameBuffer, x: isize, y: isize, r: isize, color: Color) {
    for cx in x-r..=x+r {
        for cy in y-r..=y+r {
            if distance_squared(cx, cy, x, y) < r*r {
                draw_pixel(fb, cx, cy, color);
            }
        }
    }
}

pub fn draw_pixel(fb: &RawFrameBuffer, x: isize, y: isize, color: Color) {
    // The screens are rotated 90 degrees, so the width and height of the
    // framebuffer do not match up with the expectation. I'm switching
    // them around here so these terms match up with expectations.
    let height: isize = fb.width as isize;
    let width: isize = fb.height as isize;

    if x < 0 || x >= width || y < 0 || y >= height {
        return;
    }

    let cy = (height - 1) - y;

    let offset: isize = (cy * 3) + (x * height * 3);
    unsafe {
        *(fb.ptr.offset(offset + 0)) = color.b;
        *(fb.ptr.offset(offset + 1)) = color.g;
        *(fb.ptr.offset(offset + 2)) = color.r;
    }
}

pub struct Sprite {
    width: isize,
    height: isize,
    data: Box<[Color]>
}

impl Sprite {
    pub fn new(width: isize, height: isize, rawdata: &'static[u8]) -> Sprite {
        let datavec: Vec<Color> = rawdata.chunks(3).map(|p| Color{r:p[0], g:p[1], b:p[2]}).collect();
        let data = datavec.into_boxed_slice();

        Sprite {
            width,
            height,
            data
        }
    }

    pub fn draw(&self, fb: &RawFrameBuffer, x: isize, y: isize) {
        let mut cx: isize = 0;
        let mut cy: isize = 0;
        for pixel in &self.data {
            draw_pixel(fb, x + cx, y + cy, *pixel);
            cx += 1;
            if cx == self.width as isize {
                cy += 1;
                cx = 0;
            }
        }
    }
}

pub struct Font {
    spacing: isize,
    line_height: isize,
    glyphs: HashMap<char, Sprite>
}

impl Font {
    pub fn create_default_font() -> Font {
        let mut glyphs: HashMap<char, Sprite> = HashMap::new();
        glyphs.insert('0', Sprite::new(3, 5, include_bytes!("assets/0.rgb")));
        glyphs.insert('1', Sprite::new(3, 5, include_bytes!("assets/1.rgb")));
        glyphs.insert('2', Sprite::new(3, 5, include_bytes!("assets/2.rgb")));
        glyphs.insert('3', Sprite::new(3, 5, include_bytes!("assets/3.rgb")));
        glyphs.insert('4', Sprite::new(3, 5, include_bytes!("assets/4.rgb")));
        glyphs.insert('5', Sprite::new(3, 5, include_bytes!("assets/5.rgb")));
        glyphs.insert('6', Sprite::new(3, 5, include_bytes!("assets/6.rgb")));
        glyphs.insert('7', Sprite::new(3, 5, include_bytes!("assets/7.rgb")));
        glyphs.insert('8', Sprite::new(3, 5, include_bytes!("assets/8.rgb")));
        glyphs.insert('9', Sprite::new(3, 5, include_bytes!("assets/9.rgb")));

        Font {
            spacing: 1,
            line_height: 6,
            glyphs
        }
    }

    pub fn draw_text(&self, fb: &RawFrameBuffer, text: &str, x: isize, y: isize, w: isize) {
        let mut cx: isize = 0;
        let mut cy: isize = 0;

        for chr in text.chars() {
            let sprite = &self.glyphs[&chr];
            if cx + sprite.width > w {
                cx = 0;
                cy += self.line_height;
            }
            sprite.draw(fb, x + cx, y + cy);
            cx += sprite.width + self.spacing;
        }
    }
}
