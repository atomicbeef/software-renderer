use crate::matrix::Mat4;
use crate::vector::{Vec3, Vec4};

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }
}

pub struct ClippingPlanes {
    pub right: Plane,
    pub left: Plane,
    pub top: Plane,
    pub bottom: Plane,
    pub far: Plane,
    pub near: Plane,
}

pub struct Camera {
    pub translation: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn new(
        translation: Vec3,
        up: Vec3,
        yaw: f32,
        pitch: f32,
        fov: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            translation,
            up,
            yaw,
            pitch,
            fov,
            z_near,
            z_far,
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
