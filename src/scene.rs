use minifb::{Key, Window};

use crate::{
    camera::Camera,
    mesh::Mesh,
    vector::{Vec2, Vec3},
    RenderSettings,
};

const CAMERA_MOVEMENT_SPEED: f32 = 3.0;
const CAMERA_LOOK_SENSITIVITY: f32 = 0.025;

pub fn update_scene(
    mesh: &mut Mesh,
    camera: &mut Camera,
    settings: &RenderSettings,
    window: &mut Window,
    elapsed_time: f32,
    delta_time: f32,
    mouse_motion: Vec2,
) {
    // Animate mesh
    mesh.scale = if settings.scale {
        Vec3::splat(1.0) * (2.0 * elapsed_time.sin().abs() + 0.05)
    } else {
        Vec3::splat(1.0)
    };
    mesh.rotation = if settings.rotate {
        mesh.rotation + settings.rotation
    } else {
        mesh.rotation
    };
    mesh.translation.x = if settings.translate {
        2.0 * elapsed_time.sin()
    } else {
        0.0
    };
    mesh.translation.y = if settings.translate {
        2.0 * elapsed_time.cos()
    } else {
        0.0
    };
    mesh.translation.z = if settings.translate {
        5.0 * elapsed_time.sin()
    } else {
        0.0
    };

    // Update camera direction based on input
    camera.yaw -= mouse_motion.x * CAMERA_LOOK_SENSITIVITY;
    camera.pitch += mouse_motion.y * CAMERA_LOOK_SENSITIVITY;

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
    if window.is_key_down(Key::LeftShift) {
        camera_movement_direction.y -= 1.0;
    }

    // Make movement relative to camera direction
    let camera_movement_direction_transformed = camera_movement_direction
        .rotated_y(camera.yaw)
        .rotated_x(camera.pitch)
        .normalized_or_zero();

    camera.translation +=
        camera_movement_direction_transformed * CAMERA_MOVEMENT_SPEED * delta_time;
}
