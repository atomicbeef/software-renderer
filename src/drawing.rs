use crate::color::Color;
use crate::color_buffer::ColorBuffer;
use crate::texture::{Texture, Tex2};
use crate::triangle::Triangle;
use crate::Vec2;

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

    fn draw_flat_bottom_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color) {
        let slope_left = (b.x - a.x) / (b.y - a.y);
        let slope_right = (c.x - a.x) / (c.y - a.y);

        let mut start_x = a.x;
        let mut end_x = a.x;

        for y in a.y as usize..c.y as usize + 1 {
            self.draw_line(start_x as usize, y, end_x as usize, y, color);

            start_x += slope_left;
            end_x += slope_right;

            // Prevent wide, short triangles from being drawn too wide
            if (end_x - start_x).abs() > (c.x - b.x).abs() {
                start_x = b.x;
                end_x = c.x;
            }
        }
    }

    fn draw_flat_top_triangle(&mut self, a: Vec2, b: Vec2, c: Vec2, color: Color) {
        let slope_left = (a.x - c.x) / (a.y - c.y);
        let slope_right = (b.x - c.x) / (b.y - c.y);

        let mut start_x = c.x;
        let mut end_x = c.x;

        for y in (a.y as usize..c.y as usize + 1).rev() {
            self.draw_line(start_x as usize, y, end_x as usize, y, color);

            start_x -= slope_left;
            end_x -= slope_right;
            
            // Prevent wide, short triangles from being drawn too wide
            if (end_x - start_x).abs() > (b.x - a.x).abs() {
                start_x = a.x;
                end_x = b.x;
            }
        }
    }

    pub fn draw_filled_triangle(&mut self, triangle: &Triangle, color: Color) {
        let mut points = triangle.points.clone();
        points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        if points[1].y == points[2].y {
            self.draw_flat_bottom_triangle(points[0], points[1], points[2], color);
        } else if points[0].y == points[1].y {
            self.draw_flat_top_triangle(points[0], points[1], points[2], color);
        } else {
            let midpoint = Vec2::new(
                (points[2].x - points[0].x) * (points[1].y - points[0].y) / (points[2].y - points[0].y) + points[0].x,
                points[1].y
            );
    
            self.draw_flat_bottom_triangle(points[0], points[1], midpoint, color);
            self.draw_flat_top_triangle(points[1], midpoint, points[2], color);
        }
    }

    pub fn draw_textured_triangle(&mut self, triangle: &Triangle, texture: &Texture) {
        #[derive(Debug)]
        struct Vertex {
            pos: Vec2,
            uv: Tex2,
        }

        // Floor vertices positions to prevent rendering artifacts
        let mut vertices = [
            Vertex { pos: triangle.points[0].floor(), uv: triangle.tex_coords[0] },
            Vertex { pos: triangle.points[1].floor(), uv: triangle.tex_coords[1] },
            Vertex { pos: triangle.points[2].floor(), uv: triangle.tex_coords[2] },
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
                    self.set(x, y, if x % 2 == 0 && y % 2 == 0 { Color::new(0, 0, 0) } else { Color::new(0, 0xFF, 0) });
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
                    self.set(x, y, if x % 2 == 0 && y % 2 == 0 { Color::new(0, 0, 0) } else { Color::new(0, 0xFF, 0) });
                }
            }
        }
    }
}
