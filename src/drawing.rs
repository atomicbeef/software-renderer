use crate::color::Color;
use crate::color_buffer::ColorBuffer;
use crate::depth_buffer::DepthBuffer;
use crate::texture::Texture;
use crate::triangle::{RasterPoint, Triangle, VertexPos};

impl ColorBuffer {
    pub fn draw_grid(&mut self) {
        for x in (0..self.width()).step_by(10) {
            for y in (0..self.height()).step_by(10) {
                if x % 10 == 0 || y % 10 == 0 {
                    self.set(x, y, Color::new(0x55, 0x55, 0x55));
                }
            }
        }
    }

    pub fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, color: Color) {
        for xi in 0..width.min(self.width() - x) {
            for yi in 0..height.min(self.height() - y) {
                self.set(x + xi, y + yi, color);
            }
        }
    }

    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: Color) {
        let dx = x1 as isize - x0 as isize;
        let dy = y1 as isize - y0 as isize;

        let side_length = if dx.abs() >= dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        };

        let x_inc = dx as f32 / side_length as f32;
        let y_inc = dy as f32 / side_length as f32;

        let mut x = x0 as f32;
        let mut y = y0 as f32;

        for _ in 0..side_length {
            self.set(x.round() as u16, y.round() as u16, color);
            x += x_inc;
            y += y_inc;
        }
    }

    pub fn draw_triangle(&mut self, triangle: &Triangle, color: Color) {
        // A -> B
        self.draw_line(
            triangle.points[0].x.round() as u16,
            triangle.points[0].y.round() as u16,
            triangle.points[1].x.round() as u16,
            triangle.points[1].y.round() as u16,
            color,
        );

        // B -> C
        self.draw_line(
            triangle.points[1].x.round() as u16,
            triangle.points[1].y.round() as u16,
            triangle.points[2].x.round() as u16,
            triangle.points[2].y.round() as u16,
            color,
        );

        // C -> A
        self.draw_line(
            triangle.points[2].x.round() as u16,
            triangle.points[2].y.round() as u16,
            triangle.points[0].x.round() as u16,
            triangle.points[0].y.round() as u16,
            color,
        );
    }

    pub fn draw_filled_triangle(
        &mut self,
        triangle: &Triangle,
        color: Color,
        depth_buffer: &mut DepthBuffer,
    ) {
        let (min_x, min_y, max_x, max_y) = triangle.bounding_box();

        let a_pos = VertexPos {
            x: triangle.points[0].x.round() as i32,
            y: triangle.points[0].y.round() as i32,
            z: triangle.points[0].z,
            w: triangle.points[0].w,
        };
        let a_point = RasterPoint::new(a_pos.x, a_pos.y);

        let b_pos = VertexPos {
            x: triangle.points[1].x.round() as i32,
            y: triangle.points[1].y.round() as i32,
            z: triangle.points[1].z,
            w: triangle.points[1].w,
        };
        let b_point = RasterPoint::new(b_pos.x, b_pos.y);

        let c_pos = VertexPos {
            x: triangle.points[2].x.round() as i32,
            y: triangle.points[2].y.round() as i32,
            z: triangle.points[2].z,
            w: triangle.points[2].w,
        };
        let c_point = RasterPoint::new(c_pos.x, c_pos.y);

        let area = (b_point - a_point).cross(c_point - a_point);

        let bias_1 = RasterPoint::edge_orientation(a_point, b_point);
        let bias_2 = RasterPoint::edge_orientation(b_point, c_point);
        let bias_3 = RasterPoint::edge_orientation(c_point, a_point);

        let delta_w0_x = a_point.y - b_point.y;
        let delta_w0_y = b_point.x - a_point.x;

        let delta_w1_x = b_point.y - c_point.y;
        let delta_w1_y = c_point.x - b_point.x;

        let delta_w2_x = c_point.y - a_point.y;
        let delta_w2_y = a_point.x - c_point.x;

        let p0 = RasterPoint::new(min_x as i32, min_y as i32);
        let mut w0_row = p0.edge_weight(a_point, b_point, bias_1);
        let mut w1_row = p0.edge_weight(b_point, c_point, bias_2);
        let mut w2_row = p0.edge_weight(c_point, a_point, bias_3);

        for y in min_y..=max_y {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in min_x..=max_x {
                if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                    let alpha = w1 as f32 / area as f32;
                    let beta = w2 as f32 / area as f32;
                    let gamma = w0 as f32 / area as f32;

                    let interpolated_reciprocal_w =
                        1.0 / a_pos.w * alpha + 1.0 / b_pos.w * beta + 1.0 / c_pos.w * gamma;

                    if 1.0 - interpolated_reciprocal_w < depth_buffer.get(x, y) {
                        self.set(x, y, color);
                        depth_buffer.set(x, y, 1.0 - interpolated_reciprocal_w);
                    }
                }

                w0 += delta_w0_x;
                w1 += delta_w1_x;
                w2 += delta_w2_x;
            }

            w0_row += delta_w0_y;
            w1_row += delta_w1_y;
            w2_row += delta_w2_y;
        }
    }

    pub fn draw_textured_triangle(
        &mut self,
        triangle: &Triangle,
        texture: &Texture,
        depth_buffer: &mut DepthBuffer,
        flip_v: bool,
    ) {
        let (min_x, min_y, max_x, max_y) = triangle.bounding_box();

        let a_pos = VertexPos {
            x: triangle.points[0].x.round() as i32,
            y: triangle.points[0].y.round() as i32,
            z: triangle.points[0].z,
            w: triangle.points[0].w,
        };
        let a_point = RasterPoint::new(a_pos.x, a_pos.y);

        let b_pos = VertexPos {
            x: triangle.points[1].x.round() as i32,
            y: triangle.points[1].y.round() as i32,
            z: triangle.points[1].z,
            w: triangle.points[1].w,
        };
        let b_point = RasterPoint::new(b_pos.x, b_pos.y);

        let c_pos = VertexPos {
            x: triangle.points[2].x.round() as i32,
            y: triangle.points[2].y.round() as i32,
            z: triangle.points[2].z,
            w: triangle.points[2].w,
        };
        let c_point = RasterPoint::new(c_pos.x, c_pos.y);

        let area = (b_point - a_point).cross(c_point - a_point);

        let bias_1 = RasterPoint::edge_orientation(a_point, b_point);
        let bias_2 = RasterPoint::edge_orientation(b_point, c_point);
        let bias_3 = RasterPoint::edge_orientation(c_point, a_point);

        let delta_w0_x = a_point.y - b_point.y;
        let delta_w0_y = b_point.x - a_point.x;

        let delta_w1_x = b_point.y - c_point.y;
        let delta_w1_y = c_point.x - b_point.x;

        let delta_w2_x = c_point.y - a_point.y;
        let delta_w2_y = a_point.x - c_point.x;

        let p0 = RasterPoint::new(min_x as i32, min_y as i32);
        let mut w0_row = p0.edge_weight(a_point, b_point, bias_1);
        let mut w1_row = p0.edge_weight(b_point, c_point, bias_2);
        let mut w2_row = p0.edge_weight(c_point, a_point, bias_3);

        for y in min_y..=max_y {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in min_x..=max_x {
                if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                    let alpha = w1 as f32 / area as f32;
                    let beta = w2 as f32 / area as f32;
                    let gamma = w0 as f32 / area as f32;

                    let interpolated_reciprocal_w =
                        1.0 / a_pos.w * alpha + 1.0 / b_pos.w * beta + 1.0 / c_pos.w * gamma;

                    let mut p_uv = ((triangle.tex_coords[0] / a_pos.w * alpha)
                        + (triangle.tex_coords[1] / b_pos.w * beta)
                        + (triangle.tex_coords[2] / c_pos.w * gamma))
                        / interpolated_reciprocal_w;

                    if flip_v {
                        p_uv.v = 1.0 - p_uv.v;
                    }

                    let color = texture.sample(p_uv) * triangle.color;

                    if 1.0 - interpolated_reciprocal_w < depth_buffer.get(x, y) {
                        self.set(x, y, color);
                        depth_buffer.set(x, y, 1.0 - interpolated_reciprocal_w);
                    }
                }

                w0 += delta_w0_x;
                w1 += delta_w1_x;
                w2 += delta_w2_x;
            }

            w0_row += delta_w0_y;
            w1_row += delta_w1_y;
            w2_row += delta_w2_y;
        }
    }
}
