pub mod cube_samples {
    use cube::*;
    use std::{
        thread::sleep,
        time::Duration,
    };

    pub fn sample_animation_simple(frame_delay_ms: u64) {
        let mut cube = Cube::new(20, 100);    
        let mut screen = Screen::new(160, 44, 0.6);
        let mut point = Point::new();
    
        print!("\x1b[2J");
    
        loop {
            cube.render_frame(&mut screen, &mut point);
            screen.print_frame();
            cube.rotate(0.05, 0.05, 0.01);
            screen.reset_buffers();
            sleep(Duration::from_millis(frame_delay_ms))
        }
    }

    pub fn sample_animation_complex(frame_delay_ms: u64) {
        let mut cube_main = Cube::new(20, 100);
    
        let mut cube_one = Cube::new(7, 100);
        cube_one.set_offset(-65.0, -15.0);
    
        let mut cube_two = Cube::new(7, 100);
        cube_two.set_offset(65.0, -15.0);
    
        let mut cube_three = Cube::new(7, 100);
        cube_three.set_offset(-65.0, 15.0);
    
        let mut cube_four = Cube::new(7, 100);
        cube_four.set_offset(65.0, 15.0);
    
    
        let mut screen = Screen::new(160, 44, 0.6);
        let mut point = Point::new();
    
        print!("\x1b[2J");
    
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

    pub fn sample_animation_spins(frame_delay_ms: u64) {
        let mut cube_one = Cube::new(15, 100);
        cube_one.set_offset(-50.0, 0.0);

        let mut cube_two = Cube::new(15, 100);
        cube_two.set_offset(0.0, 0.0);  // Not necessary, but consistent

        let mut cube_three = Cube::new(15, 100);
        cube_three.set_offset(50.0, 0.0);

        let mut screen = Screen::new(160, 44, 0.6);
        let mut point = Point::new();
    
        print!("\x1b[2J");

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
}