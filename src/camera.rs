use crate::{vector::{Vec3, Vec4}, matrix::Mat4};

pub struct Camera {
    pub translation: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, direction: Vec3, up: Vec3) -> Self {
        Self { translation: pos, direction, up }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let z = self.direction.normalized();
        let x = self.up.cross(z).normalized();
        let y = z.cross(x).normalized();

        Mat4::new(
            Vec4::new(x.x, y.x, z.x, 0.0),
            Vec4::new(x.y, y.y, z.y, 0.0),
            Vec4::new(x.z, y.z, z.z, 0.0),
            Vec4::new(-x.dot(self.translation), -y.dot(self.translation), -z.dot(self.translation), 1.0)
        )
    }
}