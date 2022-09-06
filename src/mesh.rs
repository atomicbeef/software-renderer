use crate::triangle::Face;
use crate::vector::Vec3;

pub const MESH_VERTICES: [Vec3; 8] = [
    Vec3::new(-1.0, -1.0, -1.0),
    Vec3::new(-1.0, 1.0, -1.0),
    Vec3::new(1.0, 1.0, -1.0),
    Vec3::new(1.0, -1.0, -1.0),
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(1.0, -1.0, 1.0),
    Vec3::new(-1.0, 1.0, 1.0),
    Vec3::new(-1.0, -1.0, 1.0)
];

pub const MESH_FACES: [Face; 6 * 2] = [
    // Front
    Face::new(0, 1, 2),
    Face::new(0, 2, 3),
    
    // Right
    Face::new(3, 2, 4),
    Face::new(3, 4, 5),
    
    // Back
    Face::new(5, 4, 6),
    Face::new(5, 6, 7),

    // Left
    Face::new(7, 6, 1),
    Face::new(7, 1, 0),
    
    // Top
    Face::new(1, 6, 4),
    Face::new(1, 4, 2),

    // Bottom
    Face::new(5, 7, 0),
    Face::new(5, 0, 3)
];