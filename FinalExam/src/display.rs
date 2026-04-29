pub struct Display {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u8>,
}

impl Display {
    pub fn new(width: u32, height: u32) -> Self {
        Display {
            width,
            height,
            pixels: vec![0x00, 0x00, 0x00, 0xff].repeat((width * height) as usize),
        }
    }

    pub fn blit(&self, frame: &mut [u8]) {
        frame.copy_from_slice(&self.pixels);
    }

    fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let idx = (y as usize * self.width as usize + x as usize) * 4;
        self.pixels[idx]     = r;
        self.pixels[idx + 1] = g;
        self.pixels[idx + 2] = b;
        self.pixels[idx + 3] = 0xff;
    }

    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let steps = dx.abs().max(dy.abs()) as usize;

    if steps == 0 {
        self.set_pixel(x1 as i32, y1 as i32, 0x00, 0xff, 0x00);
        return;
    }

    let x_step = dx / steps as f64;
    let y_step = dy / steps as f64;

    let mut x = x1;
    let mut y = y1;

    for _ in 0..=steps {
        self.set_pixel(x as i32, y as i32, 0x00, 0xff, 0x00);
        x += x_step;
        y += y_step;
    }
}
}