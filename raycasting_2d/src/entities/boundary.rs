use macroquad::prelude::*;
use super::vector::Vector; 

pub struct Boundary {
    pub a: Vector,
    pub b: Vector,
}

impl Boundary {
    pub fn new(a: Vector, b: Vector) -> Self {
       Self { a, b }
    }

    pub fn show(&self) {
        let thickness = 1.0; // thickness for this line
        let color = WHITE;

        draw_line(self.a.x, self.a.y, self.b.x, self.b.y, thickness, color);
    }
}
