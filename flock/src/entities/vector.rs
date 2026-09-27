use macroquad::rand::gen_range;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x:f32, // row
    pub y:f32, // col
}

impl Vector {
    // Creates a vector that points to somewhere
    pub fn new(x:f32, y:f32) -> Self {
        Self { x, y }
    }

    pub fn new_random() -> Self {
        let mut v = Self { x: 0.0, y: 0.0 };
        v.randomize();
        v
    }

    pub fn randomize(&mut self) {
        self.x = gen_range(-1.0, 1.0);
        self.y = gen_range(-1.0, 1.0);
    }

    pub fn add(&mut self, other:Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn scalar_mult(&mut self, mag: f32) {
        self.x *= mag;
        self.y *= mag;
    }

    pub fn scalar_divide(&mut self, mag: f32) {
        self.x /= mag;
        self.y /= mag;
    }

    pub fn distance(&self, other: Vector) -> f32 {
        let side_a = other.x - self.x;
        let side_b = other.y - self.y;

        return (side_a * side_a + side_b * side_b).sqrt();
    }

    // Normalize vector
    pub fn normalize(&mut self) {
        let t = (self.x * self.x + self.y * self.y).sqrt();
        if(t != 0.0) {
            self.x /= t;
            self.y /= t;
        }
    }
}
