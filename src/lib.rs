pub struct Cube{
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,

    size: i32,
    depth: i32,

    vertical_offset: f32,
    horizontal_offset: f32,
}

impl Cube {
    pub fn new(cube_size: i32, cube_depth: i32) -> Self {
        Cube {
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,

            size: cube_size,
            depth: cube_depth,
            
            horizontal_offset: 0.0,
            vertical_offset: 0.0,
        }
    }

    // Sets the positional offset of the cube on the screen, with (0, 0) being in the center.
    // Negative values move the cube up and left, while positive values move the cube down and right.
    pub fn set_offset(&mut self, horizontal_offset: f32, vertical_offset: f32) {
        self.horizontal_offset = horizontal_offset;
        self.vertical_offset = vertical_offset;
    }

    // Rotates the cube a set ammount. You can pass 0.0 to any of the parameters
    // to not rotate the cube along that axis.
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.x_rotation += x;
        self.y_rotation += y;
        self.z_rotation += z;
    }

    // Writes the cube's current position and rotation to the screen.
    // Should be called every frame.
    pub fn render_frame(&self, screen: &mut Screen, point_data: &mut Point) {
        let float_size: f32 = self.size as f32;
        let mut x_val: f32 = -float_size;
        let mut y_val: f32;

        while x_val < float_size {
            y_val = -float_size;

            while y_val < float_size {
                // You can change the characters used to render each side by
                // changing the char values passed to the functions here.
                self.render_surface(x_val, y_val, -float_size, '@', screen, point_data);    // Front face
                self.render_surface(-x_val, y_val, float_size, '#', screen, point_data);    // Back face

                self.render_surface(-float_size, y_val, -x_val, '~', screen, point_data);    // Left face
                self.render_surface(float_size, y_val, x_val, '$', screen, point_data);     // Right face

                self.render_surface(x_val, -float_size, -y_val, ';', screen, point_data);    // Top face
                self.render_surface(x_val, float_size, y_val, '+', screen, point_data);     // Bottom face

                y_val += screen.increment;
            }

            x_val += screen.increment;
        }
    }

    fn render_surface(&self, x: f32, y: f32, z: f32, surface_char: char, screen: &mut Screen, point_data: &mut Point) {
        point_data.x_coord = self.calculate_x(x, y, z);
        point_data.y_coord = self.calculate_y(x, y, z);
        point_data.z_coord = self.calculate_z(x, y, z) + (self.depth as f32);

        point_data.z_inverse = 1.0 / point_data.z_coord;

        point_data.x_projection = ((screen.width as f32) / 2.0 + self.horizontal_offset + 40.0 * point_data.z_inverse * point_data.x_coord * 2.0).trunc() as i32;
        point_data.y_projection = ((screen.height as f32) / 2.0 + self.vertical_offset + 40.0 * point_data.z_inverse * point_data.y_coord).trunc() as i32;

        // Only draws the point if it's not off the screen in the negative direction. 
        if point_data.x_projection.is_positive() && point_data.y_projection.is_positive() {

            // A `usize` type can't be negative, so the negative values are checked in the above `if` statement
            point_data.buffer_index = (point_data.x_projection as usize) + (point_data.y_projection as usize) * screen.width;

            // Only draws the point if it's not off the screen in the positive direction.
            if point_data.buffer_index < (screen.width * screen.height) {

                // Only daws the point if it's closer to the camera than the existing point there.
                if point_data.z_inverse > screen.z_buffer[point_data.buffer_index] {
                    screen.text_buffer[point_data.buffer_index] = surface_char;
                    screen.z_buffer[point_data.buffer_index] = point_data.z_inverse;
                }
            }
        }
    }

    fn calculate_x(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.x_rotation.sin() * self.y_rotation.sin() * self.z_rotation.cos() - k * self.x_rotation.cos() * self.y_rotation.sin() * self.z_rotation.cos() + j * self.x_rotation.cos() * self.z_rotation.sin() + k * self.x_rotation.sin() * self.z_rotation.sin() + i * self.y_rotation.cos() * self.z_rotation.cos()
    }

    fn calculate_y(&self, i: f32, j: f32, k: f32) -> f32 {
        j * self.x_rotation.cos() * self.z_rotation.cos() + k * self.x_rotation.sin() * self.z_rotation.cos() - j * self.x_rotation.sin() * self.y_rotation.sin() * self.z_rotation.sin() + k * self.x_rotation.cos() * self.y_rotation.sin() * self.z_rotation.sin() - i * self.y_rotation.cos() * self.z_rotation.sin()
    }

    fn calculate_z(&self, i: f32, j: f32, k: f32) -> f32 {
        k * self.x_rotation.cos() * self.y_rotation.cos() - j * self.x_rotation.sin() * self.y_rotation.cos() + i * self.y_rotation.sin()
    }
}

pub struct Point {
    pub x_coord: f32,
    pub y_coord: f32,
    pub z_coord: f32,
    pub z_inverse: f32,

    pub x_projection: i32,
    pub y_projection: i32,

    pub buffer_index: usize,
}

impl Point {
    pub fn new() -> Self {
        Point {
            x_coord: 0.0,
            y_coord: 0.0,
            z_coord: 0.0,
            z_inverse: 0.0,

            x_projection: 0,
            y_projection: 0,

            buffer_index: 0,
        }
    }
}

pub struct Screen {
    pub width: usize,
    pub height: usize,

    pub text_buffer: Vec<char>,
    pub z_buffer: Vec<f32>,

    pub increment: f32,
}

impl Screen {
    pub fn new(width: usize, height: usize, increment: f32) -> Self {
        print!("\x1b[2J");
        
        Screen {
            width,
            height,

            text_buffer: vec![' '; width * height],
            z_buffer: vec![0.0; width * height],

            increment,
        }
    }

    // Prints the frame to the terminal
    pub fn print_frame(&self) {
        print!("\x1b[H");

        for k in 0..(self.width * self.height) {
            // The buffers are linear, so linebreaks are inserted when the frame is printed.
            match k % self.width {
                0 => print!("\n"),
                1.. => print!("{}", self.text_buffer[k])
            }
        }
    }

    // Resets the buffers to blank the screen, preventing "burn-in" of the points closest to the camera
    pub fn reset_buffers(&mut self) {
        for i in 0..self.width * self.height {
            self.text_buffer[i] = ' ';
            self.z_buffer[i] = 0.0;
        }
    }
}