use minifb::{Key, Window, WindowOptions};

mod color_buffer;
mod drawing;
mod mesh;
mod triangle;
mod vector;

use color_buffer::ColorBuffer;
use mesh::{Mesh, CUBE_FACES, CUBE_VERTICES};
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
    mesh: &mut Mesh
) {
    triangles_to_render.clear();

    mesh.rotation.x += 0.01;
    mesh.rotation.y += 0.01;
    mesh.rotation.z += 0.01;

    for face in mesh.faces.iter() {
        let face_vertices = [
            mesh.vertices[face.a as usize],
            mesh.vertices[face.b as usize],
            mesh.vertices[face.c as usize]
        ];

        let mut triangle = Triangle::new(Vec2::default(), Vec2::default(), Vec2::default());

        for (i, vertex) in face_vertices.iter().enumerate() {
            let transformed_vertex = vertex.rotated_x(mesh.rotation.x);
            let transformed_vertex = transformed_vertex.rotated_y(mesh.rotation.y);
            let mut transformed_vertex = transformed_vertex.rotated_z(mesh.rotation.z);

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

    let mut mesh = Mesh::new(CUBE_VERTICES.into(), CUBE_FACES.into(), Vec3::new(0.0, 0.0, 0.0));

    let mut triangles_to_render: Vec<Triangle> = Vec::new();
    let fov_factor = 650f32;
    
    let camera_position = Vec3::new(0.0, 0.0, -5.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&mut triangles_to_render, fov_factor, &camera_position, &mut mesh);
        render(&mut buffer, &mut window, &triangles_to_render);
    }
}
