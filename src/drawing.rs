use crate::color::Color;
use crate::color_buffer::ColorBuffer;
use crate::depth_buffer::DepthBuffer;
use crate::texture::Texture;
use crate::triangle::{Triangle, Vertex};
use crate::vector::Vec4;
use crate::Vec2;

fn barycentric_weights(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> (f32, f32, f32) {
    let ac = c - a;
    let ab = b - a;
    let ac_cross_ab = ac.cross(ab);

    let pc = c - p;
    let pb = b - p;
    let pc_cross_pb = pc.cross(pb);

    let alpha = pc_cross_pb / ac_cross_ab;

    let ap = p - a;
    let ac_cross_ap = ac.cross(ap);

    let beta = ac_cross_ap / ac_cross_ab;

    let gamma = 1.0 - alpha - beta;

    (alpha, beta, gamma)
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
            triangle.points[0].x as u16,
            triangle.points[0].y as u16,
            triangle.points[1].x as u16,
            triangle.points[1].y as u16,
            color,
        );

        // B -> C
        self.draw_line(
            triangle.points[1].x as u16,
            triangle.points[1].y as u16,
            triangle.points[2].x as u16,
            triangle.points[2].y as u16,
            color,
        );

        // C -> A
        self.draw_line(
            triangle.points[2].x as u16,
            triangle.points[2].y as u16,
            triangle.points[0].x as u16,
            triangle.points[0].y as u16,
            color,
        );
    }

    fn draw_color_and_depth(
        &mut self,
        a: Vec4,
        b: Vec4,
        c: Vec4,
        p: Vec2,
        color: Color,
        depth_buffer: &mut DepthBuffer,
    ) {
        let (alpha, beta, gamma) =
            barycentric_weights(Vec2::from(a), Vec2::from(b), Vec2::from(c), p);

        let interpolated_reciprocal_w = 1.0 / a.w * alpha + 1.0 / b.w * beta + 1.0 / c.w * gamma;

        if 1.0 - interpolated_reciprocal_w < depth_buffer.get(p.x as u16, p.y as u16) {
            self.set(p.x as u16, p.y as u16, color);
            depth_buffer.set(p.x as u16, p.y as u16, 1.0 - interpolated_reciprocal_w);
        }
    }

    pub fn draw_filled_triangle(
        &mut self,
        triangle: &Triangle,
        color: Color,
        depth_buffer: &mut DepthBuffer,
    ) {
        let (min_x, min_y, max_x, max_y) = triangle.bounding_box();

        for y in min_y as u16..=max_y as u16 {
            for x in min_x as u16..=max_x as u16 {
                let p = Vec2::new(x as f32, y as f32);

                if triangle.point_inside(p) {
                    self.draw_color_and_depth(
                        triangle.points[0],
                        triangle.points[1],
                        triangle.points[2],
                        p,
                        color,
                        depth_buffer,
                    )
                }
            }
        }
    }

    fn draw_texel(
        &mut self,
        a: Vertex,
        b: Vertex,
        c: Vertex,
        p: Vec2,
        texture: &Texture,
        depth_buffer: &mut DepthBuffer,
        flip_v: bool,
        triangle_color: Color,
    ) {
        let (alpha, beta, gamma) =
            barycentric_weights(Vec2::from(a.pos), Vec2::from(b.pos), Vec2::from(c.pos), p);

        let interpolated_reciprocal_w =
            1.0 / a.pos.w * alpha + 1.0 / b.pos.w * beta + 1.0 / c.pos.w * gamma;
        let mut p_uv =
            ((a.uv / a.pos.w * alpha) + (b.uv / b.pos.w * beta) + (c.uv / c.pos.w * gamma))
                / interpolated_reciprocal_w;

        if flip_v {
            p_uv.v = 1.0 - p_uv.v;
        }

        let color = texture.sample(p_uv) * triangle_color;

        if 1.0 - interpolated_reciprocal_w < depth_buffer.get(p.x as u16, p.y as u16) {
            self.set(p.x as u16, p.y as u16, color);
            depth_buffer.set(p.x as u16, p.y as u16, 1.0 - interpolated_reciprocal_w);
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

        for y in min_y as u16..=max_y as u16 {
            for x in min_x as u16..=max_x as u16 {
                let p = Vec2::new(x as f32, y as f32);

                if triangle.point_inside(p) {
                    self.draw_texel(
                        Vertex {
                            pos: triangle.points[0],
                            uv: triangle.tex_coords[0],
                        },
                        Vertex {
                            pos: triangle.points[1],
                            uv: triangle.tex_coords[1],
                        },
                        Vertex {
                            pos: triangle.points[2],
                            uv: triangle.tex_coords[2],
                        },
                        p,
                        texture,
                        depth_buffer,
                        flip_v,
                        triangle.color,
                    )
                }
            }
        }
    }
}
