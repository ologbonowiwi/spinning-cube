use std::thread::sleep_ms;

use libc::{putchar, usleep};


static mut A: f64 = 0.0;
static mut B: f64 = 0.0;
static mut C: f64 = 0.0;

const CUBE_WIDTH: f32 = 20.0;
const WIDTH: i32 = 160;
const HEIGHT: i32 = 44;
const BACKGROUND_ASCII_CODE: i32 = 32; // equivalent to blank space (' ')

fn calculate_x(x: &f32, y: &f32, z: &f32) -> f64 {
    unsafe {
        f64::from(*y) * A.sin() * B.sin() * C.cos() - f64::from(*z) * A.cos() * B.sin() * C.cos() + f64::from(*y) * A.cos() * C.sin() + f64::from(*z) * A.sin() * C.sin() + f64::from(*x) * B.cos() * C.cos()
    }
}

fn calculate_y(x: &f32, y: &f32, z: &f32) -> f64 {
    unsafe {
        f64::from(*y) * A.cos() * C.cos() + f64::from(*z) * A.sin() * C.cos() - f64::from(*y) * A.sin() * B.sin() * C.sin() + f64::from(*z) * A.cos() * B.sin() * C.sin() - f64::from(*x) * B.cos() * C.sin()
    }
}

fn calculate_z(x: &f32, y: &f32, z: &f32) -> f64 {
    unsafe {
        f64::from(*z) * A.cos() * B.cos() - f64::from(*y) * A.sin() * B.cos() + f64::from(*x) * B.sin()
    }
}

fn calculate_for_surface(cube_x: &f32, cube_y: &f32, cube_z: &f32, ascii_char: i32, z_buffer: &mut Vec<f64>, buffer: &mut Vec<i32>) {
    const K1: f64 = 40.0;
    const DISTANCE_FROM_CAM: i32 = 100;

    let x = calculate_x(cube_x, cube_y, cube_z);
    let y = calculate_y(cube_x, cube_y, cube_z);
    let z = calculate_z(cube_x, cube_y, cube_z) + DISTANCE_FROM_CAM as f64;

    let ooz = 1.0 / z;

    let xp = (WIDTH as f64 / 2 as f64 - 2 as f64 * CUBE_WIDTH as f64 + K1 * ooz * x * 2 as f64) as i64;
    let yp = (HEIGHT as f64 / 2 as f64 + K1 * ooz * y) as i64;

    let idx = xp + yp * WIDTH as i64;

    if idx >= 0 && idx < (WIDTH * HEIGHT) as i64 {
        if ooz > z_buffer[idx as usize] as f64 {
            z_buffer[idx as usize] = ooz;
            buffer[idx as usize] = ascii_char;
        }
    }
}

fn main() {
    const INCREMENT_SPEED: f32 = 0.6;

    println!("\x1b[2J");

    loop {
        let mut buffer = vec![BACKGROUND_ASCII_CODE; (WIDTH*HEIGHT) as usize];
        let mut z_buffer = vec![0 as f64; (WIDTH*HEIGHT*4) as usize];
        let mut cube_x = -CUBE_WIDTH;
        
        while cube_x < CUBE_WIDTH {
            cube_x += INCREMENT_SPEED;

            let mut cube_y = -CUBE_WIDTH;

            while cube_y < CUBE_WIDTH {
                cube_y += INCREMENT_SPEED;

                calculate_for_surface(&cube_x, &cube_y, &-CUBE_WIDTH, 33, &mut z_buffer, &mut buffer);
                calculate_for_surface(&CUBE_WIDTH, &cube_y, &cube_x, 36, &mut z_buffer, &mut buffer);
                calculate_for_surface(&-CUBE_WIDTH, &cube_y, &-cube_x, 126, &mut z_buffer, &mut buffer);
                calculate_for_surface(&-cube_x, &cube_y, &CUBE_WIDTH, 35, &mut z_buffer, &mut buffer);
                calculate_for_surface(&cube_x, &-CUBE_WIDTH, &-cube_y, 38, &mut z_buffer, &mut buffer);
                calculate_for_surface(&cube_x, &CUBE_WIDTH, &cube_y, 43, &mut z_buffer, &mut buffer);
            }
        }

        println!("\x1b[H");
        for k in 0..=WIDTH * HEIGHT {
            unsafe {
                putchar(if k % WIDTH != 0 { buffer[k as usize] } else { 10 });
            }
        }

        unsafe {
            A += 0.05;
            B += 0.05;
        }

        sleep_ms(20);
    }
}
