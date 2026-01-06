use macroquad::prelude::*;
use macroquad::rand::{srand, gen_range};

mod entities;

use entities::boundary::Boundary;
use entities::vector::Vector;
use entities::particle::Particle;

//  Window Configuration for Macroquad
// -------------------------------------------------------------
fn window_configuration() -> Conf {
    Conf {
        window_title: "Raycasting 2D".to_string(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut walls = Vec::new();
    srand(macroquad::miniquad::date::now() as u64);

    for _i in 0..5 {
        let x1 = gen_range(0.0, screen_width() as f32);
        let y1 = gen_range(0.0, screen_height() as f32);
        let x2 = gen_range(0.0, screen_width() as f32);
        let y2 = gen_range(0.0, screen_height() as f32);

        walls.push(Boundary::new(Vector::new(x1, y1), Vector::new(x2, y2)));
    }

    let x1 = 0.0;
    let y1 = 0.0;
    let x2 = screen_width() + 1.0;
    let y2 = 0.0;
    let x3 = 0.0;
    let y3 = screen_height() + 1.0;
    let x4 = screen_width() + 1.0;
    let y4 = screen_height() + 1.0;

    walls.push(Boundary::new(Vector::new(x1,y1), Vector::new(x2,y2)));
    walls.push(Boundary::new(Vector::new(x1,y1), Vector::new(x3,y3)));
    walls.push(Boundary::new(Vector::new(x2,y2), Vector::new(x4,y4)));
    walls.push(Boundary::new(Vector::new(x3,y3), Vector::new(x4,y4)));

    // let mut ray = Ray::new(Vector::new(200.0, 300.0), 0);
    let mut particle = Particle::new();

    loop {
        clear_background(BLACK);

        let (mouse_x, mouse_y) = mouse_position();
        particle.update(mouse_x, mouse_y);
        particle.show();

        for wall in &walls {
            wall.show();
        }

        particle.look(&walls);

        next_frame().await;
    }
}

