use tinyvec::ArrayVec;

use crate::{texture::Tex2, vector::Vec3};

// Each time a plane clips a triangle, a maximum of one extra vertex can be produced
// Since a triangle has 3 vertices and we're clipping against the 6 frustum planes
// This means we get a maximum of 3 + 6 = 9 vertices that can possibly exist in a polygon
pub const MAX_POLYGON_VERTS: usize = 9;
pub const MAX_TRIANGLES: usize = MAX_POLYGON_VERTS - 2;

#[derive(Clone, Copy, Default, Debug)]
pub struct PolygonVertex {
    pub pos: Vec3,
    pub uv: Tex2,
}

#[derive(Clone, Default, Debug)]
pub struct Polygon {
    verts: ArrayVec<[PolygonVertex; MAX_POLYGON_VERTS]>,
}

impl Polygon {
    pub fn new(verts: ArrayVec<[PolygonVertex; MAX_POLYGON_VERTS]>) -> Self {
        Self { verts }
    }

    pub fn vertices(&self) -> &[PolygonVertex] {
        &self.verts
    }

    pub fn add_vertex(&mut self, vert: PolygonVertex) {
        self.verts.push(vert);
    }

    /// Fan triangulate the polygon (only works for convex polygons)
    pub fn triangulate(&self) -> ArrayVec<[[PolygonVertex; 3]; MAX_TRIANGLES]> {
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
