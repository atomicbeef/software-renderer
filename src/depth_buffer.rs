pub struct DepthBuffer {
    buffer: Vec<f32>,
    width: usize,
    height: usize,
}

impl DepthBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self { buffer: vec![1.0; width * height], width, height }
    }

    pub fn clear(&mut self, depth: f32) {
        self.buffer.fill(depth);
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self. width && y < self.height {
            self.buffer[self.width * y + x]
        } else {
            1.0
        }
    }

    pub fn set(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            self.buffer[self.width * y + x] = depth;
        }
    }
}