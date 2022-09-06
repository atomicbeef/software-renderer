use minifb::{Key, Window, WindowOptions};

mod color_buffer;
mod drawing;
mod mesh;
mod triangle;
mod vector;

use color_buffer::ColorBuffer;
use mesh::{MESH_FACES, MESH_VERTICES};
use triangle::Triangle;
use vector::{Vec2, Vec3};

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

fn project_point(point3d: &Vec3, fov_factor: f32) -> Vec2 {
    Vec2::new(point3d.x * fov_factor / point3d.z, point3d.y * fov_factor / point3d.z)
}

fn update(
    triangles_to_render: &mut Vec<Triangle>,
    fov_factor: f32,
    camera_position: &Vec3,
    cube_rotation: &mut Vec3
) {
    triangles_to_render.clear();

    cube_rotation.x += 0.01;
    cube_rotation.y += 0.01;
    cube_rotation.z += 0.01;

    for face in MESH_FACES {
        let face_vertices = [
            MESH_VERTICES[face.a as usize],
            MESH_VERTICES[face.b as usize],
            MESH_VERTICES[face.c as usize]
        ];

        let mut triangle = Triangle::new(Vec2::default(), Vec2::default(), Vec2::default());

        for (i, vertex) in face_vertices.iter().enumerate() {
            let transformed_vertex = vertex.rotated_x(cube_rotation.x);
            let transformed_vertex = transformed_vertex.rotated_y(cube_rotation.y);
            let mut transformed_vertex = transformed_vertex.rotated_z(cube_rotation.z);

            transformed_vertex.z -= camera_position.z;

            let mut projected_vertex = project_point(&transformed_vertex, fov_factor);

            projected_vertex.x += (WINDOW_WIDTH / 2) as f32;
            projected_vertex.y += (WINDOW_HEIGHT / 2) as f32;

            triangle.points[i] = projected_vertex;
        }

        triangles_to_render.push(triangle);
    }
}

fn render(buffer: &mut ColorBuffer, window: &mut Window, triangles_to_render: &Vec<Triangle>) {
    buffer.draw_grid();

    for triangle in triangles_to_render.iter() {
        for point in triangle.points {
            if point.x == f32::NEG_INFINITY || point.x == f32::INFINITY || point.y == f32::NEG_INFINITY || point.y == f32::INFINITY {
                continue;
            }
    
            buffer.draw_rect(
                point.x as usize,
                point.y as usize,
                4,
                4,
                0x0000FF00
            );
        }

        buffer.draw_triangle(triangle, 0x0000FFFF);
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

    let mut triangles_to_render: Vec<Triangle> = Vec::new();
    let fov_factor = 650f32;
    
    let camera_position = Vec3::new(0.0, 0.0, -5.0);
    let mut cube_rotation = Vec3::new(0.0, 0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&mut triangles_to_render, fov_factor, &camera_position, &mut cube_rotation);
        render(&mut buffer, &mut window, &mut triangles_to_render);
    }
}
