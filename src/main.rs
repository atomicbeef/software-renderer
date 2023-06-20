use std::{env, time::Instant};
use std::path::Path;
use std::process::ExitCode;

use matrix::Mat4;
use minifb::{Key, Window, WindowOptions};

mod color_buffer;
mod drawing;
mod matrix;
mod mesh;
mod obj;
mod triangle;
mod vector;

use color_buffer::ColorBuffer;
use mesh::Mesh;
use triangle::Triangle;
use vector::{Vec2, Vec3, Vec4};

const WINDOW_WIDTH: usize = 1024;
const WINDOW_HEIGHT: usize = 768;

#[derive(Clone, Copy)]
enum RenderMode {
    Wireframe,
    WireframeVertex,
    WireframeFilled,
    Filled,
}

#[derive(Clone, Copy)]
struct RenderSettings {
    render_mode: RenderMode,
    backface_cull: bool,
}

fn update(
    triangles_to_render: &mut Vec<Triangle>,
    projection_matrix: Mat4,
    camera_position: Vec3,
    mesh: &mut Mesh,
    settings: RenderSettings,
    elapsed_time: f32,
) {
    triangles_to_render.clear();

    // Animate mesh
    mesh.rotation.x += 0.005;
    mesh.rotation.y += 0.01;
    mesh.rotation.z += 0.01;
    mesh.translation.x = 2.0 * elapsed_time.sin();
    mesh.translation.y = 2.0 * elapsed_time.cos();
    mesh.translation.z = 5.0;

    let scale_matrix = Mat4::scale(mesh.scale.x, mesh.scale.y, mesh.scale.z);
    let translation_matrix = Mat4::translation(mesh.translation.x, mesh.translation.y, mesh.translation.z);
    let rotation_x_matrix = Mat4::rotation_x(mesh.rotation.x);
    let rotation_y_matrix = Mat4::rotation_y(mesh.rotation.y);
    let rotation_z_matrix = Mat4::rotation_z(mesh.rotation.z);

    let world_matrix = translation_matrix * rotation_x_matrix * rotation_y_matrix * rotation_z_matrix * scale_matrix;

    for face in mesh.faces.iter() {
        let face_vertices = [
            mesh.vertices[face.a as usize],
            mesh.vertices[face.b as usize],
            mesh.vertices[face.c as usize]
        ];

        // Transform
        let transformed_vertices = face_vertices.map(|vertex| world_matrix * Vec4::from(vertex));

        if settings.backface_cull {
            // Backface cull
            let ab = Vec3::from(transformed_vertices[1]) - Vec3::from(transformed_vertices[0]);
            let ac = Vec3::from(transformed_vertices[2]) - Vec3::from(transformed_vertices[0]);

            let mut normal = ab.cross(&ac);
            normal.normalize();

            let camera_ray = camera_position - Vec3::from(transformed_vertices[0]);

            if normal.dot(&camera_ray) < 0.0 {
                continue;
            }
        }

        // Project
        let projected_vertices = transformed_vertices.map(|vertex| {
            let mut projected = Vec2::from(projection_matrix.project_vec4(vertex));
            
            // Scale and translate into view
            projected.x *= WINDOW_WIDTH as f32 / 2.0;
            projected.y *= WINDOW_HEIGHT as f32 / 2.0;

            projected.x += WINDOW_WIDTH as f32 / 2.0;
            projected.y += WINDOW_HEIGHT as f32 / 2.0;
            
            projected
        });

        let triangle = Triangle::new(
            projected_vertices[0],
            projected_vertices[1],
            projected_vertices[2],
            (transformed_vertices[0].z + transformed_vertices[1].z + transformed_vertices[2].z) / 3.0
        );

        triangles_to_render.push(triangle);
    }

    // Sort triangles by depth
    triangles_to_render.sort_by(|a, b| { a.depth.partial_cmp(&b.depth).unwrap() });
}

fn render(buffer: &mut ColorBuffer, window: &mut Window, triangles_to_render: &[Triangle], settings: RenderSettings) {
    buffer.draw_grid();

    for triangle in triangles_to_render.iter() {
        for point in triangle.points {
            if point.x == f32::NEG_INFINITY || point.x == f32::INFINITY || point.y == f32::NEG_INFINITY || point.y == f32::INFINITY {
                continue;
            }
    
            if matches!(settings.render_mode, RenderMode::WireframeVertex) {
                buffer.draw_rect(
                    point.x as usize,
                    point.y as usize,
                    2,
                    2,
                    0x0000FF00
                );
            }
        }

        match settings.render_mode {
            RenderMode::Wireframe | RenderMode::WireframeVertex => {
                buffer.draw_triangle(triangle, 0x00FF0000);
            },
            RenderMode::Filled => {
                buffer.draw_filled_triangle(triangle, 0x0000FFFF);
            },
            RenderMode::WireframeFilled => {
                buffer.draw_triangle(triangle, 0x00FF0000);
                buffer.draw_filled_triangle(triangle, 0x0000FFFF);
            }
        };
    }

    window.update_with_buffer(buffer.buffer(), buffer.width(), buffer.height())
        .unwrap();
    
    buffer.clear(0x00000000);
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: Incorrect arguments specified!");
        println!("Usage: software-renderer <mesh>");
        return ExitCode::from(1);
    }

    let mut buffer = ColorBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut window = Window::new(
        "3D Renderer",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default()
    ).expect("Error: Window could not be created!");

    let mesh_path = Path::new(&args[1]);
    let mut mesh = Mesh::from_obj(mesh_path);

    let mut triangles_to_render: Vec<Triangle> = Vec::new();
    
    let camera_position = Vec3::new(0.0, 0.0, 0.0);

    let projection_matrix = Mat4::projection(
        std::f32::consts::FRAC_PI_3,
        WINDOW_HEIGHT as f32 / WINDOW_WIDTH as f32,
        0.1,
        100.0
    );

    let mut render_settings = RenderSettings {
        render_mode: RenderMode::WireframeFilled,
        backface_cull: true
    };

    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Key1) {
            render_settings.render_mode = RenderMode::WireframeVertex;
        } else if window.is_key_down(Key::Key2) {
            render_settings.render_mode = RenderMode::Wireframe;
        } else if window.is_key_down(Key::Key3) {
            render_settings.render_mode = RenderMode::Filled;
        } else if window.is_key_down(Key::Key4) {
            render_settings.render_mode = RenderMode::WireframeFilled;
        }

        if window.is_key_down(Key::C) {
            render_settings.backface_cull = true;
        } else if window.is_key_down(Key::D) {
            render_settings.backface_cull = false;
        }

        update(
            &mut triangles_to_render,
            projection_matrix,
            camera_position,
            &mut mesh,
            render_settings,
            start_time.elapsed().as_secs_f32(),
        );
        render(&mut buffer, &mut window, &triangles_to_render, render_settings);
    }

    return ExitCode::from(0);
}
