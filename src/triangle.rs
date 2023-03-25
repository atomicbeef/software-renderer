use crate::vector::Vec2;

pub struct Face {
    pub a: u16,
    pub b: u16,
    pub c: u16
}

impl Face {
    pub const fn new(a: u16, b: u16, c: u16) -> Self {
        Self { a, b, c }
    }
}

pub struct Triangle {
    pub points: [Vec2; 3],
    pub depth: f32,
}

impl Triangle {
    pub fn new(a: Vec2, b: Vec2, c: Vec2, depth: f32) -> Self {
        Self { points: [a, b, c], depth }
    }
}