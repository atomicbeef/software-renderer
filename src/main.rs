use std::{env, time::Instant};
use std::path::Path;
use std::process::ExitCode;

use camera::Camera;
use color::Color;
use depth_buffer::DepthBuffer;
use matrix::Mat4;
use minifb::{Key, Window, WindowOptions, KeyRepeat};

mod camera;
mod color;
mod color_buffer;
mod depth_buffer;
mod drawing;
mod matrix;
mod mesh;
mod obj;
mod texture;
mod triangle;
mod vector;

use color_buffer::ColorBuffer;
use mesh::Mesh;
use texture::Texture;
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
    WireframeTextured,
    Textured,
}

#[derive(Clone, Copy)]
struct RenderSettings {
    render_mode: RenderMode,
    backface_cull: bool,
    shaded: bool,
    translate: bool,
    rotate: bool,
    rotation: Vec3,
    scale: bool,
    flip_uvs_vertically: bool,
}

fn update(
    triangles_to_render: &mut Vec<Triangle>,
    projection_matrix: Mat4,
    camera: &mut Camera,
    mesh: &mut Mesh,
    settings: RenderSettings,
    elapsed_time: f32,
) {
    triangles_to_render.clear();

    // Animate mesh
    mesh.scale = if settings.scale { Vec3::splat(1.0) * (2.0 * elapsed_time.sin().abs() + 0.05) } else { Vec3::splat(1.0) };
    mesh.rotation = if settings.rotate { mesh.rotation + settings.rotation } else { mesh.rotation };
    mesh.translation.x = if settings.translate { 2.0 * elapsed_time.sin() } else { 0.0 };
    mesh.translation.y = if settings.translate { 2.0 * elapsed_time.cos() } else { 0.0 };
    mesh.translation.z = 5.0;

    let scale_matrix = Mat4::scale(mesh.scale.x, mesh.scale.y, mesh.scale.z);
    let translation_matrix = Mat4::translation(mesh.translation.x, mesh.translation.y, mesh.translation.z);
    let rotation_x_matrix = Mat4::rotation_x(mesh.rotation.x);
    let rotation_y_matrix = Mat4::rotation_y(mesh.rotation.y);
    let rotation_z_matrix = Mat4::rotation_z(mesh.rotation.z);

    let world_matrix = translation_matrix * rotation_x_matrix * rotation_y_matrix * rotation_z_matrix * scale_matrix;
    let camera_matrix = camera.view_matrix();

    for face in mesh.faces.iter() {
        let face_vertices = [
            mesh.vertices[face.a as usize],
            mesh.vertices[face.b as usize],
            mesh.vertices[face.c as usize]
        ];

        // World transform
        let world_transformed_vertices = face_vertices.map(|vertex| world_matrix * Vec4::from(vertex));

        // Calculate face normal for backface culling and lighting
        let ab = Vec3::from(world_transformed_vertices[1]) - Vec3::from(world_transformed_vertices[0]);
        let ac = Vec3::from(world_transformed_vertices[2]) - Vec3::from(world_transformed_vertices[0]);
        let normal = ab.cross(ac).normalized();

        if settings.backface_cull {
            let camera_ray = camera.translation - Vec3::from(world_transformed_vertices[0]);

            if normal.dot(camera_ray) < 0.0 {
                continue;
            }
        }

        // Lighting
        let light_direction = Vec3::new(0.0, 0.0, 1.0).normalized();
        let percent_lit = normal.dot(light_direction) * -0.5 + 0.5;
        let triangle_color = if settings.shaded { face.color * percent_lit } else { face.color };

        // Camera transform
        let camera_transformed_vertices = world_transformed_vertices.map(|vertex| camera_matrix * vertex);

        // Project
        let projected_vertices = camera_transformed_vertices.map(|vertex| {
            let mut projected = projection_matrix.project_vec4(vertex);
            
            // Scale and translate into view
            projected.x *= WINDOW_WIDTH as f32 / 1.0;
            projected.y *= WINDOW_HEIGHT as f32 / -1.0;

            projected.x += WINDOW_WIDTH as f32 / 2.0;
            projected.y += WINDOW_HEIGHT as f32 / 2.0;
            
            projected
        });

        let triangle = Triangle::new(
            projected_vertices[0],
            projected_vertices[1],
            projected_vertices[2],
            mesh.vertex_uvs[face.a_uv as usize],
            mesh.vertex_uvs[face.b_uv as usize],
            mesh.vertex_uvs[face.c_uv as usize],
            triangle_color
        );

        triangles_to_render.push(triangle);
    }
}

fn render(
    color_buffer: &mut ColorBuffer,
    depth_buffer: &mut DepthBuffer,
    window: &mut Window,
    triangles_to_render: &[Triangle],
    settings: RenderSettings,
    texture: &Texture,
) {
    color_buffer.draw_grid();
    depth_buffer.clear(1.0);

    for triangle in triangles_to_render.iter() {
        for point in triangle.points {
            if point.x == f32::NEG_INFINITY || point.x == f32::INFINITY || point.y == f32::NEG_INFINITY || point.y == f32::INFINITY {
                continue;
            }
    
            if matches!(settings.render_mode, RenderMode::WireframeVertex) {
                color_buffer.draw_rect(
                    point.x as usize,
                    point.y as usize,
                    2,
                    2,
                    Color::new(0, 0xFF, 0)
                );
            }
        }

        match settings.render_mode {
            RenderMode::Wireframe | RenderMode::WireframeVertex => {
                color_buffer.draw_triangle(triangle, Color::new(0, 0xFF, 0));
            },
            RenderMode::Filled => {
                color_buffer.draw_filled_triangle(triangle, triangle.color, depth_buffer);
            },
            RenderMode::WireframeFilled => {
                color_buffer.draw_triangle(triangle, Color::new(0xFF, 0, 0));
                color_buffer.draw_filled_triangle(triangle, triangle.color, depth_buffer);
            },
            RenderMode::Textured => {
                color_buffer.draw_textured_triangle(triangle, &texture, depth_buffer, settings.flip_uvs_vertically)
            },
            RenderMode::WireframeTextured => {
                color_buffer.draw_triangle(triangle, Color::new(0xFF, 0, 0));
                color_buffer.draw_textured_triangle(triangle, &texture, depth_buffer, settings.flip_uvs_vertically);
            }
        };
    }

    window.update_with_buffer(color_buffer.buffer(), color_buffer.width(), color_buffer.height())
        .unwrap();
    
    color_buffer.clear(Color::new(0, 0, 0));
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Error: Incorrect arguments specified!");
        println!("Usage: software-renderer [mesh]");
        return ExitCode::from(1);
    }

    let mut color_buffer = ColorBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut depth_buffer = DepthBuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut window = Window::new(
        "3D Renderer",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default()
    ).expect("Error: Window could not be created!");

    let mut mesh = if args.len() == 1 {
        Mesh::cube(1.0)
    } else {
        let mesh_path = Path::new(&args[1]);
        Mesh::from_obj(mesh_path)
    };

    let texture = if args.len() == 1 {
        Texture::grid(
            64,
            64,
            Color::new(0xFF, 0, 0),
            Color::new(0xFF, 0xFF, 0xFF)
        )
    } else {
        let texture_path = Path::new(&args[1]).with_extension("png");
        Texture::from_png(&texture_path).unwrap_or_else(|err| {
            eprintln!("Error reading texture: {err}");
            Texture::from_color(1, 1, Color::new(0xFF, 0x00, 0xFF))
        })
    };

    let mut triangles_to_render: Vec<Triangle> = Vec::new();
    
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0)
    );

    let projection_matrix = Mat4::projection(
        std::f32::consts::FRAC_PI_3,
        WINDOW_HEIGHT as f32 / WINDOW_WIDTH as f32,
        0.1,
        100.0
    );

    let mut render_settings = RenderSettings {
        render_mode: RenderMode::Textured,
        backface_cull: true,
        shaded: true,
        translate: false,
        rotate: true,
        rotation: Vec3::new(0.0, 0.01, 0.0),
        scale: false,
        flip_uvs_vertically: false,
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
        } else if window.is_key_down(Key::Key5) {
            render_settings.render_mode = RenderMode::Textured;
        } else if window.is_key_down(Key::Key6) {
            render_settings.render_mode = RenderMode::WireframeTextured;
        }

        if window.is_key_down(Key::C) {
            render_settings.backface_cull = true;
        } else if window.is_key_down(Key::D) {
            render_settings.backface_cull = false;
        }

        if window.is_key_down(Key::L) {
            render_settings.shaded = true;
        } else if window.is_key_down(Key::U) {
            render_settings.shaded = false;
        }

        if window.is_key_pressed(Key::T, KeyRepeat::No) {
            render_settings.translate = !render_settings.translate;
        }
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            render_settings.rotate = !render_settings.rotate;
        }
        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            render_settings.scale = !render_settings.scale;
        }

        if window.is_key_pressed(Key::X, KeyRepeat::No) {
            render_settings.rotation.x = if render_settings.rotation.x > 0.0 { 0.0 } else { 0.01 };
        }
        if window.is_key_pressed(Key::Y, KeyRepeat::No) {
            render_settings.rotation.y = if render_settings.rotation.y > 0.0 { 0.0 } else { 0.01 };
        }
        if window.is_key_pressed(Key::Z, KeyRepeat::No) {
            render_settings.rotation.z = if render_settings.rotation.z > 0.0 { 0.0 } else { 0.01 };
        }

        if window.is_key_pressed(Key::P, KeyRepeat::No) {
            mesh.rotation = Vec3::default();
        }

        if window.is_key_pressed(Key::F, KeyRepeat::No) {
            render_settings.flip_uvs_vertically = !render_settings.flip_uvs_vertically;
        }

        update(
            &mut triangles_to_render,
            projection_matrix,
            &mut camera,
            &mut mesh,
            render_settings,
            start_time.elapsed().as_secs_f32(),
        );
        render(&mut color_buffer, &mut depth_buffer, &mut window, &triangles_to_render, render_settings, &texture);
    }

    return ExitCode::from(0);
}
