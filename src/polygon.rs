use tinyvec::ArrayVec;

use crate::vector::Vec3;

// Each time a plane clips a triangle, a maximum of one extra vertex can be produced
// Since a triangle has 3 vertices and we're clipping against the 6 frustum planes
// This means we get a maximum of 3 + 6 = 9 vertices that can possibly exist in a polygon
pub const MAX_POLYGON_VERTS: usize = 9;
pub const MAX_TRIANGLES: usize = MAX_POLYGON_VERTS - 2;

#[derive(Clone, Default, Debug)]
pub struct Polygon {
    verts: ArrayVec<[Vec3; MAX_POLYGON_VERTS]>,
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            verts: ArrayVec::new(),
        }
    }

    pub fn vertices(&self) -> &[Vec3] {
        &self.verts
    }

    pub fn add_vertex(&mut self, vert: Vec3) {
        self.verts.push(vert);
    }

    /// Fan triangulate the polygon (only works for convex polygons)
    pub fn triangulate(&self) -> ArrayVec<[[Vec3; 3]; MAX_TRIANGLES]> {
        let mut triangles = ArrayVec::new();

        if self.verts.len() < 3 {
            return triangles;
        }

        let origin = self.verts[0];

        for i in 2..self.verts.len() {
            triangles.push([origin, self.verts[i - 1], self.verts[i]]);
        }

        triangles
    }
}

impl From<&[Vec3; 3]> for Polygon {
    fn from(value: &[Vec3; 3]) -> Self {
        let mut polygon = Self::default();

        for &vert in value {
            polygon.add_vertex(vert);
        }

        polygon
    }
}
