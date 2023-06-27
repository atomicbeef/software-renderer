use std::ops::{Mul, MulAssign};

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

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.x = x;
        matrix.y.y = y;
        matrix.z.z = z;

        matrix
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.w.x = x;
        matrix.w.y = y;
        matrix.w.z = z;

        matrix
    }

    pub fn rotation_x(angle: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.y.y = angle.cos();
        matrix.y.z = angle.sin();
        matrix.z.y = -angle.sin();
        matrix.z.z = angle.cos();

        matrix
    }

    pub fn rotation_y(angle: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.x = angle.cos();
        matrix.x.z = -angle.sin();
        matrix.z.x = angle.sin();
        matrix.z.z = angle.cos();

        matrix
    }

    pub fn rotation_z(angle: f32) -> Self {
        let mut matrix = Self::IDENTITY;

        matrix.x.x = angle.cos();
        matrix.x.y = angle.sin();
        matrix.y.x = -angle.sin();
        matrix.y.y = angle.cos();

        matrix
    }

    pub fn projection(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        Self::new(
            Vec4::new(aspect * (fov / 2.0).atan(), 0.0, 0.0, 0.0),
            Vec4::new(0.0, (fov / 2.0).atan(), 0.0, 0.0),
            Vec4::new(0.0, 0.0, zfar / (zfar - znear), 1.0),
            Vec4::new(0.0, 0.0, -zfar * znear / (zfar - znear), 0.0)
        )
    }

    pub fn project_vec4(&self, vector: Vec4) -> Vec4 {
        let mut multiplied = self * vector;
        
        if multiplied.w != 0.0 {
            multiplied.x /= multiplied.w;
            multiplied.y /= multiplied.w;
            multiplied.z /= multiplied.w;
        }

        multiplied
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

impl Mul<Vec4> for &Mat4 {
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
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.x),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.x),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.x)
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.y),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.y),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.y),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.y)
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.z),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.z),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.z),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.z)
            ),
            Vec4::new(
                Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x).dot(rhs.w),
                Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y).dot(rhs.w),
                Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z).dot(rhs.w),
                Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w).dot(rhs.w)
            ),
        )
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vec4;

    use super::Mat4;

    fn eq_f32(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }

    fn eq_vec4(a: Vec4, b: Vec4) -> bool {
        eq_f32(a.x, b.x) && eq_f32(a.y, b.y) && eq_f32(a.z, b.z) && eq_f32(a.w, b.w)
    }

    fn eq_mat4(a: Mat4, b: Mat4) -> bool {
        eq_vec4(a.x, b.x) && eq_vec4(a.y, b.y) && eq_vec4(a.z, b.z) && eq_vec4(a.w, b.w)
    }

    #[test]
    fn multiply_mat4_vec4() {
        let a = Mat4::new(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0)
        );
        let b = Vec4::new(5.0, 1.0, 2.0, 0.0);
        let c = a * b;

        assert!(eq_vec4(c, Vec4::new(28.0, 36.0, 44.0, 52.0)))
    }

    #[test]
    fn scale_vec4() {
        let a = Mat4::scale(5.0, 0.0, 3.0);
        let b = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let c = a * b;

        assert!(eq_vec4(c, Vec4::new(5.0, 0.0, 9.0, 1.0)));
    }

    #[test]
    fn multiply_mat4_mat4() {
        let a = Mat4::translation(1.0, 0.0, 5.0);
        let b = Mat4::scale(1.0, 2.0, 3.0);
        let c = a * b;

        println!("{c:?}");

        assert!(eq_mat4(c, Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 3.0, 0.0),
            Vec4::new(1.0, 0.0, 5.0, 1.0)
        )));
    }

    #[test]
    fn rotate_y_matrix() {
        let a = Mat4::rotation_y(std::f32::consts::FRAC_PI_4);

        println!("{a:?}");
    }
}