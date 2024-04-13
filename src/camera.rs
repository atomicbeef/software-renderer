use crate::matrix::Mat4;
use crate::plane::Plane;
use crate::vector::{Vec3, Vec4};

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

    pub fn clipping_planes(&self, horizontal_aspect_ratio: f32) -> ClippingPlanes {
        let half_fov_vertical = self.fov / 2.0;
        let half_fov_horizontal = 2.0 * (half_fov_vertical.tan() * horizontal_aspect_ratio).atan();

        ClippingPlanes {
            right: Plane::new(
                Vec3::default(),
                Vec3::new(-half_fov_horizontal.cos(), 0.0, half_fov_horizontal.sin()),
            ),
            left: Plane::new(
                Vec3::default(),
                Vec3::new(half_fov_horizontal.cos(), 0.0, half_fov_horizontal.sin()),
            ),
            top: Plane::new(
                Vec3::default(),
                Vec3::new(0.0, -half_fov_vertical.cos(), half_fov_vertical.sin()),
            ),
            bottom: Plane::new(
                Vec3::default(),
                Vec3::new(0.0, half_fov_vertical.cos(), half_fov_vertical.sin()),
            ),
            far: Plane::new(Vec3::new(0.0, 0.0, self.z_far), Vec3::new(0.0, 0.0, -1.0)),
            near: Plane::new(Vec3::new(0.0, 0.0, self.z_near), Vec3::new(0.0, 0.0, 1.0)),
        }
    }
}
