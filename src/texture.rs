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

pub struct Texture {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<Color>,
}

impl Texture {
    pub fn from_color(width: u16, height: u16, fill_color: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![fill_color; width as usize * height as usize]
        }
    }

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
        let col = (self.width as f32 * pos.u).floor() as usize;
        let row = (self.height as f32 * pos.v).floor() as usize;
        let index = row * self.height as usize + col;
        
        self.pixels[index]
    }
}