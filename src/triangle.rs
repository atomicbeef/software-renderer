use crate::color::Color;
use crate::fixed::FixedI32;
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
    pub const fn new(
        a: u16,
        b: u16,
        c: u16,
        a_uv: u16,
        b_uv: u16,
        c_uv: u16,
        color: Color,
    ) -> Self {
        Self {
            a,
            b,
            c,
            a_uv,
            b_uv,
            c_uv,
            color,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexPos {
    pub x: FixedI32,
    pub y: FixedI32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: VertexPos,
    pub uv: Tex2,
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub points: [Vec4; 3],
    pub tex_coords: [Tex2; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(
        a: Vec4,
        b: Vec4,
        c: Vec4,
        a_uv: Tex2,
        b_uv: Tex2,
        c_uv: Tex2,
        color: Color,
    ) -> Self {
        Self {
            points: [a, b, c],
            tex_coords: [a_uv, b_uv, c_uv],
            color,
        }
    }

    pub fn bounding_box(&self) -> (u16, u16, u16, u16) {
        let a = self.points[0];
        let b = self.points[1];
        let c = self.points[2];

        let min_x = a.x.min(b.x.min(c.x));
        let min_y = a.y.min(b.y.min(c.y));

        let max_x = a.x.max(b.x.max(c.x));
        let max_y = a.y.max(b.y.max(c.y));

        (
            min_x.floor() as u16,
            min_y.floor() as u16,
            max_x.ceil() as u16,
            max_y.ceil() as u16,
        )
    }
}
