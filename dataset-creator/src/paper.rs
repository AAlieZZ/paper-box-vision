use geo::{coord, point, polygon, Contains, EuclideanDistance, Point, Rect, Rotate};
use image::{ImageBuffer, Luma};
use rand::{distributions::uniform::SampleRange, Rng};
use crate::IMGSIZE;

fn point_in_circle<R, N>(mut rng: N, xy: R) -> Point
where
    R: SampleRange<f64> + Clone,
    N: Rng
{
    let p = point!(x: rng.gen_range(xy.clone()), y: rng.gen_range(xy.clone()));
    match p.euclidean_distance(&point!(x: 0.5, y: 0.5)) <= 0.5 {
        true => p,
        false => point_in_circle(rng, xy),
    }
}

pub fn paper_img<T: Rng>(mut rng: T) -> (ImageBuffer<Luma<u8>, Vec<u8>>, Rect) {
    let begin = point_in_circle(&mut rng, 0.0..=0.5);
    let over = point_in_circle(&mut rng, 0.5..1.0);
    let high = rng.gen_range(0.0..(over.x() - begin.x()).min(over.y() - begin.y())/2.0);
    // println!("{:#?}\t{:#?}\t{}", begin, over, high);
    let jut_length = rng.gen_range(0.0..high);
    let jut_high = rng.gen_range(0.0..jut_length/2.0);
    let rotate = rng.gen_range(0.0..360.0);
    let poly = polygon![
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
    // let poly = polygon![
    //     (x: begin.x(), y: begin.y()),
    //     (x: over.x(), y: begin.y()),
    //     (x: over.x(), y: over.y()),
    //     (x: begin.x(), y: over.y()),
    // ];
    let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(IMGSIZE, IMGSIZE);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel): (u32, u32, &mut Luma<u8>)| {
        match poly.contains(&point!(x: x as f64 / IMGSIZE as f64, y: y as f64 / IMGSIZE as f64)) {
            true => *pixel = image::Luma([255]),
            false => *pixel = image::Luma([0]),
        }
    });
    (img, Rect::new(
        coord! { x: begin.x() + high, y: begin.y() + high},
        coord! { x: over.x() - high, y: over.y() - high},
    ).rotate_around_centroid(rotate))
}