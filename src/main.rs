//! Triangle generator

extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;

/// the width of the png
const WIDTH: u32 = 800;

/// the height of the png
const HEIGHT: u32 = 600;

/// points used to build the triangle
pub struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let mut count: u32 = 1_000_000;

    let points: [Point; 3] = [
        Point { x: WIDTH / 2, y: 0 },
        Point { x: 0, y: HEIGHT },
        Point { x: WIDTH, y: HEIGHT },
    ];

    // init a variable for an array index
    let mut index: usize;

    /// Some random starting point
    let mut p = Point {
        x: rand::thread_rng().gen_range(0, WIDTH),
        y: rand::thread_rng().gen_range(0, HEIGHT),
    };

    let pixel = img[(0, 0)];

    while count > 0 {
        count -= 1;
        index = rand::thread_rng().gen_range(0, 3);
        p.x = (p.x + points[index].x) / 2;
        p.y = (p.y + points[index].y) / 2;
        img.put_pixel(p.x, p.y, pixel);
    }

    let ref mut fout = File::create(&Path::new("tri.png")).unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}
