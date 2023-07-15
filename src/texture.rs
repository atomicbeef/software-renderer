use std::ops::{Add, Mul};

use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Tex2 {
    pub u: f32,
    pub v: f32,
}

impl Tex2 {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

impl Add for Tex2 {
    type Output = Tex2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        }
    }
}

impl Mul<f32> for Tex2 {
    type Output = Tex2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            u: self.u * rhs,
            v: self.v * rhs,
        }
    }
}

pub struct Texture {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<Color>,
}

impl Texture {
    pub fn grid(width: u16, height: u16, fill_color: Color, line_color: Color) -> Self {
        let mut pixels = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                if x % 10 == 0 || y % 10 == 0 {
                    pixels.push(line_color);
                } else {
                    pixels.push(fill_color);
                }
            }
        }
        
        Self { width, height, pixels }
    }

    pub fn sample(&self, pos: Tex2) -> Color {
        let col = ((self.width - 1) as f32 * pos.u) as usize;
        let row = ((self.height - 1) as f32 * pos.v) as usize;
        let index = row * self.height as usize + col;
        
        self.pixels[index]
    }
}