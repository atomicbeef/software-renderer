use std::ops::{Add, Sub};

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
pub struct RasterPoint {
    pub x: i32,
    pub y: i32,
}

impl RasterPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn cross(&self, b: Self) -> i32 {
        self.x * b.y - self.y * b.x
    }

    pub fn edge_weight(&self, a: Self, b: Self, bias: i32) -> i32 {
        let ab = b - a;
        let ap = *self - a;
        ab.cross(ap) + bias
    }

    /// Returns 0 if an edge is flat top or left, otherwise returns -1
    pub fn edge_orientation(a: Self, b: Self) -> i32 {
        let is_flat_top = b.y - a.y == 0 && b.x - a.x > 0;
        let is_left = b.y - a.y < 0;

        if is_flat_top || is_left {
            0
        } else {
            -1
        }
    }
}

impl Add<RasterPoint> for RasterPoint {
    type Output = RasterPoint;

    fn add(self, rhs: RasterPoint) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<RasterPoint> for RasterPoint {
    type Output = RasterPoint;

    fn sub(self, rhs: RasterPoint) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexPos {
    pub x: i32,
    pub y: i32,
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
