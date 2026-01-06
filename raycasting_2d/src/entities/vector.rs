#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalize(&mut self) {
        let mag = (self.x * self.x + self.y * self.y).sqrt();
        if mag != 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }
}
