use std::{error::Error, fs, io::Write};

mod color;
mod ray;
mod vec;

use color::Color;
use ray::Ray;
use vec::Vec3;

fn ray_color(ray: &Ray) -> Option<Vec3> {
    if let Some(unit_direction) = Vec3::unit(ray.direction) {
        let a = (unit_direction.y + 1.0) * 0.5;
        Some(
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } * (1.0 - a)
                + Vec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                } * a,
        )
    } else {
        None
    }
}

fn create_image() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let mut image_height = f64::floor(image_width / aspect_ratio) as u64;

    if image_height < 1 {
        image_height = 1;
    } else {
        image_height = image_height;
    }

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width / (image_height as f64);
    let camera_center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_u = Vec3 {
        x: (viewport_width as f32),
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = Vec3 {
        x: 0.0,
        y: (-viewport_height as f32),
        z: 0.0,
    };

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut image = fs::File::create("image.ppm").unwrap();
    write!(
        &mut image,
        "P3\n{} {}\n255\n",
        image_width as u64, image_height
    )
    .expect("cannot write to file");

    for i in 0..image_height {
        for j in 0..(image_width as u64) {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * j as f32) + (pixel_delta_v * i as f32);
            let ray_direction = pixel_center - camera_center;
            let r = Ray {
                origin: camera_center,
                direction: ray_direction,
            };
            if let Some(pixel_color) = ray_color(&r) {
                let ir = (pixel_color.x.clamp(0.0, 1.0) * 255.99) as u64;
                let ig = (pixel_color.y.clamp(0.0, 1.0) * 255.99) as u64;
                let ib = (pixel_color.z.clamp(0.0, 1.0) * 255.99) as u64;
                let final_color = Color {
                    r: ir,
                    g: ig,
                    b: ib,
                };
                write!(&mut image, "{:?}", final_color).expect("cannot write :(");
            }
        }
    }

    Ok(())
}

fn main() {
    create_image().unwrap_or_else(|err| {
        println!("couldn't create image, error: {}", err);
    });
}
