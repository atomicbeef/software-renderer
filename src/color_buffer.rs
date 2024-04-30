use crate::color::Color;

pub struct ColorBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl ColorBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color.into());
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.buffer[self.width * y + x] = color.into();
    }
}
