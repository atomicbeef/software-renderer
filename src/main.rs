use minifb::{Key, Window, WindowOptions};

mod color_buffer;
mod drawing;
mod vector;

use color_buffer::ColorBuffer;
use vector::{Vec2, Vec3};

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

fn project_point_orthographic(point3d: Vec3, fov_factor: f32) -> Vec2 {
    Vec2::new(point3d.x * fov_factor, point3d.y * fov_factor)
}

fn update(cube_points: &Vec<Vec3>, projected_points: &mut Vec<Vec2>, fov_factor: f32) {
    for i in 0..cube_points.len() {
        projected_points[i] = project_point_orthographic(cube_points[i], fov_factor);
    }
}

fn render(buffer: &mut ColorBuffer, window: &mut Window, projected_points: &Vec<Vec2>) {
    buffer.draw_grid();

    for point in projected_points {
        buffer.draw_rect(
            point.x as usize + WINDOW_WIDTH / 2,
            point.y as usize + WINDOW_HEIGHT / 2,
            4,
            4,
            0x00FFFF00
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
    for x in (-100i8..100).step_by(25) {
        let x = f32::from(x) / 100.0;
        for y in (-100i8..100).step_by(25) {
            let y = f32::from(y) / 100.0;
            for z in (-100i8..100).step_by(25) {
                let z = f32::from(z) / 100.0;

                cube_points.push(Vec3::new(x, y, z));
            }
        }
    }

    let mut projected_points: Vec<Vec2> = vec![Vec2::new(0.0, 0.0); 9 * 9* 9];
    let fov_factor = 128f32;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&cube_points, &mut projected_points, fov_factor);
        render(&mut buffer, &mut window, &projected_points);
    }
}
