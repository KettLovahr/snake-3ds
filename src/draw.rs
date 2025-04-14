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

    if x < 0 || x > width || y < 0 || y > height {
        return;
    }

    let offset: isize = (y * 3) + (x * height * 3);
    unsafe {
        *(fb.ptr.offset(offset + 0)) = color.b;
        *(fb.ptr.offset(offset + 1)) = color.g;
        *(fb.ptr.offset(offset + 2)) = color.r;
    }
}
