use std::{fs, io::Write};

use vec::Vec3;

mod vec;
mod ray;
fn create_image(image_width: u64, image_height: u64){

    let mut image = fs::File::create("image.ppm").unwrap();
    write!(&mut image, "P3\n{} {}\n255\n", image_height, image_width)
        .expect("cannot write to file");

    for i in 0..image_height {
        for j in 0..image_width {
            let rgb = Vec3{x: j, y: i, z: 128.0};
            write!(&mut image, "{:?}",rgb).expect("cannot write to file");
        }
    }
}
fn main() {
    //create_image(512, 512);   
}
