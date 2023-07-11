use crate::color::Color;
use crate::texture::Tex2;
use crate::triangle::Face;
use crate::vector::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub translation: Vec3,
}

impl Mesh {
    pub fn cube(half_extents: f32) -> Self {
        let vertices = vec![
            Vec3::new(-half_extents, -half_extents, -half_extents),
            Vec3::new(-half_extents, half_extents, -half_extents),
            Vec3::new(half_extents, half_extents, -half_extents),
            Vec3::new(half_extents, -half_extents, -half_extents),
            Vec3::new(-half_extents, half_extents, half_extents),
            Vec3::new(half_extents, half_extents, half_extents),
            Vec3::new(-half_extents, -half_extents, half_extents),
            Vec3::new(half_extents, -half_extents, half_extents),
        ];

        let faces = vec![
            // Front
            Face::new(
                2,
                3,
                0,
                Tex2::new(1.0, 1.0),
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
                Color::new(0xFF, 0, 0)
            ),
            Face::new(
                0,
                1,
                2,
                Tex2::new(0.0, 0.0),
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 1.0),
                Color::new(0xFF, 0, 0)
            ),
            // Right
            Face::new(
                7,
                3,
                2,
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
                Tex2::new(0.0, 1.0),
                Color::new(0, 0xFF, 0)
            ),
            Face::new(
                2,
                5,
                7,
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 1.0),
                Tex2::new(1.0, 0.0),
                Color::new(0, 0xFF, 0)
            ),
            // Top
            Face::new(
                2,
                1,
                4,
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
                Tex2::new(0.0, 1.0),
                Color::new(0, 0, 0xFF)
            ),
            Face::new(
                4,
                5,
                2,
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 1.0),
                Tex2::new(1.0, 0.0),
                Color::new(0, 0, 0xFF)
            ),
            // Back
            Face::new(
                4,
                6,
                7,
                Tex2::new(1.0, 1.0),
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
                Color::new(0xFF, 0xFF, 0)
            ),
            Face::new(
                7,
                5,
                4,
                Tex2::new(0.0, 0.0),
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 1.0),
                Color::new(0xFF, 0xFF, 0)
            ),
            // Left
            Face::new(
                1,
                0,
                6,
                Tex2::new(1.0, 1.0),
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
                Color::new(0xFF, 0, 0xFF)
            ),
            Face::new(
                6,
                4,
                1,
                Tex2::new(0.0, 0.0),
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 1.0),
                Color::new(0xFF, 0, 0xFF)
            ),
            // Bottom
            Face::new(
                6,
                0,
                3,
                Tex2::new(0.0, 0.0),
                Tex2::new(0.0, 1.0),
                Tex2::new(1.0, 1.0),
                Color::new(0, 0xFF, 0xFF)
            ),
            Face::new(
                3,
                7,
                6,
                Tex2::new(1.0, 1.0),
                Tex2::new(1.0, 0.0),
                Tex2::new(0.0, 0.0),
                Color::new(0, 0xFF, 0xFF)
            )
        ];

        Self {
            vertices,
            faces,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}