use crate::{
    polygon::{Polygon, PolygonVertex},
    vector::Vec4,
};

#[derive(Debug)]
pub enum Plane {
    Left,
    Right,
    Top,
    Bottom,
    Far,
    Near,
    W,
}

const W_EPSILON: f32 = 0.00001;

impl Plane {
    pub fn point_inside(&self, point: Vec4) -> bool {
        match self {
            Plane::Right => point.x <= point.w,
            Plane::Left => point.x >= -point.w,
            Plane::Top => point.y <= point.w,
            Plane::Bottom => point.y >= -point.w,
            Plane::Far => point.z <= point.w,
            Plane::Near => point.z >= -point.w,
            // Prevent division by 0 if clipping produces coordinate with w = 0
            Plane::W => point.w >= W_EPSILON,
        }
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
                let t = match self {
                    Plane::Left => {
                        (previous_vert.pos.w + previous_vert.pos.x)
                            / ((previous_vert.pos.w + previous_vert.pos.x)
                                - (vert.pos.w + vert.pos.x))
                    }
                    Plane::Right => {
                        (previous_vert.pos.w - previous_vert.pos.x)
                            / ((previous_vert.pos.w - previous_vert.pos.x)
                                - (vert.pos.w - vert.pos.x))
                    }
                    Plane::Top => {
                        (previous_vert.pos.w - previous_vert.pos.y)
                            / ((previous_vert.pos.w - previous_vert.pos.y)
                                - (vert.pos.w - vert.pos.y))
                    }
                    Plane::Bottom => {
                        (previous_vert.pos.w + previous_vert.pos.y)
                            / ((previous_vert.pos.w + previous_vert.pos.y)
                                - (vert.pos.w + vert.pos.y))
                    }
                    Plane::Far => {
                        (previous_vert.pos.w - previous_vert.pos.z)
                            / ((previous_vert.pos.w - previous_vert.pos.z)
                                - (vert.pos.w - vert.pos.z))
                    }
                    Plane::Near => {
                        (previous_vert.pos.w + previous_vert.pos.z)
                            / ((previous_vert.pos.w + previous_vert.pos.z)
                                - (vert.pos.w + vert.pos.z))
                    }
                    Plane::W => {
                        (W_EPSILON - previous_vert.pos.w) / (previous_vert.pos.w - vert.pos.w)
                    }
                };

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
