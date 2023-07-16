use crate::color::Color;
use crate::color_buffer::ColorBuffer;
use crate::depth_buffer::DepthBuffer;
use crate::texture::Texture;
use crate::triangle::{Triangle, Vertex};
use crate::Vec2;
use crate::vector::Vec4;

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
    
    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for xi in 0..width {
            for yi in 0..height {
                self.set(x + xi, y + yi, color);
            }
        }
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let dx = x1 as isize - x0 as isize;
        let dy = y1 as isize - y0 as isize;

        let side_length = if dx.abs() >= dy.abs() { dx.abs() } else { dy.abs() };

        let x_inc = dx as f32 / side_length as f32;
        let y_inc = dy as f32 / side_length as f32;

        let mut x = x0 as f32;
        let mut y = y0 as f32;

        for _ in 0..side_length {
            self.set(x.round() as usize, y.round() as usize, color);
            x += x_inc;
            y += y_inc;
        }
    }

    pub fn draw_triangle(&mut self, triangle: &Triangle, color: Color) {
        // A -> B
        self.draw_line(
            triangle.points[0].x as usize,
            triangle.points[0].y as usize,
            triangle.points[1].x as usize,
            triangle.points[1].y as usize,
            color
        );

        // B -> C
        self.draw_line(
            triangle.points[1].x as usize,
            triangle.points[1].y as usize,
            triangle.points[2].x as usize,
            triangle.points[2].y as usize,
            color
        );

        // C -> A
        self.draw_line(
            triangle.points[2].x as usize,
            triangle.points[2].y as usize,
            triangle.points[0].x as usize,
            triangle.points[0].y as usize,
            color
        );
    }

    fn draw_color_and_depth(
        &mut self,
        a: Vec4,
        b: Vec4,
        c: Vec4,
        p: Vec2,
        color: Color,
        depth_buffer: &mut DepthBuffer
    ) {
        let (alpha, beta, gamma) = barycentric_weights(
            Vec2::from(a),
            Vec2::from(b),
            Vec2::from(c),
            p
        );

        let interpolated_reciprocal_w = 1.0 / a.w * alpha + 1.0 / b.w * beta + 1.0 / c.w * gamma;

        if 1.0 - interpolated_reciprocal_w < depth_buffer.get(p.x as usize, p.y as usize) {
            self.set(p.x as usize, p.y as usize, color);
            depth_buffer.set(p.x as usize, p.y as usize, 1.0 - interpolated_reciprocal_w);
        }
    }

    pub fn draw_filled_triangle(&mut self, triangle: &Triangle, color: Color, depth_buffer: &mut DepthBuffer) {
        // Floor vertex x and y components to align to pixels and prevent rendering artifacts
        let mut vertices = [
            Vec4::new(
                triangle.points[0].x.floor(),
                triangle.points[0].y.floor(),
                triangle.points[0].z,
                triangle.points[0].w
            ),
            Vec4::new(
                triangle.points[1].x.floor(),
                triangle.points[1].y.floor(),
                triangle.points[1].z,
                triangle.points[1].w
            ),
            Vec4::new(
                triangle.points[2].x.floor(),
                triangle.points[2].y.floor(),
                triangle.points[2].z,
                triangle.points[2].w
            ),
        ];

        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        // Render flat bottom triangle
        let flat_bottom_inverse_slope_1 = if vertices[1].y - vertices[0].y != 0.0 {
            (vertices[1].x - vertices[0].x) / (vertices[1].y - vertices[0].y).abs()
        } else {
            0.0
        };

        let flat_bottom_inverse_slope_2 = if vertices[2].y - vertices[0].y != 0.0 {
            (vertices[2].x - vertices[0].x) / (vertices[2].y - vertices[0].y).abs()
        } else {
            0.0
        };

        if vertices[1].y - vertices[0].y != 0.0 {
            for y in vertices[0].y as usize..=vertices[1].y as usize {
                let x0 = vertices[1].x + (y as f32 - vertices[1].y) * flat_bottom_inverse_slope_1;
                let x1 = vertices[0].x + (y as f32 - vertices[0].y) * flat_bottom_inverse_slope_2;

                let x_start = if x0 < x1 { x0 } else { x1 };
                let x_end = if x1 > x0 { x1 } else { x0 };

                for x in x_start as usize..=x_end as usize {
                    self.draw_color_and_depth(
                        vertices[0],
                        vertices[1],
                        vertices[2],
                        Vec2::new(x as f32, y as f32),
                        color,
                        depth_buffer
                    );
                }
            }
        }
        
        // Render flat top triangle
        let flat_top_inverse_slope_1 = if vertices[2].y - vertices[1].y != 0.0 {
            (vertices[2].x - vertices[1].x) / (vertices[2].y - vertices[1].y).abs()
        } else {
            0.0
        };

        let flat_top_inverse_slope_2 = if vertices[2].y - vertices[0].y != 0.0 {
            (vertices[2].x - vertices[0].x) / (vertices[2].y - vertices[0].y).abs()
        } else {
            0.0
        };

        if vertices[2].y - vertices[1].y != 0.0 {
            for y in vertices[1].y as usize..=vertices[2].y as usize {
                let x0 = vertices[1].x + (y as f32 - vertices[1].y) * flat_top_inverse_slope_1;
                let x1 = vertices[0].x + (y as f32 - vertices[0].y) * flat_top_inverse_slope_2;

                let x_start = if x0 < x1 { x0 as usize } else { x1 as usize };
                let x_end = if x1 > x0 { x1 as usize } else { x0 as usize };

                for x in x_start..=x_end {
                    self.draw_color_and_depth(
                        vertices[0],
                        vertices[1],
                        vertices[2],
                        Vec2::new(x as f32, y as f32),
                        color,
                        depth_buffer
                    );
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
        flip_v: bool
    ) {
        let (alpha, beta, gamma) = barycentric_weights(
            Vec2::from(a.pos),
            Vec2::from(b.pos),
            Vec2::from(c.pos),
            p
        );

        let interpolated_reciprocal_w = 1.0 / a.pos.w * alpha + 1.0 / b.pos.w * beta + 1.0 / c.pos.w * gamma;
        let mut p_uv = ((a.uv / a.pos.w * alpha) + (b.uv / b.pos.w * beta) + (c.uv / c.pos.w * gamma)) / interpolated_reciprocal_w;

        if flip_v {
            p_uv.v = 1.0 - p_uv.v;
        }

        let color = texture.sample(p_uv);

        if 1.0 - interpolated_reciprocal_w < depth_buffer.get(p.x as usize, p.y as usize) {
            self.set(p.x as usize, p.y as usize, color);
            depth_buffer.set(p.x as usize, p.y as usize, 1.0 - interpolated_reciprocal_w);
        }
    }

    pub fn draw_textured_triangle(
        &mut self,
        triangle: &Triangle,
        texture: &Texture,
        depth_buffer: &mut DepthBuffer,
        flip_v: bool
    ) {
        // Floor vertex x and y components to align to pixels and prevent rendering artifacts
        let mut vertices = [
            Vertex {
                pos: Vec4::new(
                    triangle.points[0].x.floor(),
                    triangle.points[0].y.floor(),
                    triangle.points[0].z,
                    triangle.points[0].w,
                ),
                uv: triangle.tex_coords[0]
            },
            Vertex {
                pos: Vec4::new(
                    triangle.points[1].x.floor(),
                    triangle.points[1].y.floor(),
                    triangle.points[1].z,
                    triangle.points[1].w,
                ),
                uv: triangle.tex_coords[1]
            },
            Vertex {
                pos: Vec4::new(
                    triangle.points[2].x.floor(),
                    triangle.points[2].y.floor(),
                    triangle.points[2].z,
                    triangle.points[2].w,
                ),
                uv: triangle.tex_coords[2]
            },
        ];

        vertices.sort_by(|a, b| a.pos.y.partial_cmp(&b.pos.y).unwrap());

        // Render flat bottom triangle
        let flat_bottom_inverse_slope_1 = if vertices[1].pos.y - vertices[0].pos.y != 0.0 {
            (vertices[1].pos.x - vertices[0].pos.x) / (vertices[1].pos.y - vertices[0].pos.y).abs()
        } else {
            0.0
        };

        let flat_bottom_inverse_slope_2 = if vertices[2].pos.y - vertices[0].pos.y != 0.0 {
            (vertices[2].pos.x - vertices[0].pos.x) / (vertices[2].pos.y - vertices[0].pos.y).abs()
        } else {
            0.0
        };

        if vertices[1].pos.y - vertices[0].pos.y != 0.0 {
            for y in vertices[0].pos.y as usize..=vertices[1].pos.y as usize {
                let x0 = vertices[1].pos.x + (y as f32 - vertices[1].pos.y) * flat_bottom_inverse_slope_1;
                let x1 = vertices[0].pos.x + (y as f32 - vertices[0].pos.y) * flat_bottom_inverse_slope_2;

                let x_start = if x0 < x1 { x0 } else { x1 };
                let x_end = if x1 > x0 { x1 } else { x0 };

                for x in x_start as usize..=x_end as usize {
                    self.draw_texel(
                        vertices[0],
                        vertices[1],
                        vertices[2],
                        Vec2::new(x as f32, y as f32),
                        texture,
                        depth_buffer,
                        flip_v
                    )
                }
            }
        }
        
        // Render flat top triangle
        let flat_top_inverse_slope_1 = if vertices[2].pos.y - vertices[1].pos.y != 0.0 {
            (vertices[2].pos.x - vertices[1].pos.x) / (vertices[2].pos.y - vertices[1].pos.y).abs()
        } else {
            0.0
        };

        let flat_top_inverse_slope_2 = if vertices[2].pos.y - vertices[0].pos.y != 0.0 {
            (vertices[2].pos.x - vertices[0].pos.x) / (vertices[2].pos.y - vertices[0].pos.y).abs()
        } else {
            0.0
        };

        if vertices[2].pos.y - vertices[1].pos.y != 0.0 {
            for y in vertices[1].pos.y as usize..=vertices[2].pos.y as usize {
                let x0 = vertices[1].pos.x + (y as f32 - vertices[1].pos.y) * flat_top_inverse_slope_1;
                let x1 = vertices[0].pos.x + (y as f32 - vertices[0].pos.y) * flat_top_inverse_slope_2;

                let x_start = if x0 < x1 { x0 as usize } else { x1 as usize };
                let x_end = if x1 > x0 { x1 as usize } else { x0 as usize };

                for x in x_start..=x_end {
                    self.draw_texel(
                        vertices[0],
                        vertices[1],
                        vertices[2],
                        Vec2::new(x as f32, y as f32),
                        texture,
                        depth_buffer,
                        flip_v
                    )
                }
            }
        }
    }
}
