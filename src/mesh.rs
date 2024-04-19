use crate::texture::Tex2;
use crate::triangle::Face;
use crate::vector::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub vertex_uvs: Vec<Tex2>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub translation: Vec3,
    initial_translation: Vec3,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vec3>,
        vertex_uvs: Vec<Tex2>,
        faces: Vec<Face>,
        rotation: Vec3,
        scale: Vec3,
        translation: Vec3,
    ) -> Self {
        Self {
            vertices,
            vertex_uvs,
            faces,
            rotation,
            scale,
            translation,
            initial_translation: translation,
        }
    }

    pub fn initial_translation(&self) -> Vec3 {
        self.initial_translation
    }
}
