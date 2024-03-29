use crate::{
    matrix::Mat4,
    vector::{Vec3, Vec4},
};

pub struct Camera {
    pub translation: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new(pos: Vec3, up: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            translation: pos,
            up,
            yaw,
            pitch,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        let z = Vec3::new(0.0, 0.0, 1.0)
            .rotated_y(self.yaw)
            .rotated_x(self.pitch)
            .normalized();
        let x = self.up.cross(z).normalized();
        let y = z.cross(x).normalized();

        Mat4::new(
            Vec4::new(x.x, y.x, z.x, 0.0),
            Vec4::new(x.y, y.y, z.y, 0.0),
            Vec4::new(x.z, y.z, z.z, 0.0),
            Vec4::new(
                -x.dot(self.translation),
                -y.dot(self.translation),
                -z.dot(self.translation),
                1.0,
            ),
        )
    }
}
