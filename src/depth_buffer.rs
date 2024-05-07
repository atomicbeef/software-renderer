pub struct DepthBuffer {
    buffer: Vec<f32>,
    width: u16,
    height: u16,
}

impl DepthBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: vec![1.0; width as usize * height as usize],
            width,
            height,
        }
    }

    pub fn clear(&mut self, depth: f32) {
        self.buffer.fill(depth);
    }

    pub fn get(&self, x: u16, y: u16) -> f32 {
        assert!(x < self.width);
        assert!(y < self.height);

        self.buffer[self.width as usize * y as usize + x as usize]
    }

    pub fn set(&mut self, x: u16, y: u16, depth: f32) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.buffer[self.width as usize * y as usize + x as usize] = depth;
    }
}
