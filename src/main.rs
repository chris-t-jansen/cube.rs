use std::{thread::sleep, time::Duration};

const FRAME_WIDTH: usize = 160;
const FRAME_HEIGHT: usize = 44;

const INCREMENT_SPEED: f32 = 0.6;

struct PublicVariables {
    pub A: f32,
    pub B: f32,
    pub C: f32,

    pub cube_width: f32,
    pub distance_from_cam: f32,
    pub horizontal_offset: f32,
    pub K1: f32,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub ooz: f32,
    pub xp: i32,
    pub yp: i32,
    pub idx: usize,
}

impl PublicVariables {
    fn new() -> PublicVariables {
        PublicVariables {
            A: f32::default(),
            B: f32::default(),
            C: f32::default(),

            cube_width: 20.0,
            distance_from_cam: 110.0,
            horizontal_offset: f32::default(),
            K1: 40.0,

            x: f32::default(),
            y: f32::default(),
            z: f32::default(),

            ooz: f32::default(),
            xp: i32::default(),
            yp: i32::default(),
            idx: usize::default(),
        }
    }
}

struct Buffers {
    pub buffer: [char; FRAME_WIDTH * FRAME_HEIGHT],
    pub z_buffer: [f32; FRAME_WIDTH * FRAME_HEIGHT],
}

impl Buffers {
    fn new() -> Buffers {
        Buffers {
            buffer: [' '; FRAME_WIDTH * FRAME_HEIGHT],
            z_buffer: [f32::default(); FRAME_WIDTH * FRAME_HEIGHT],
        }
    }
}

fn calculate_x(i: f32, j: f32, k: f32, vars: &PublicVariables) -> f32 {
    j * vars.A.sin() * vars.B.sin() * vars.C.cos() - k * vars.A.cos() * vars.B.sin() * vars.C.cos() + j * vars.A.cos() * vars.C.sin() + k * vars.A.sin() * vars.C.sin() + i * vars.B.cos() * vars.C.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, vars: &PublicVariables) -> f32 {
    j * vars.A.cos() * vars.C.cos() + k * vars.A.sin() * vars.C.cos() - j * vars.A.sin() * vars.B.sin() * vars.C.sin() + k * vars.A.cos() * vars.B.sin() * vars.C.sin() - i * vars.B.cos() * vars.C.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, vars: &PublicVariables) -> f32 {
    k * vars.A.cos() * vars.B.cos() - j * vars.A.sin() * vars.B.cos() + i * vars.B.sin()
}

fn calculate_for_surface(cube_x: f32, cube_y: f32, cube_z: f32, character: char, vars: &mut PublicVariables, buffers: &mut Buffers) {
    vars.x = calculate_x(cube_x, cube_y, cube_z, &vars);
    vars.y = calculate_y(cube_x, cube_y, cube_z, &vars);
    vars.z = calculate_z(cube_x, cube_y, cube_z, &vars) + vars.distance_from_cam;

    vars.ooz = 1.0 / vars.z;

    let xp: f32 = (FRAME_WIDTH as f32) / 2.0 + vars.horizontal_offset + vars.K1 * vars.ooz * vars.x * 2.0;
    vars.xp = xp.trunc() as i32;

    let yp: f32 = (FRAME_HEIGHT as f32) / 2.0 + vars.K1 * vars.ooz * vars.y;
    vars.yp = yp.trunc() as i32;

    if vars.xp.is_positive() && vars.yp.is_positive() {
        vars.idx = usize::try_from(vars.xp).expect("XP to USIZE failed") + usize::try_from(vars.yp).expect("YP to USIZE failed") * FRAME_WIDTH;
        if vars.idx < FRAME_WIDTH * FRAME_HEIGHT {
            if vars.ooz > buffers.z_buffer[vars.idx] {
                buffers.buffer[vars.idx] = character;
                buffers.z_buffer[vars.idx] = vars.ooz;
            }
        }
    }
}

fn main() {
    let mut vars = PublicVariables::new();
    let mut buffers: Buffers;

    print!("\x1b[2J");

    loop {
        buffers = Buffers::new();

        // First cube
        vars.cube_width = 20.0;
        vars.horizontal_offset = -2.0 * vars.cube_width;

        let mut cube_x = vars.cube_width * -1.0;
        while cube_x < vars.cube_width {
            let mut cube_y = vars.cube_width * -1.0;
            while cube_y < vars.cube_width {
                calculate_for_surface(cube_x, cube_y, -vars.cube_width, '@', &mut vars, &mut buffers);
                calculate_for_surface(vars.cube_width, cube_y, cube_x, '$', &mut vars, &mut buffers);
                calculate_for_surface(-vars.cube_width, cube_y, -cube_x, '~', &mut vars, &mut buffers);
                calculate_for_surface(-cube_x, cube_y, vars.cube_width, '#', &mut vars, &mut buffers);
                calculate_for_surface(cube_x, -vars.cube_width, -cube_y, ';', &mut vars, &mut buffers);
                calculate_for_surface(cube_x, vars.cube_width, cube_y, '+', &mut vars, &mut buffers);

                cube_y += INCREMENT_SPEED;
            }
            cube_x += INCREMENT_SPEED;
        }

        // Second cube
        vars.cube_width = 10.0;
        vars.horizontal_offset = 1.0 * vars.cube_width;
        cube_x = vars.cube_width * -1.0;
        while cube_x < vars.cube_width {
            let mut cube_y = vars.cube_width * -1.0;
            while cube_y < vars.cube_width {
                calculate_for_surface(cube_x, cube_y, -vars.cube_width, '@', &mut vars, &mut buffers);
                calculate_for_surface(vars.cube_width, cube_y, cube_x, '$', &mut vars, &mut buffers);
                calculate_for_surface(-vars.cube_width, cube_y, -cube_x, '~', &mut vars, &mut buffers);
                calculate_for_surface(-cube_x, cube_y, vars.cube_width, '#', &mut vars, &mut buffers);
                calculate_for_surface(cube_x, -vars.cube_width, -cube_y, ';', &mut vars, &mut buffers);
                calculate_for_surface(cube_x, vars.cube_width, cube_y, '+', &mut vars, &mut buffers);

                cube_y += INCREMENT_SPEED;
            }
            cube_x += INCREMENT_SPEED;
        }

        // Third cube
        vars.cube_width = 5.0;
        vars.horizontal_offset = 8.0 * vars.cube_width;
        
        cube_x = vars.cube_width * -1.0;
        while cube_x < vars.cube_width {
            let mut cube_y = vars.cube_width * -1.0;
            while cube_y < vars.cube_width {
                calculate_for_surface(cube_x, cube_y, -vars.cube_width, '@', &mut vars, &mut buffers);
                calculate_for_surface(vars.cube_width, cube_y, cube_x, '$', &mut vars, &mut buffers);
                calculate_for_surface(-vars.cube_width, cube_y, -cube_x, '~', &mut vars, &mut buffers);
                calculate_for_surface(-cube_x, cube_y, vars.cube_width, '#', &mut vars, &mut buffers);
                calculate_for_surface(cube_x, -vars.cube_width, -cube_y, ';', &mut vars, &mut buffers);
                calculate_for_surface(cube_x, vars.cube_width, cube_y, '+', &mut vars, &mut buffers);

                cube_y += INCREMENT_SPEED;
            }
            cube_x += INCREMENT_SPEED;
        }

        print!("\x1b[H");

        for k in 0..(FRAME_WIDTH * FRAME_HEIGHT) {
            print!("{}", if k % FRAME_WIDTH == 0 { '\n' } else { buffers.buffer[k] })
        }

        vars.A += 0.05;
        vars.B += 0.05;
        vars.C += 0.01;
        
        sleep(Duration::from_micros(16000));
    }
}
