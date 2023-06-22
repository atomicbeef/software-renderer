#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32)
    }
}