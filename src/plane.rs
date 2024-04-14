use crate::{
    polygon::{Polygon, PolygonVertex},
    vector::Vec3,
};

#[derive(Debug)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }

    pub fn point_inside(&self, point: Vec3) -> bool {
        (point - self.point).dot(self.normal) > 0.0
    }

    /// Clip a polygon against the plane (only works for convex polygons)
    pub fn clip_polygon(&self, polygon: &Polygon) -> Polygon {
        if polygon.vertices().len() == 0 {
            return Polygon::default();
        } else if polygon.vertices().len() == 1 && !self.point_inside(polygon.vertices()[0].pos) {
            return Polygon::default();
        } else if polygon.vertices().len() == 1 && self.point_inside(polygon.vertices()[0].pos) {
            return polygon.clone();
        }

        let mut clipped_polygon = Polygon::default();

        let mut previous_vert = polygon.vertices()[polygon.vertices().len() - 1];
        let mut previous_vert_in = self.point_inside(previous_vert.pos);
        for &vert in polygon.vertices().iter() {
            let vert_in = self.point_inside(vert.pos);

            if previous_vert_in && !vert_in || !previous_vert_in && vert_in {
                // Calculate the intersection point between the vertices on the plane
                let d1 = (previous_vert.pos - self.point).dot(self.normal);
                let d2 = (vert.pos - self.point).dot(self.normal);

                let t = d1 / (d1 - d2);

                let intersection = previous_vert.pos + t * (vert.pos - previous_vert.pos);
                let interpolated_uv = previous_vert.uv + t * (vert.uv - previous_vert.uv);

                clipped_polygon.add_vertex(PolygonVertex {
                    pos: intersection,
                    uv: interpolated_uv,
                });
            }

            if vert_in {
                clipped_polygon.add_vertex(vert);
            }

            previous_vert = vert;
            previous_vert_in = vert_in;
        }

        clipped_polygon
    }
}
