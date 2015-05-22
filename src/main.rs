use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

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
}

fn main() {
    const WIDTH: i64 = 100;
    const HEIGHT: i64 = 100;

    let output_file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(output_file);
    write!(&mut writer, "P3\n{} {}\n255\n", WIDTH, HEIGHT).unwrap();

    for _y in 0..WIDTH {
        for _x in 0..HEIGHT {
            let sum = vec![0, 150, 0];
            write!(&mut writer, "{} {} {} ", sum[0], sum[1], sum[2]).unwrap();
        }
    }
}
