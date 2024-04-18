use minifb::{Key, KeyRepeat, Window};

use crate::{camera::Camera, mesh::Mesh, texture::Texture, vector::Vec3, RenderSettings};

const CAMERA_MOVEMENT_SPEED: f32 = 3.0;
const CAMERA_LOOK_SENSITIVITY: f32 = 0.025;

pub struct Object {
    pub mesh: Mesh,
    pub texture: Texture,
}

pub struct Scene {
    objects: Vec<Object>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        Self {
            objects: Vec::new(),
            camera,
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn update(
        &mut self,
        settings: &RenderSettings,
        window: &mut Window,
        elapsed_time: f32,
        delta_time: f32,
    ) {
        // Animate objects
        for object in self.objects.iter_mut() {
            object.mesh.scale = if settings.scale {
                Vec3::splat(1.0) * (2.0 * elapsed_time.sin().abs() + 0.05)
            } else {
                Vec3::splat(1.0)
            };

            object.mesh.rotation = if settings.rotate {
                object.mesh.rotation + settings.rotation
            } else {
                object.mesh.rotation
            };

            object.mesh.translation.x = if settings.translate {
                2.0 * elapsed_time.sin()
            } else {
                0.0
            };

            object.mesh.translation.y = if settings.translate {
                2.0 * elapsed_time.cos()
            } else {
                0.0
            };

            object.mesh.translation.z = if settings.translate {
                5.0 * elapsed_time.sin()
            } else {
                0.0
            };

            if window.is_key_pressed(Key::P, KeyRepeat::No) {
                object.mesh.rotation = Vec3::default();
            }
        }

        // Update camera direction based on input
        if window.is_key_down(Key::Left) {
            self.camera.yaw -= CAMERA_LOOK_SENSITIVITY;
        }
        if window.is_key_down(Key::Right) {
            self.camera.yaw += CAMERA_LOOK_SENSITIVITY;
        }
        if window.is_key_down(Key::Up) {
            self.camera.pitch -= CAMERA_LOOK_SENSITIVITY;
        }
        if window.is_key_down(Key::Down) {
            self.camera.pitch += CAMERA_LOOK_SENSITIVITY;
        }

        // Update camera translation based on input
        let mut camera_movement_direction = Vec3::default();
        if window.is_key_down(Key::W) {
            camera_movement_direction.z += 1.0;
        }
        if window.is_key_down(Key::S) {
            camera_movement_direction.z -= 1.0;
        }
        if window.is_key_down(Key::D) {
            camera_movement_direction.x += 1.0;
        }
        if window.is_key_down(Key::A) {
            camera_movement_direction.x -= 1.0;
        }
        if window.is_key_down(Key::Space) {
            camera_movement_direction.y += 1.0;
        }
        if window.is_key_down(Key::LeftCtrl) {
            camera_movement_direction.y -= 1.0;
        }

        // Make movement relative to camera direction
        let camera_movement_direction_transformed = camera_movement_direction
            .rotated_x(self.camera.pitch)
            .rotated_y(self.camera.yaw)
            .normalized_or_zero();

        self.camera.translation +=
            camera_movement_direction_transformed * CAMERA_MOVEMENT_SPEED * delta_time;
    }
}
