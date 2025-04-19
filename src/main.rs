use std::{fs, io::Write};

mod color;
mod ray;
mod vec;

use color::Color;
use ray::Ray;
use vec::Vec3;

fn create_image(image_width: u64, image_height: u64) {
    let mut image = fs::File::create("image.ppm").unwrap();
    write!(&mut image, "P3\n{} {}\n255\n", image_height, image_width)
        .expect("cannot write to file");

    for i in 0..image_height {
        for j in 0..image_width {
            let rgb = Color { r: j, g: i, b: 0 };
            write!(&mut image, "{:?}", rgb).expect("cannot write to file");
        }
    }
}

fn main() {
    create_image(256, 256);
    let v = Vec3::new();
}
