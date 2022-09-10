use crate::triangle::Face;
use crate::vector::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3
}