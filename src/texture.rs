use std::borrow::Cow;
use std::ops::{Add, Mul, Div};
use std::path::Path;
use std::fs::File;

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

impl Div<f32> for Tex2 {
    type Output = Tex2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            u: self.u / rhs,
            v: self.v / rhs
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextureError<'a> {
    ReadError(Cow<'a, str>),
    DecodeError,
    UnsupportedBitDepth,
    UnsupportedColorType,
}

impl std::fmt::Display for TextureError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadError(path) => {
                write!(f, "could not open PNG texture for reading at {path}",)
            },
            Self::DecodeError => { write!(f, "could not decode PNG texture") },
            Self::UnsupportedBitDepth => { write!(f, "unsupported PNG texture bit depth") },
            Self::UnsupportedColorType => { write!(f, "unsupported PNG color type") },
        }
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Texture {
    pub fn from_color(width: u32, height: u32, color: Color) -> Self {
        Self { width, height, pixels: vec![color; width as usize * height as usize] }
    }

    pub fn grid(width: u32, height: u32, fill_color: Color, line_color: Color) -> Self {
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

    pub fn from_png(path: &Path) -> Result<Self, TextureError> {
        let png_file = File::open(path).or_else(|_| Err(TextureError::ReadError(path.to_string_lossy())))?;

        let decoder = png::Decoder::new(png_file);
        let mut reader = decoder.read_info().or(Err(TextureError::DecodeError))?;
        
        let mut byte_buffer: Vec<u8> = vec![0; reader.output_buffer_size()];
        let frame_metadata = reader.next_frame(&mut byte_buffer).or(Err(TextureError::DecodeError))?;

        if !matches!(frame_metadata.bit_depth, png::BitDepth::Eight) {
            return Err(TextureError::UnsupportedBitDepth);
        }

        let pixels = match frame_metadata.color_type {
            png::ColorType::Rgba => {
                byte_buffer.chunks_exact(4).map(|colors| Color::new(
                    colors[0],
                    colors[1],
                    colors[2]
                )).collect()
            },
            png::ColorType::Rgb => {
                byte_buffer.chunks_exact(3).map(|colors| Color::new(
                    colors[0],
                    colors[1],
                    colors[2]
                )).collect()
            },
            _ => { return Err(TextureError::UnsupportedColorType); }
        };

        Ok(Self {
            width: frame_metadata.width,
            height: frame_metadata.height,
            pixels
        })
    }

    pub fn sample(&self, pos: Tex2) -> Color {
        let col = ((self.width - 1) as f32 * pos.u) as usize;
        let row = ((self.height - 1) as f32 * pos.v) as usize;
        let index = row * self.height as usize + col;
        
        self.pixels[index]
    }
}