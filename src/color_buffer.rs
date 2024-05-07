use crate::color::Color;

pub struct ColorBuffer {
    buffer: Vec<u32>,
    width: u16,
    height: u16,
}

impl ColorBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: vec![0; width as usize * height as usize],
            width,
            height,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color.into());
    }

    pub fn set(&mut self, x: u16, y: u16, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.buffer[self.width as usize * y as usize + x as usize] = color.into();
    }
}
