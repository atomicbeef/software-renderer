use std::ops::{Add, Sub};

use crate::color::Color;
use crate::color_buffer::ColorBuffer;
use crate::depth_buffer::DepthBuffer;
use crate::fixed::FixedI32;
use crate::texture::Texture;
use crate::triangle::Triangle;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vec2I {
    x: i32,
    y: i32,
}

impl Vec2I {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2I> for Vec2I {
    type Output = Self;

    fn add(self, rhs: Vec2I) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vec2I> for Vec2I {
    type Output = Self;

    fn sub(self, rhs: Vec2I) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RasterPoint {
    pub x: FixedI32,
    pub y: FixedI32,
}

impl RasterPoint {
    pub fn new(x: FixedI32, y: FixedI32) -> Self {
        Self { x, y }
    }

    pub fn cross(&self, b: Self) -> FixedI32 {
        self.x * b.y - self.y * b.x
    }

    pub fn edge_weight(&self, a: Self, b: Self, bias: FixedI32) -> FixedI32 {
        let ab = b - a;
        let ap = *self - a;

        ab.cross(ap) + bias
    }

    /// Returns 0 if an edge is flat top or left, otherwise returns -1
    pub fn edge_orientation(a: Self, b: Self) -> FixedI32 {
        let is_flat_top = b.y - a.y == FixedI32::ZERO && b.x - a.x > FixedI32::ZERO;
        let is_left = b.y - a.y < FixedI32::ZERO;

        if is_flat_top || is_left {
            FixedI32::ZERO
        } else {
            FixedI32::NEGATIVE_ONE
        }
    }
}

impl Add<RasterPoint> for RasterPoint {
    type Output = RasterPoint;

    fn add(self, rhs: RasterPoint) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<RasterPoint> for RasterPoint {
    type Output = RasterPoint;

    fn sub(self, rhs: RasterPoint) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

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

    pub fn draw_line(&mut self, p0: RasterPoint, p1: RasterPoint, color: Color) {
        let dp = p1 - p0;

        let side_length = if dp.x.abs() >= dp.y.abs() {
            dp.x.abs()
        } else {
            dp.y.abs()
        };

        if side_length == FixedI32::ZERO {
            return;
        }

        let x_inc = dp.x / side_length;
        let y_inc = dp.y / side_length;

        let mut p = p0;

        for _ in 0..side_length.to_i32_lossy() {
            self.set(p.x.to_i32_lossy() as u16, p.y.to_i32_lossy() as u16, color);
            p.x += x_inc;
            p.y += y_inc;
        }
    }

    pub fn draw_triangle(&mut self, triangle: &Triangle, color: Color) {
        let a = RasterPoint::new(
            FixedI32::from_f32_lossy(triangle.points[0].x),
            FixedI32::from_f32_lossy(triangle.points[0].y),
        );
        let b = RasterPoint::new(
            FixedI32::from_f32_lossy(triangle.points[1].x),
            FixedI32::from_f32_lossy(triangle.points[1].y),
        );
        let c = RasterPoint::new(
            FixedI32::from_f32_lossy(triangle.points[2].x),
            FixedI32::from_f32_lossy(triangle.points[2].y),
        );

        self.draw_line(a, b, color);
        self.draw_line(b, c, color);
        self.draw_line(c, a, color);
    }

    fn rasterize_triangle<F: FnMut(u16, u16, f32, f32, f32, &mut Self)>(
        &mut self,
        triangle: &Triangle,
        mut fill: F,
    ) {
        let (min_x, min_y, max_x, max_y) = triangle.bounding_box();
        let min_x = (min_x - 1.0).floor().max(0.0) as u16;
        let min_y = (min_y - 1.0).floor().max(0.0) as u16;
        let max_x = (max_x + 1.0).ceil().min((self.width() - 1) as f32) as u16;
        let max_y = (max_y + 1.0).ceil().min((self.height() - 1) as f32) as u16;

        let a = RasterPoint::new(
            FixedI32::from_f32_lossy(triangle.points[0].x),
            FixedI32::from_f32_lossy(triangle.points[0].y),
        );

        let b = RasterPoint::new(
            FixedI32::from_f32_lossy(triangle.points[1].x),
            FixedI32::from_f32_lossy(triangle.points[1].y),
        );

        let c = RasterPoint::new(
            FixedI32::from_f32_lossy(triangle.points[2].x),
            FixedI32::from_f32_lossy(triangle.points[2].y),
        );

        let area = (b - a).cross(c - a);

        let bias_1 = RasterPoint::edge_orientation(a, b);
        let bias_2 = RasterPoint::edge_orientation(b, c);
        let bias_3 = RasterPoint::edge_orientation(c, a);

        let delta_w0_x = a.y - b.y;
        let delta_w0_y = b.x - a.x;

        let delta_w1_x = b.y - c.y;
        let delta_w1_y = c.x - b.x;

        let delta_w2_x = c.y - a.y;
        let delta_w2_y = a.x - c.x;

        let p0 = RasterPoint::new(
            FixedI32::from(min_x) + FixedI32::HALF_PIXEL,
            FixedI32::from(min_y) + FixedI32::HALF_PIXEL,
        );
        let mut w0_row = p0.edge_weight(a, b, bias_1);
        let mut w1_row = p0.edge_weight(b, c, bias_2);
        let mut w2_row = p0.edge_weight(c, a, bias_3);

        for y in min_y..=max_y {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in min_x..=max_x {
                if w0 >= FixedI32::ZERO && w1 >= FixedI32::ZERO && w2 >= FixedI32::ZERO {
                    let alpha = f32::from(w1) / f32::from(area);
                    let beta = f32::from(w2) / f32::from(area);
                    let gamma = f32::from(w0) / f32::from(area);

                    fill(x, y, alpha, beta, gamma, self);
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

    pub fn draw_filled_triangle(
        &mut self,
        triangle: &Triangle,
        color: Color,
        depth_buffer: &mut DepthBuffer,
    ) {
        let fill = |x, y, alpha, beta, gamma, color_buffer: &mut Self| {
            let interpolated_reciprocal_w = 1.0 / triangle.points[0].w * alpha
                + 1.0 / triangle.points[1].w * beta
                + 1.0 / triangle.points[2].w * gamma;

            if 1.0 - interpolated_reciprocal_w < depth_buffer.get(x, y) {
                color_buffer.set(x, y, color);
                depth_buffer.set(x, y, 1.0 - interpolated_reciprocal_w);
            }
        };

        self.rasterize_triangle(triangle, fill);
    }

    pub fn draw_textured_triangle(
        &mut self,
        triangle: &Triangle,
        texture: &Texture,
        depth_buffer: &mut DepthBuffer,
        flip_v: bool,
    ) {
        let fill = |x, y, alpha, beta, gamma, color_buffer: &mut Self| {
            let interpolated_reciprocal_w = 1.0 / triangle.points[0].w * alpha
                + 1.0 / triangle.points[1].w * beta
                + 1.0 / triangle.points[2].w * gamma;

            let mut p_uv = ((triangle.tex_coords[0] / triangle.points[0].w * alpha)
                + (triangle.tex_coords[1] / triangle.points[1].w * beta)
                + (triangle.tex_coords[2] / triangle.points[2].w * gamma))
                / interpolated_reciprocal_w;

            if flip_v {
                p_uv.v = 1.0 - p_uv.v;
            }

            let color = texture.sample(p_uv) * triangle.color;

            if 1.0 - interpolated_reciprocal_w < depth_buffer.get(x, y) {
                color_buffer.set(x, y, color);
                depth_buffer.set(x, y, 1.0 - interpolated_reciprocal_w);
            }
        };

        self.rasterize_triangle(triangle, fill);
    }
}
