use geo::{coord, point, polygon, Contains, CoordsIter, Point, Polygon, Rect, Rotate};
use image::{ImageBuffer, Luma};
use rand::Rng;
use crate::IMGSIZE;

fn border<T: Rng>(mut rng: T) -> (Point, Point) {
    let mut begin = point!(x: rng.gen_range(0.0 + 5.0 / IMGSIZE as f64..=0.5), y: rng.gen_range(0.0 + 5.0 / IMGSIZE as f64..=0.5));
    let mut over = point!(x: rng.gen_range(0.5..1.0 - 5.0 / IMGSIZE as f64), y: rng.gen_range(0.5..1.0 - 5.0 / IMGSIZE as f64));
    while (over.x() - begin.x()).min(over.y() - begin.y())/2.0 < 5.0 / IMGSIZE as f64 {
        begin = point!(x: rng.gen_range(0.0..=0.5), y: rng.gen_range(0.0..=0.5));
        over = point!(x: rng.gen_range(0.5..1.0), y: rng.gen_range(0.5..1.0));
    }
    (begin, over)
}

fn generate_high<T: Rng>(mut rng: T, begin: Point, over: Point) -> f64 {
    let mut high = rng.gen_range(0.0..(over.x() - begin.x()).min(over.y() - begin.y())/2.0);
    while high < 3.0 / IMGSIZE as f64 {
        high = rng.gen_range(0.0..(over.x() - begin.x()).min(over.y() - begin.y())/2.0);
    }
    high
}

fn jut<T: Rng>(mut rng: T, high: f64) -> (f64, f64) {
    let mut jut_length = rng.gen_range(0.0..high);
    let mut jut_high = rng.gen_range(0.0..jut_length/2.0);
    while jut_high < 1.5 / IMGSIZE as f64 {
        jut_length = rng.gen_range(0.0..high);
        jut_high = rng.gen_range(0.0..jut_length/2.0);
    }
    (jut_length, jut_high)
}

pub fn paper_img<T: Rng>(mut rng: T) -> (ImageBuffer<Luma<u8>, Vec<u8>>, Polygon) {
    let (begin, over) = border(&mut rng);
    let high = generate_high(&mut rng, begin, over);
    // println!("{:#?}\t{:#?}\t{}", begin, over, high);
    let (jut_length, jut_high) = jut(&mut rng, high);
    let rotate = rng.gen_range(0.0..360.0);
    let mut poly = polygon![
        (x: begin.x() + high, y: begin.y() + high),
        (x: begin.x() + high - jut_high, y: begin.y() + high - jut_high),
        (x: begin.x() + high - jut_high, y: begin.y() + high - jut_length + jut_high),
        (x: begin.x() + high, y: begin.y() + high - jut_length),
        (x: begin.x() + high, y: begin.y()),
        (x: over.x() - high, y: begin.y()),
        (x: over.x() - high, y: begin.y() + high - jut_length),
        (x: over.x() - high + jut_high, y: begin.y() + high - jut_length + jut_high),
        (x: over.x() - high + jut_high, y: begin.y() + high - jut_high),
        (x: over.x() - high, y: begin.y() + high),
        (x: over.x(), y: begin.y() + high),
        (x: over.x(), y: over.y() - high),
        (x: over.x() - high, y: over.y() - high),
        (x: over.x() - high + jut_high, y: over.y() - high + jut_high),
        (x: over.x() - high + jut_high, y: over.y() - high + jut_length - jut_high),
        (x: over.x() - high, y: over.y() - high + jut_length),
        (x: over.x() - high, y: over.y()),
        (x: begin.x() + high, y: over.y()),
        (x: begin.x() + high, y: over.y() - high + jut_length),
        (x: begin.x() + high - jut_high, y: over.y() - high + jut_length - jut_high),
        (x: begin.x() + high - jut_high, y: over.y() - high + jut_high),
        (x: begin.x() + high, y: over.y() - high),
        (x: begin.x(), y: over.y() - high),
        (x: begin.x(), y: begin.y() + high),
    ].rotate_around_centroid(rotate);
    while !poly.coords_iter().fold(true, |inside, p| inside && Rect::new(
        coord! { x: 0.0, y: 0.0},
        coord! { x: 1.0, y: 1.0},
    ).contains(&p)) {
        poly = poly.rotate_around_centroid(-(90.0/IMGSIZE as f64));
    }
    let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(IMGSIZE, IMGSIZE);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel): (u32, u32, &mut Luma<u8>)| {
        match poly.contains(&point!(x: x as f64 / IMGSIZE as f64, y: y as f64 / IMGSIZE as f64)) {
            true => *pixel = image::Luma([255]),
            false => *pixel = image::Luma([0]),
        }
    });
    (img, poly)
}