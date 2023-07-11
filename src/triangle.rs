use crate::color::Color;
use crate::texture::Tex2;
use crate::vector::Vec2;

pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub a_uv: Tex2,
    pub b_uv: Tex2,
    pub c_uv: Tex2,
    pub color: Color,
}

impl Face {
    pub const fn new(a: u16, b: u16, c: u16, a_uv: Tex2, b_uv: Tex2, c_uv: Tex2, color: Color) -> Self {
        Self { a, b, c, a_uv, b_uv, c_uv, color }
    }
}

#[derive(Debug)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub tex_coords: [Tex2; 3],
    pub depth: f32,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, a_uv: Tex2, b_uv: Tex2, c_uv: Tex2, depth: f32, color: Color) -> Self {
        Self { points: [a, b, c], tex_coords: [a_uv, b_uv, c_uv], depth, color }
    }
}