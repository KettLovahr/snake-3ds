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
    if x < 0 || x > 400 || y < 0 || y > 240 {
        return;
    }

    let offset: isize = (y * 3) + (x * 240 * 3);
    unsafe {
        *(fb.ptr.offset(offset + 0)) = color.b;
        *(fb.ptr.offset(offset + 1)) = color.g;
        *(fb.ptr.offset(offset + 2)) = color.r;
    }
}
