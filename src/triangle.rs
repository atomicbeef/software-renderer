use crate::color::Color;
use crate::texture::Tex2;
use crate::vector::{Vec2, Vec4};

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
pub struct Vertex {
    pub pos: Vec4,
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

    pub fn bounding_box(&self) -> (f32, f32, f32, f32) {
        let a = self.points[0];
        let b = self.points[1];
        let c = self.points[2];

        let min_x = a.x.min(b.x.min(c.x));
        let min_y = a.y.min(b.y.min(c.y));

        let max_x = a.x.max(b.x.max(c.x));
        let max_y = a.y.max(b.y.max(c.y));

        (min_x, min_y, max_x, max_y)
    }

    fn point_inside_edge(a: Vec2, b: Vec2, p: Vec2) -> bool {
        let edge = b - a;
        edge.cross(p - a) >= 0.0
    }

    pub fn point_inside(&self, p: Vec2) -> bool {
        let a = Vec2::from(self.points[0]);
        let b = Vec2::from(self.points[1]);
        let c = Vec2::from(self.points[2]);

        Self::point_inside_edge(a, b, p)
            && Self::point_inside_edge(b, c, p)
            && Self::point_inside_edge(c, a, p)
    }
}
