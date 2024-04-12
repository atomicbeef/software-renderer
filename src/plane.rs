use crate::{polygon::Polygon, vector::Vec3};

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

    /// Assumes that point actually intersects the plane somewhere
    pub fn intersection(&self, a: Vec3, b: Vec3) -> Vec3 {
        let d1 = (a - self.point).dot(self.normal);
        let d2 = (b - self.point).dot(self.normal);

        let t = d1 / (d1 - d2);

        a + t * (b - a)
    }

    /// Clip a polygon against the plane (only works for convex polygons)
    pub fn clip_polygon(&self, polygon: &Polygon) -> Polygon {
        if polygon.vertices().len() == 0 {
            return Polygon::new();
        } else if polygon.vertices().len() == 1 && !self.point_inside(polygon.vertices()[0]) {
            return Polygon::new();
        } else if polygon.vertices().len() == 1 && self.point_inside(polygon.vertices()[0]) {
            return polygon.clone();
        }

        let mut clipped_polygon = Polygon::new();

        let mut previous_vert = polygon.vertices()[polygon.vertices().len() - 1];
        let mut previous_vert_in = self.point_inside(previous_vert);
        for &vert in polygon.vertices().iter() {
            let vert_in = self.point_inside(vert);

            if previous_vert_in && !vert_in || !previous_vert_in && vert_in {
                let intersection = self.intersection(previous_vert, vert);
                clipped_polygon.add_vertex(intersection);
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
