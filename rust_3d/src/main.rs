use macroquad::prelude::*;

fn draw_point(x: f32, y: f32, _z: f32) {
    let color : Color = WHITE;
    draw_circle(x, y, 1.0, color);
}

#[macroquad::main("My Sketch")]
async fn main() {
    let mut x = 100.0;
    let mut speed = 2.0;

    loop {
        clear_background(BLACK);

        draw_point(x, screen_height() / 2.0, 0.0);

        // x += speed;
        // if x > screen_width() || x < 0.0 {
        //     speed = -speed;
        // }

        next_frame().await;
    }
}

