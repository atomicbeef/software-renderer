use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        let r = (self.r as f32 / 255.0) * (rhs.r as f32 / 255.0);
        let g = (self.g as f32 / 255.0) * (rhs.g as f32 / 255.0);
        let b = (self.b as f32 / 255.0) * (rhs.b as f32 / 255.0);

        Self::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}
