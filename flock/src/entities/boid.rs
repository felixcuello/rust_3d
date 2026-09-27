use macroquad::prelude::*;
use super::vector::Vector;

// The idea of this file is to implement what TheCodingTrain did (in rust) for Boids
// Which is also based on this: https://www.red3d.com/cwr/boids/

#[derive(Copy, Clone)]
pub struct Boid {
    pub position:Vector,
    pub velocity:Vector,
    pub acceleration:Vector,
}

impl Boid {
    pub fn new(mut position:Vector, mut velocity:Vector, mut acceleration:Vector) -> Self {
        Self { position, velocity, acceleration }
    }

    pub fn update(&mut self) {
        self.position.add(self.velocity);
        self.velocity.add(self.acceleration);
    }

    //  align
    //  This method is fundamental to align the direction of the boid based on the
    //  boids that are CLOSER to it
    pub fn align(&mut self, boids:&Vec<Boid>) {
        let perception_radius = 100.0;
        let mut average = Vector::new(0.0, 0.0);
        let mut total = 0;

        for other_boid in boids.iter() {
            let distance = self.position.distance(other_boid.position);

            if(distance > 0.0 && distance < perception_radius) {
                average.add(other_boid.velocity);
                total += 1;
            }
        }

        if(total > 0) {
            average.scalar_divide(total as f32);
            self.velocity = average;
        }
    }


    // -----------------------------------------------------------------------------
    //   Draw a Boid in the same way TheCodingTrain has the boids :-)
    //   explanation on how I ended up drawing the Boid at the end of the file
    // -----------------------------------------------------------------------------
    pub fn show(&self) {
        let color = WHITE;
        let width = 1.0;
        let size = 16.0;
        let length_width_ratio = 4.0;

        // let radius = 5.0;
        // draw_circle(self.position.x, self.position.y, radius, color);
        let x = self.position.x;
        let y = self.position.y;
        let vel_x = self.velocity.x;
        let vel_y = self.velocity.y;

        let mut direction = Vector::new(vel_x, vel_y);
        direction.normalize();
        direction.scalar_mult(size);

        let a = x + direction.x;
        let b = y + direction.y;

        // calculate (e,f)
        let mut br = Vector::new(-direction.y, direction.x);
        br.normalize();
        br.scalar_mult(size / length_width_ratio);
        br.add(self.position);
        let (e, f) = (br.x, br.y);

        // calculate (c,d)
        let mut bl = Vector::new(direction.y, -direction.x);
        bl.normalize();
        bl.scalar_mult(size / length_width_ratio);
        bl.add(self.position);

        let (c, d) = (bl.x, bl.y);

        // spine
        // let spine_color = YELLOW;
        // draw_line(x, y, a, b, width, spine_color);

        // triangle
        draw_line(a, b, c, d, width, color);
        draw_line(c, d, e, f, width, color);
        draw_line(e, f, a, b, width, color);
    }
}

/*

I want to make the flock to be a triangle instead of a point or a circle.
To do so I have to draw something like this first

|>

This is a triangle composed of 3 points:

         (a,b)
           O
          /|\
         / | \
        /  |  \
       O---*---O
  (c,d)  (x,y)  (e,f)


(1) FIND (a,b) - point of the boid
The position is (x,y), and other points will be rotated based on the velocity vector which points to the
direction the Boid is moving at the moment. So, first of all let's draw a yellow line between the position
and the position + velocity (normalized) multiplied by 10.0 (that's going to be the size of the boid)


(2) FIND (c,d) -> (e,f)
To find this, we need to find a perpendicular vector of (x,y) -> (a,b) and then draw it. It should have
HALF the size of the (x,y) -> (a,b) vector, and the center must cross (x,y)

(2.1) HOW TO FIND THAT VECTOR?
A perpendicular vector to the (x,y) is (-y,x) or (y, -x).

So if I create (-y, x), normalize it, multiply by 5.0 and sum (x,y) I think I have (e,f) so:

(e,f) = |(-y,x)| * size / 4.0 + (x,y)

(c,d) = |(y,-x)| * size / 4.0 + (x,y)


With all 3 points we have the boid 100% defined :-D

 */
