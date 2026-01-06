use macroquad::prelude::*;

use super::vector::Vector;
use super::ray::Ray;
use super::boundary::Boundary;

pub struct Particle {
    pub pos: Vector,
    pub rays: Vec<Ray>,
}

impl Particle {
    pub fn new() -> Self {
        let pos = Vector::new(screen_width() / 2.0, screen_height() / 2.0);
        let mut rays = Vec::new();

        for angle in (0..360).step_by(1) {
            rays.push( Ray::new(pos, angle) )
        }

        Self { pos, rays }
    }

    pub fn show(&self) {
        let color = YELLOW;
        draw_circle(self.pos.x, self.pos.y, 2.0, color);

        for ray in &self.rays {
            ray.show();
        }
    }

    pub fn look(&self, walls: &Vec<Boundary>) {
        for ray in &self.rays {
            let mut closest_point: Option<Vector> = None;
            let mut min_distance = std::f32::MAX;

            for wall in walls {
                match ray.cast(&wall) {
                    Some(point) => {
                        let origin = vec2(self.pos.x, self.pos.y);
                        let destination = vec2(point.x, point.y);
                        let distance = (origin - destination).length();

                        if distance < min_distance {
                            min_distance = distance;
                            closest_point = Some(point);
                        }
                    }

                    None => {} // nothing to do
                }
            }

            if let Some(point) = closest_point {
                let translucent_yellow = Color::new(1.0, 1.0, 0.0, 0.2);  // 30% opacity
                draw_line(self.pos.x, self.pos.y, point.x, point.y, 1.0, translucent_yellow);
                draw_circle(point.x, point.y, 1.0, YELLOW);
            }
        }
    }

    pub fn update(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;

        for ray in &mut self.rays {
            ray.pos = self.pos;
        }
    }
}
