use std::{thread::sleep, time::Duration};

use libc::{putchar};

const CUBE_WIDTH: f32 = 20.0;
const WIDTH: f64 = 160.0;
const HEIGHT: f64 = 44.0;
const BACKGROUND_ASCII_CODE: i32 = 32; // equivalent to blank space (' ')

struct CalculationParams<'a> {
    x: &'a f32,
    y: &'a f32,
    z: &'a f32,
    a: &'a f64,
    b: &'a f64,
    c: &'a f64
}

fn calculate_x(data: &CalculationParams) -> f64 {
    f64::from(*data.y) * data.a.sin() * data.b.sin() * data.c.cos() - f64::from(*data.z) * data.a.cos() * data.b.sin() * data.c.cos()
    + f64::from(*data.y) * data.a.cos() * data.c.sin() + f64::from(*data.z) * data.a.sin() * data.c.sin()
    + f64::from(*data.x) * data.b.cos() * data.c.cos()
}

fn calculate_y(data: &CalculationParams) -> f64 {
    f64::from(*data.y) * data.a.cos() * data.c.cos()
    + f64::from(*data.z) * data.a.sin() * data.c.cos() - f64::from(*data.y) * data.a.sin() * data.b.sin() * data.c.sin()
    + f64::from(*data.z) * data.a.cos() * data.b.sin() * data.c.sin() - f64::from(*data.x) * data.b.cos() * data.c.sin()
}

fn calculate_z(data: &CalculationParams) -> f64 {
    f64::from(*data.z) * data.a.cos() * data.b.cos() - f64::from(*data.y) * data.a.sin() * data.b.cos() + f64::from(*data.x) * data.b.sin()
}

fn calculate_for_surface(data: CalculationParams, ascii_char: i32, z_buffer: &mut [f64], buffer: &mut [i32]) {
    const K1: f64 = 40.0;
    const DISTANCE_FROM_CAM: i32 = 100;

    let x = calculate_x(&data);
    let y = calculate_y(&data);
    let z = calculate_z(&data) + DISTANCE_FROM_CAM as f64;

    let ooz = 1.0 / z;

    let xp = (WIDTH / 2f64 - 2f64 * CUBE_WIDTH as f64 + K1 * ooz * x * 2f64) as i64;
    let yp = (HEIGHT / 2f64 + K1 * ooz * y) as i64;

    let idx = xp + yp * WIDTH as i64;

    if idx >= 0 && idx < (WIDTH * HEIGHT) as i64 && ooz > z_buffer[idx as usize] as f64 {
        z_buffer[idx as usize] = ooz;
        buffer[idx as usize] = ascii_char;
    }
}

fn main() {
    const INCREMENT_SPEED: f32 = 0.6;

    println!("\x1b[2J");

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let c: f64 = 0.0;

    loop {
        let mut buffer = vec![BACKGROUND_ASCII_CODE; (WIDTH*HEIGHT) as usize];
        let mut z_buffer = vec![0 as f64; (WIDTH*HEIGHT*4f64) as usize];
        let mut cube_x = -CUBE_WIDTH;
        
        while cube_x < CUBE_WIDTH {
            cube_x += INCREMENT_SPEED;

            let mut cube_y = -CUBE_WIDTH;

            while cube_y < CUBE_WIDTH {
                cube_y += INCREMENT_SPEED;

                calculate_for_surface(CalculationParams{ x: &cube_x, y: &cube_y, z: &-CUBE_WIDTH, a: &a, b: &b, c: &c }, 33, &mut z_buffer, &mut buffer);
                calculate_for_surface(CalculationParams{ x: &CUBE_WIDTH, y: &cube_y, z: &cube_x, a: &a, b: &b, c: &c }, 36, &mut z_buffer, &mut buffer);
                calculate_for_surface(CalculationParams{ x: &-CUBE_WIDTH, y: &cube_y, z: &-cube_x, a: &a, b: &b, c: &c }, 126, &mut z_buffer, &mut buffer);
                calculate_for_surface(CalculationParams{ x: &-cube_x, y: &cube_y, z: &CUBE_WIDTH, a: &a, b: &b, c: &c }, 35, &mut z_buffer, &mut buffer);
                calculate_for_surface(CalculationParams{ x: &cube_x, y: &-CUBE_WIDTH, z: &-cube_y, a: &a, b: &b, c: &c }, 38, &mut z_buffer, &mut buffer);
                calculate_for_surface(CalculationParams{ x: &cube_x, y: &CUBE_WIDTH, z: &cube_y, a: &a, b: &b, c: &c }, 43, &mut z_buffer, &mut buffer);
            }
        }

        println!("\x1b[H");
        for k in 0..=WIDTH as i32 * HEIGHT as i32 {
            unsafe {
                putchar(if k % WIDTH as i32 != 0 { buffer[k as usize] } else { 10 });
            }
        }

        a += 0.05;
        b += 0.05;

        sleep(Duration::from_millis(20));
    }
}
