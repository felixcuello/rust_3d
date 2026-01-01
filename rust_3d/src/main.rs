use macroquad::prelude::*;

/*
 * COORDINATE SYSTEM
 *
 * We want the coordinate system to be like in mathematics:
 *
 *                      A (1,0)
 *                      |
 *                      |
 *                      |
 *                      | (0,0)
 * (0,-1) <-------------+---------------> (0,1)
 *                      |
 *                      |
 *                      |
 *                      | (-1,0)
 *                      V
 *
 */


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


// Project Point: projects a 3D point onto a 2D screen
// -------------------------------------------------------------
fn project_point(x: f32, y: f32, z:f32) -> (f32, f32) {
    // Project
    let x = x / z;
    let y = y / z;

    // Convert to the mathematics coordinate system
    let x = (x + 1.0) / 2.0 * screen_width();
    let y = (1.0 - (y + 1.0) / 2.0) * screen_height();

    (x, y)
}


// Draw Point: draws a point on a 3D coordinate system
// -------------------------------------------------------------
fn draw_point_3d(x: f32, y: f32, z: f32) {
    let color : Color = WHITE;

    let (x, y) = project_point(x, y, z);

    // TODO: Adjust radius based on the Z distance
    let radius = (8.0 - 3.0*z).clamp(0.3, 10.0);

    draw_circle(x, y, radius, color);
}


// Draw Coordinate System: draws the X and Y axes
// -------------------------------------------------------------
fn draw_coordinate_system() {
    let color = Color::from_rgba(0, 0, 180, 255);

    draw_line(0.0, screen_height() / 2.0, screen_width(), screen_height() / 2.0, 1.0, color);
    draw_line(screen_width() / 2.0, 0.0, screen_width() / 2.0, screen_height(), 1.0, color);
}

// Rotate Point: rotates a 3D point around the Z-axis
// -------------------------------------------------------------
fn rotate_point_xz(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let x_rotated = x * cos_a - z * sin_a;
    let z_rotated = x * sin_a + z * cos_a;

    (x_rotated, y, z_rotated)
}

// Rotate Point: rotates a 3D point around the Z-axis
// -------------------------------------------------------------
fn rotate_point_yz(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let y_rotated = y * cos_a - z * sin_a;
    let z_rotated = y * sin_a + z * cos_a;

    (x, y_rotated, z_rotated)
}

// Rotate Point: rotates a 3D point around the Z-axis
// -------------------------------------------------------------
fn rotate_point_yx(x: f32, y: f32, z: f32, angle: f32) -> (f32, f32, f32) {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let y_rotated = y * cos_a - x * sin_a;
    let x_rotated = y * sin_a + x * cos_a;

    (x_rotated, y_rotated, z)
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut dz = 1.0;

    let v: Vec<(f32, f32, f32)> = vec![
        (0.50, 0.50, 0.50),
        (-0.50, 0.50, 0.50),
        (-0.50, -0.50, 0.50),
        (0.50, -0.50, 0.50),

        (0.50, 0.50, -0.50),
        (-0.50, 0.50, -0.50),
        (-0.50, -0.50, -0.50),
        (0.50, -0.50, -0.50),
    ];

    let mut angle:f32 = 0.0;

    loop {
        clear_background(BLACK);

        //draw_coordinate_system();

        // dz = dz * 1.001;
        angle = angle + 0.02;

        for (x, y, z) in v.iter() {
            let (x,y,z) = (*x, *y, *z);
            let (x, y, z) = rotate_point_xz(x, y, z, angle);
            let (x, y, z) = rotate_point_yz(x, y, z, angle);
            // let (x, y, z) = rotate_point_yx(x, y, z, angle);
            let (x_world, y_world, z_world) = (x + 0.0, y + 0.0, z + 2.0);
            draw_point_3d(x_world, y_world, z_world);
        }

        next_frame().await;
    }
}

