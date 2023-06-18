use std::ops::Mul;

use crate::vector::Vec4;

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    x: Vec4,
    y: Vec4,
    z: Vec4,
    w: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Self = Self {
        x: Vec4::new(1.0, 0.0, 0.0, 0.0),
        y: Vec4::new(0.0, 1.0, 0.0, 0.0),
        z: Vec4::new(0.0, 0.0, 1.0, 0.0),
        w: Vec4::new(0.0, 0.0, 0.0, 1.0)
    };

    pub const fn new(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { x, y, z, w }
    }

    pub fn new_scale(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.x = x;
        matrix.y.y = y;
        matrix.z.z = z;

        matrix
    }

    pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.w = x;
        matrix.y.w = y;
        matrix.z.w = z;

        matrix
    }

    pub fn new_rotation_x(angle: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.y.y = angle.cos();
        matrix.y.z = -angle.sin();
        matrix.z.y = angle.sin();
        matrix.z.z = angle.cos();

        matrix
    }

    pub fn new_rotation_y(angle: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.x = angle.cos();
        matrix.x.z = angle.sin();
        matrix.z.x = -angle.sin();
        matrix.z.z = angle.cos();

        matrix
    }

    pub fn new_rotation_z(angle: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.x = angle.cos();
        matrix.x.y = -angle.sin();
        matrix.y.x = angle.sin();
        matrix.y.y = angle.cos();

        matrix
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs),
            Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs),
            Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs),
            Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs)
        )
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        Mat4::new(
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.x),
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.y),
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.z),
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.w)
            ),
            Vec4::new(
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.x),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.y),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.z),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.w)
            ),
            Vec4::new(
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.x),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.y),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.z),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.w)
            ),
            Vec4::new(
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.x),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.y),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.z),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.w)
            ),
        )
    }
}