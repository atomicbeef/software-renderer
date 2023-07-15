use crate::color::Color;
use crate::texture::Tex2;
use crate::vector::Vec4;

pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub a_uv: u16,
    pub b_uv: u16,
    pub c_uv: u16,
    pub color: Color,
}

impl Face {
    pub const fn new(a: u16, b: u16, c: u16, a_uv: u16, b_uv: u16, c_uv: u16, color: Color) -> Self {
        Self { a, b, c, a_uv, b_uv, c_uv, color }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec4,
    pub uv: Tex2,
}

#[derive(Debug)]
pub struct Triangle {
    pub points: [Vec4; 3],
    pub tex_coords: [Tex2; 3],
    pub depth: f32,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: Vec4, b: Vec4, c: Vec4, a_uv: Tex2, b_uv: Tex2, c_uv: Tex2, depth: f32, color: Color) -> Self {
        Self { points: [a, b, c], tex_coords: [a_uv, b_uv, c_uv], depth, color }
    }
}