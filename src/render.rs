use minifb::Window;
use tinyvec::ArrayVec;

use crate::{
    camera::Camera,
    color::Color,
    color_buffer::ColorBuffer,
    depth_buffer::DepthBuffer,
    matrix::Mat4,
    mesh::Mesh,
    polygon::{Polygon, PolygonVertex},
    texture::Texture,
    triangle::Triangle,
    vector::{Vec3, Vec4},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    Wireframe,
    WireframeVertex,
    WireframeFilled,
    Filled,
    WireframeTextured,
    Textured,
}

#[derive(Clone, Copy)]
pub struct RenderSettings {
    pub render_mode: RenderMode,
    pub backface_cull: bool,
    pub shaded: bool,
    pub translate: bool,
    pub rotate: bool,
    pub rotation: Vec3,
    pub scale: bool,
    pub flip_uvs_vertically: bool,
    pub render_width: usize,
    pub render_height: usize,
}

pub fn prepare_triangles(
    triangles_to_render: &mut Vec<Triangle>,
    projection_matrix: Mat4,
    camera: &mut Camera,
    mesh: &mut Mesh,
    settings: &RenderSettings,
) {
    triangles_to_render.clear();

    let scale_matrix = Mat4::scale(mesh.scale.x, mesh.scale.y, mesh.scale.z);
    let translation_matrix =
        Mat4::translation(mesh.translation.x, mesh.translation.y, mesh.translation.z);
    let rotation_x_matrix = Mat4::rotation_x(mesh.rotation.x);
    let rotation_y_matrix = Mat4::rotation_y(mesh.rotation.y);
    let rotation_z_matrix = Mat4::rotation_z(mesh.rotation.z);

    let world_matrix = translation_matrix
        * rotation_x_matrix
        * rotation_y_matrix
        * rotation_z_matrix
        * scale_matrix;
    let camera_matrix = camera.view_matrix();

    for face in mesh.faces.iter() {
        let face_vertices = [
            mesh.vertices[face.a as usize],
            mesh.vertices[face.b as usize],
            mesh.vertices[face.c as usize],
        ];

        // World transform
        let world_transformed_vertices =
            face_vertices.map(|vertex| world_matrix * Vec4::from(vertex));

        // Calculate face normal for backface culling and lighting
        let ab =
            Vec3::from(world_transformed_vertices[1]) - Vec3::from(world_transformed_vertices[0]);
        let ac =
            Vec3::from(world_transformed_vertices[2]) - Vec3::from(world_transformed_vertices[0]);
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
        let triangle_color = if settings.shaded
            && (settings.render_mode == RenderMode::WireframeTextured
                || settings.render_mode == RenderMode::Textured)
        {
            Color::new(255, 255, 255) * percent_lit
        } else if !settings.shaded
            && (settings.render_mode == RenderMode::WireframeTextured
                || settings.render_mode == RenderMode::Textured)
        {
            Color::new(255, 255, 255)
        } else if settings.shaded {
            face.color * percent_lit
        } else {
            face.color
        };

        // Camera transform
        let camera_transformed_vertices =
            world_transformed_vertices.map(|vertex| camera_matrix * vertex);

        // Clip
        let mut polygon_verts = ArrayVec::new();
        polygon_verts.push(PolygonVertex {
            pos: camera_transformed_vertices[0].into(),
            uv: mesh.vertex_uvs[face.a_uv as usize],
        });
        polygon_verts.push(PolygonVertex {
            pos: camera_transformed_vertices[1].into(),
            uv: mesh.vertex_uvs[face.b_uv as usize],
        });
        polygon_verts.push(PolygonVertex {
            pos: camera_transformed_vertices[2].into(),
            uv: mesh.vertex_uvs[face.c_uv as usize],
        });

        let polygon = Polygon::new(polygon_verts);

        let clipping_planes =
            camera.clipping_planes(settings.render_width as f32 / settings.render_height as f32);

        let polygon = clipping_planes.right.clip_polygon(&polygon);
        let polygon = clipping_planes.left.clip_polygon(&polygon);
        let polygon = clipping_planes.top.clip_polygon(&polygon);
        let polygon = clipping_planes.bottom.clip_polygon(&polygon);
        let polygon = clipping_planes.far.clip_polygon(&polygon);
        let polygon = clipping_planes.near.clip_polygon(&polygon);

        let clipped_triangles = polygon.triangulate();

        // Project
        for triangle in clipped_triangles {
            let projected_vertices = triangle.map(|vertex| {
                let vertex = Vec4::from(vertex.pos);
                let mut projected = projection_matrix.project_vec4(vertex);

                // Scale and translate into view
                projected.x *= settings.render_width as f32 / 1.0;
                projected.y *= settings.render_height as f32 / -1.0;

                projected.x += settings.render_width as f32 / 2.0;
                projected.y += settings.render_height as f32 / 2.0;

                projected
            });

            let triangle = Triangle::new(
                projected_vertices[0],
                projected_vertices[1],
                projected_vertices[2],
                triangle[0].uv,
                triangle[1].uv,
                triangle[2].uv,
                triangle_color,
            );

            triangles_to_render.push(triangle);
        }
    }
}

pub fn render(
    color_buffer: &mut ColorBuffer,
    depth_buffer: &mut DepthBuffer,
    window: &mut Window,
    triangles_to_render: &[Triangle],
    settings: &RenderSettings,
    texture: &Texture,
) {
    color_buffer.draw_grid();
    depth_buffer.clear(1.0);

    for triangle in triangles_to_render.iter() {
        for point in triangle.points {
            if point.x == f32::NEG_INFINITY
                || point.x == f32::INFINITY
                || point.y == f32::NEG_INFINITY
                || point.y == f32::INFINITY
            {
                continue;
            }

            if matches!(settings.render_mode, RenderMode::WireframeVertex) {
                color_buffer.draw_rect(
                    point.x as usize,
                    point.y as usize,
                    2,
                    2,
                    Color::new(0, 0xFF, 0),
                );
            }
        }

        match settings.render_mode {
            RenderMode::Wireframe | RenderMode::WireframeVertex => {
                color_buffer.draw_triangle(triangle, Color::new(0, 0xFF, 0));
            }
            RenderMode::Filled => {
                color_buffer.draw_filled_triangle(triangle, triangle.color, depth_buffer);
            }
            RenderMode::WireframeFilled => {
                color_buffer.draw_triangle(triangle, Color::new(0xFF, 0, 0));
                color_buffer.draw_filled_triangle(triangle, triangle.color, depth_buffer);
            }
            RenderMode::Textured => color_buffer.draw_textured_triangle(
                triangle,
                &texture,
                depth_buffer,
                settings.flip_uvs_vertically,
            ),
            RenderMode::WireframeTextured => {
                color_buffer.draw_triangle(triangle, Color::new(0xFF, 0, 0));
                color_buffer.draw_textured_triangle(
                    triangle,
                    &texture,
                    depth_buffer,
                    settings.flip_uvs_vertically,
                );
            }
        };
    }

    window
        .update_with_buffer(
            color_buffer.buffer(),
            color_buffer.width(),
            color_buffer.height(),
        )
        .unwrap();

    color_buffer.clear(Color::new(0, 0, 0));
}
