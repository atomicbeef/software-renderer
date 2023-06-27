use crate::color::Color;
use crate::color_buffer::ColorBuffer;
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
}
