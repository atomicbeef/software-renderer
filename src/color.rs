use std::ops::Mul;

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

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            (self.r as f32 * rhs) as u8,
            (self.g as f32 * rhs) as u8,
            (self.b as f32 * rhs) as u8,
        )
    }
}