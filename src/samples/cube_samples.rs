use cube::*;
use std::{
    thread::sleep,
    time::Duration,
};

// Renders a single cube in the center of the screen, spinning along all three axis.
pub fn sample_animation_simple(frame_delay_ms: u64) {
    let mut cube = Cube::new(20, 100);    
    let mut screen = Screen::new(160, 44, 0.6);
    let mut point = Point::new();

    loop {
        cube.render_frame(&mut screen, &mut point);
        screen.print_frame();
        cube.rotate(0.05, 0.05, 0.01);
        screen.reset_buffers();
        sleep(Duration::from_millis(frame_delay_ms))
    }
}

// Renders five cubes: one large one in the center of the screen, and a small one
// in each of the four corners of the screen. Each cube rotates in a different direction.
pub fn sample_animation_complex(frame_delay_ms: u64) {
    // Center cube
    let mut cube_main = Cube::new(20, 100);

    // Top left cube
    let mut cube_one = Cube::new(7, 100);
    cube_one.set_offset(-65.0, -15.0);

    // Top right cube
    let mut cube_two = Cube::new(7, 100);
    cube_two.set_offset(65.0, -15.0);

    // Bottom left cube
    let mut cube_three = Cube::new(7, 100);
    cube_three.set_offset(-65.0, 15.0);

    // Bottom right cube
    let mut cube_four = Cube::new(7, 100);
    cube_four.set_offset(65.0, 15.0);


    let mut screen = Screen::new(160, 44, 0.6);
    let mut point = Point::new();

    loop {
        cube_main.render_frame(&mut screen, &mut point);
        cube_one.render_frame(&mut screen, &mut point);
        cube_two.render_frame(&mut screen, &mut point);
        cube_three.render_frame(&mut screen, &mut point);
        cube_four.render_frame(&mut screen, &mut point);

        screen.print_frame();

        cube_main.rotate(0.05, 0.05, 0.01);
        cube_one.rotate(-0.05, 0.05, 0.01);
        cube_two.rotate(0.05, -0.05, 0.01);
        cube_three.rotate(-0.05, 0.05, -0.01);
        cube_four.rotate(0.05, -0.05, -0.01);

        screen.reset_buffers();

        sleep(Duration::from_millis(frame_delay_ms))
    }
}

// Renders three cubes in a line in the middle of the screen.
// Each cube spins on only one of the three axis.
pub fn sample_animation_spins(frame_delay_ms: u64) {

    // Left cube
    let mut cube_one = Cube::new(15, 100);
    cube_one.set_offset(-50.0, 0.0);

    // Middle cube
    let mut cube_two = Cube::new(15, 100);
    // Offset is (0.0, 0.0) by default

    // Right cube
    let mut cube_three = Cube::new(15, 100);
    cube_three.set_offset(50.0, 0.0);

    let mut screen = Screen::new(160, 44, 0.6);
    let mut point = Point::new();

    loop {
        cube_one.render_frame(&mut screen, &mut point);
        cube_two.render_frame(&mut screen, &mut point);
        cube_three.render_frame(&mut screen, &mut point);

        screen.print_frame();

        cube_one.rotate(0.05, 0.0, 0.0);
        cube_two.rotate(0.0, 0.05, 0.0);
        cube_three.rotate(0.0, 0.0, 0.05);

        screen.reset_buffers();

        sleep(Duration::from_millis(frame_delay_ms))
    }
}

// Renders a single spinning cube that moves back and forth across the screen.
pub fn sample_animation_slide(frame_delay_ms: u64) {
    let mut cube = Cube::new(20, 100);
    let mut screen = Screen::new(160, 44, 0.6);
    let mut point = Point::new();

    let mut x_offset: f32 = 0.0;
    let movement_speed: f32 = 0.05;

    loop {
        cube.render_frame(&mut screen, &mut point);
        screen.print_frame();

        cube.rotate(0.05, 0.05, 0.01);

        x_offset += movement_speed;
        cube.set_offset(x_offset.sin() * 50.0, 0.0);

        screen.reset_buffers();
        sleep(Duration::from_millis(frame_delay_ms))
    }
}