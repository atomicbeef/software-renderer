use minifb::{Key, Window, WindowOptions};

struct ColorBuffer {
    pub buffer: Vec<u32>,
}

impl ColorBuffer {
    fn new(width: usize, height: usize) -> Self {
        ColorBuffer { buffer: vec![0; width * height ] }
    }

    fn clear(&mut self, color: u32) {
        for c in self.buffer.iter_mut() { *c = color; }
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let mut buffer = ColorBuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "3D Renderer",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    )
    .expect("Error: Window could not be created!");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer.buffer, WIDTH, HEIGHT)
            .unwrap();
        
        buffer.clear(0x000000FF);
    }
}
