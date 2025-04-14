#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub static RED:     Color = Color{ r: 255, g: 0,   b: 0,   };
pub static GREEN:   Color = Color{ r: 0,   g: 0,   b: 255, };
pub static BLUE:    Color = Color{ r: 0,   g: 0,   b: 255, };
pub static CYAN:    Color = Color{ r: 0,   g: 255, b: 255, };
pub static MAGENTA: Color = Color{ r: 255, g: 0,   b: 255, };
pub static YELLOW:  Color = Color{ r: 255, g: 255, b: 0,   };
pub static WHITE:   Color = Color{ r: 255, g: 255, b: 255, };
pub static BLACK:   Color = Color{ r: 0,   g: 0,   b: 0,   };

pub static ORANGE:  Color = Color{ r: 255, g: 127, b: 0,   };

pub static RAINBOW: &[Color] = &[
    Color { r: 255, g: 0, b: 0 },
    Color { r: 255, g: 127, b: 0 },
    Color { r: 255, g: 255, b: 0 },
    Color { r: 127, g: 255, b: 0 },
    Color { r: 0, g: 255, b: 0 },
    Color { r: 0, g: 255, b: 127 },
    Color { r: 0, g: 255, b: 255 },
    Color { r: 0, g: 255, b: 255 },
    Color { r: 0, g: 127, b: 255 },
    Color { r: 0, g: 0, b: 255 },
    Color { r: 127, g: 0, b: 255 },
    Color { r: 255, g: 0, b: 255 },
];
