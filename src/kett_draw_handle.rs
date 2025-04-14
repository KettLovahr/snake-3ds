use crate::Color;
use ctru::services::gfx::RawFrameBuffer;

pub fn draw_rectangle(
    framebuffer: &RawFrameBuffer,
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    color: Color,
) {
    for cx in x..(x + w) {
        for cy in (y * 3)..(y + h) * 3 {
            unsafe {
                let val: u8 = match cy % 3 {
                    // Pixels are stored in BGR8 format
                    0 => color.b,
                    1 => color.g,
                    2 => color.r,
                    _ => 0,
                };

                // The 3DS displays have a height of 240 pixels
                *(framebuffer.ptr.offset(cy + (cx * 240 * 3))) = val;
            }
        }
    }
}
