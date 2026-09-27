use macroquad::prelude::*;
use macroquad::miniquad::date::now;
use macroquad::rand::srand;

mod entities;

use entities::boid::Boid;
use entities::vector::Vector;

//  Window Configuration for Macroquad
// -------------------------------------------------------------
fn window_configuration() -> Conf {
    Conf {
        window_title: "Rust 3D".to_string(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

//    REFERENCES:
//    let color = Color::from_rgba(0, 0, 180, 255);
//    draw_circle(x, y, radius, color);
//    draw_line(0.0, screen_height() / 2.0, screen_width(), screen_height() / 2.0, 1.0, color);
//    let cos_a = angle.cos();
//    let sin_a = angle.sin();

#[macroquad::main(window_configuration)]
async fn main() {
    srand(now() as u64); // seed the random generator

    let mut flock:Vec<Boid> = vec![];

    for _i in 1..10 {
        let position = Vector::new(screen_width() / 2.0, screen_height() / 2.0);
        let velocity = Vector::new_random();
        let acceleration = Vector::new(0.0, 0.0);
        let boid = Boid::new(position, velocity, acceleration);

        flock.push(boid);
    }

    let immutable_flock = flock.clone();

    loop {
        clear_background(BLACK);

        for boid in &mut flock {
            // boid.align(&immutable_flock);
            boid.update();
            boid.show();
        }

        next_frame().await;
    }
}

