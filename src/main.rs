use minifb::{Key, Window, WindowOptions};

mod color_buffer;
mod drawing;
mod vector;

use color_buffer::ColorBuffer;
use vector::{Vec2, Vec3};

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

fn project_point(point3d: Vec3, fov_factor: f32) -> Vec2 {
    Vec2::new(point3d.x * fov_factor / point3d.z, point3d.y * fov_factor / point3d.z)
}

fn update(cube_points: &Vec<Vec3>, projected_points: &mut Vec<Vec2>, fov_factor: f32, camera_position: &Vec3) {
    for i in 0..cube_points.len() {
        let mut point = cube_points[i];
        point.z -= camera_position.z;

        projected_points[i] = project_point(point, fov_factor);
    }
}

fn render(buffer: &mut ColorBuffer, window: &mut Window, projected_points: &Vec<Vec2>) {
    buffer.draw_grid();

    for point in projected_points {
        if point.x == f32::NEG_INFINITY || point.x == f32::INFINITY || point.y == f32::NEG_INFINITY || point.y == f32::INFINITY {
            continue;
        }

        buffer.draw_rect(
            (point.x + (WINDOW_WIDTH / 2) as f32) as usize,
            (point.y + (WINDOW_HEIGHT / 2) as f32) as usize,
            4,
            4,
            0x0000FF00
        );
    }

    window.update_with_buffer(buffer.buffer(), buffer.width(), buffer.height())
        .unwrap();
    
    buffer.clear(0x00000000);
}

fn main() {
    let mut buffer = ColorBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut window = Window::new(
        "3D Renderer",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default()
    )
    .expect("Error: Window could not be created!");

    let mut cube_points: Vec<Vec3> = Vec::with_capacity(9 * 9 * 9);
    for x in (-100i8..125).step_by(25) {
        let x = f32::from(x) / 100.0;
        for y in (-100i8..125).step_by(25) {
            let y = f32::from(y) / 100.0;
            for z in (-100i8..125).step_by(25) {
                let z = f32::from(z) / 100.0;

                cube_points.push(Vec3::new(x, y, z));
            }
        }
    }

    let mut projected_points: Vec<Vec2> = vec![Vec2::new(0.0, 0.0); 9 * 9* 9];
    let fov_factor = 684f32;
    
    let camera_position = Vec3::new(0.0, 0.0, -5.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&cube_points, &mut projected_points, fov_factor, &camera_position);
        render(&mut buffer, &mut window, &projected_points);
    }
}
