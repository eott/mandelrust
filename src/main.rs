use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::cmp::min;

struct Complex {
    img: f64,
    real: f64,
}

impl Complex {
    fn new(i: f64, r: f64) -> Complex {
        Complex {
            img: i,
            real: r
        }
    }

    fn clone(&self) -> Complex {
        Complex {
            img: self.img,
            real: self.real,
        }
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex {
            img: self.img + other.img,
            real: self.real + other.real,
        }
    }

    fn square(&self) -> Complex {
        Complex {
            img: 2.0 * self.img * self.real,
            real: self.real * self.real - self.img * self.img
        }
    }

    fn abs(&self) -> f64 {
        (self.img * self.img + self.real * self.real).sqrt()
    }
 }

fn check_for_member_of_set(c_0: &Complex, max_iter: i32) -> i32{
    let mut c = c_0.clone();

    for i in 0..max_iter {

        if c.abs() > 2.0 {
            return i;
        }

        c = c.square().add(c_0);
    }

    max_iter
}

fn get_color_for_iterations(count: i32) -> [i32; 3] {
    [min(count, 255), min(count * 4, 255), min(count * 10, 255)]
}

fn main() {
    const WIDTH: i32 = 100;
    const HEIGHT: i32 = 100;
    const MAX_ITER: i32 = 300;
    const DIMENSION: [f64; 4] = [-2.0, 2.0, -2.0, 2.0];

    let step_img = (DIMENSION[1] - DIMENSION[0]) / HEIGHT as f64;
    let step_real = (DIMENSION[3] - DIMENSION[2]) / WIDTH as f64;

    let output_file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(output_file);
    write!(&mut writer, "P3\n{} {}\n255\n", WIDTH, HEIGHT).unwrap();

    for y in 0..WIDTH {
        for x in 0..HEIGHT {
            let c = Complex::new(
                DIMENSION[0] + step_img * y as f64,
                DIMENSION[2] + step_real * x as f64
            );

            let iterations = check_for_member_of_set(&c, MAX_ITER);
            let color = get_color_for_iterations(iterations);

            write!(&mut writer, "{} {} {} ", color[0], color[1], color[2]).unwrap();
        }
    }
}
