use geo::{coord, point, polygon, Contains, CoordsIter, Point, Rect, Rotate};
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

fn corner_point_size(centre: geo::Coord, jutpoint1: geo::Coord, jutpoint2: geo::Coord) -> f64 {
    (centre.x - jutpoint1.x).abs().max(
        (centre.y - jutpoint1.y).abs()
    ).max(
        (centre.x - jutpoint2.x).abs()
    ).max(
        (centre.y - jutpoint2.y).abs()
    ) * 2.0
}

pub fn paper_img<T: Rng>(mut rng: T) -> (ImageBuffer<Luma<u8>, Vec<u8>>, [(geo::Coord, f64); 4]) {
    let (begin, over) = border(&mut rng);
    let high = generate_high(&mut rng, begin, over);
    // println!("{:#?}\t{:#?}\t{}", begin, over, high);
    let (jut_length, jut_high) = jut(&mut rng, high);
    let rotate = rng.gen_range(0.0..360.0);
    let gray_scale: u8 = rng.gen_range(0..255);
    let mut poly = polygon![
        (x: begin.x() + high, y: begin.y() + high), // 角点1
        (x: begin.x() + high - jut_high, y: begin.y() + high - jut_high),
        (x: begin.x() + high - jut_high, y: begin.y() + high - jut_length + jut_high),
        (x: begin.x() + high, y: begin.y() + high - jut_length),
        (x: begin.x() + high, y: begin.y()),
        (x: over.x() - high, y: begin.y()),
        (x: over.x() - high, y: begin.y() + high - jut_length),
        (x: over.x() - high + jut_high, y: begin.y() + high - jut_length + jut_high),
        (x: over.x() - high + jut_high, y: begin.y() + high - jut_high),
        (x: over.x() - high, y: begin.y() + high),  // 角点2
        (x: over.x(), y: begin.y() + high),
        (x: over.x(), y: over.y() - high),
        (x: over.x() - high, y: over.y() - high),   // 角点3
        (x: over.x() - high + jut_high, y: over.y() - high + jut_high),
        (x: over.x() - high + jut_high, y: over.y() - high + jut_length - jut_high),
        (x: over.x() - high, y: over.y() - high + jut_length),
        (x: over.x() - high, y: over.y()),
        (x: begin.x() + high, y: over.y()),
        (x: begin.x() + high, y: over.y() - high + jut_length),
        (x: begin.x() + high - jut_high, y: over.y() - high + jut_length - jut_high),
        (x: begin.x() + high - jut_high, y: over.y() - high + jut_high),
        (x: begin.x() + high, y: over.y() - high),  // 角点4
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
        let bg = if rng.gen_bool(gray_scale as f64 / 255.0) {
            match gray_scale.checked_sub(8) {
                Some(n) => rng.gen_range(0..=n),
                None => rng.gen_range((gray_scale+8)..=255),
            }
        } else {
            match gray_scale.checked_add(8) {
                Some(n) => rng.gen_range(n..=255),
                None => rng.gen_range(0..=(gray_scale-8)),
            }
        };
        match poly.contains(&point!(x: x as f64 / IMGSIZE as f64, y: y as f64 / IMGSIZE as f64)) {
            true => *pixel = image::Luma([gray_scale]),
            false => *pixel = image::Luma([bg]),
        }
    });
    let poly: Vec<geo::Coord> = poly.coords_iter().collect();
    let polys = [
        (poly[0], corner_point_size(poly[0], poly[1], poly[2])),
        (poly[9], corner_point_size(poly[9], poly[8], poly[7])),
        (poly[12], corner_point_size(poly[12], poly[13], poly[14])),
        (poly[21], corner_point_size(poly[21], poly[20], poly[19]))
    ];
    (img, polys)
}

// pub fn test(mut img: ImageBuffer<Luma<u8>, Vec<u8>>, polys: [(geo::Coord, f64); 4]) -> ImageBuffer<Luma<u8>, Vec<u8>> {
//     for (o, s) in polys {
//         let poly = Rect::new(coord! {x: o.x - (s/2.0), y: o.y - (s/2.0)}, coord! {x: o.x + (s/2.0), y: o.y + (s/2.0)});
//         img.enumerate_pixels_mut().for_each(|(x, y, pixel): (u32, u32, &mut Luma<u8>)| {
//             match poly.contains(&point!(x: x as f64 / IMGSIZE as f64, y: y as f64 / IMGSIZE as f64)) {
//                 true => *pixel = image::Luma([128]),
//                 false => (),
//             }
//         })
//     }
//     img
// }