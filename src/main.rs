use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

fn main() {
    let output_file = File::create("image.ppm").unwrap();
    let mut writer = BufWriter::new(output_file);
    write!(&mut writer, "P3\n{} {}\n255\n", 100, 100).unwrap();
    for _y in 0..100 {
        for _x in 0..100 {
            let sum = vec![0, 150, 0];
            write!(&mut writer, "{} {} {} ", sum[0], sum[1], sum[2]).unwrap();
        }
    }
}
