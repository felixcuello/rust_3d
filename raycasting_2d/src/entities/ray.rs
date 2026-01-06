use std::f32::consts::PI;
use macroquad::prelude::*;
use super::vector::Vector;
use super::boundary::Boundary;

pub struct Ray {
    pub pos: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(pos: Vector, angle: i32) -> Self {
        let angle_in_radians = (angle as f32) * PI / 180.0;

        let dir = Vector::new(angle_in_radians.cos(), angle_in_radians.sin());

        Ray { pos, dir }
    }

    pub fn show(&self) {
        let thickness = 1.0;
        let color = YELLOW;

        draw_line(
            self.pos.x,
            self.pos.y,
            self.pos.x + self.dir.x * 10.0,
            self.pos.y + self.dir.y * 10.0,
            thickness,
            color
        );
    }

    pub fn look_at(&mut self, point: Vector) {
        let mut new_direction = Vector::new(point.x - self.pos.x, point.y - self.pos.y);
        new_direction.normalize();

        self.dir.x = new_direction.x;
        self.dir.y = new_direction.y;
    }

    // Everything here is explained in:
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    pub fn cast(&self, boundary: &Boundary) -> Option<Vector> {
        let x1 = boundary.a.x;
        let y1 = boundary.a.y;
        let x2 = boundary.b.x;
        let y2 = boundary.b.y;

        let x3 = self.pos.x;
        let y3 = self.pos.y;
        let x4 = self.pos.x + self.dir.x;
        let y4 = self.pos.y + self.dir.y;

        let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if den == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if t > 0.0 && t < 1.0 && u > 0.0 {
            let cast_x = x1 + t * (x2 - x1);
            let cast_y = y1 + t * (y2 - y1);
            return Some(Vector::new(cast_x, cast_y));
        } else {
            return None;
        }
    }
}
