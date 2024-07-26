mod args;
mod paper;

use std::path::Path;

use clap::Parser;
use paper::paper_img;
use args::Args;

const IMGSIZE: u32 = 640;

fn main () {
    let args = Args::parse();
    let mut rng = rand::thread_rng();
    (0..args.train).for_each(|i| {
        let (img, rect) = paper_img(&mut rng);
        img.save(Path::new(&args.path).join("images").join("train").join(i.to_string() + ".jpg")).expect(format!("Cannot save: {:#?}", Path::new(&args.path).join("images").join("train").join(i.to_string() + ".jpg")).as_str());
        std::fs::write(Path::new(&args.path).join("labels").join("train").join(i.to_string() + ".txt"), rect.into_iter().map(|(o, s)| format!("0 {} {} {} {}\n", o.x, o.y, s, s)).collect::<String>()).expect(format!("Cannot write: {:#?}", Path::new(&args.path).join("labels").join("train").join(i.to_string() + ".txt")).as_str());
        println!("train:\t{}", i);
    });
    (0..args.val).for_each(|i| {
        let (img, rect) = paper_img(&mut rng);
        img.save(Path::new(&args.path).join("images").join("val").join(i.to_string() + ".jpg")).expect(format!("Cannot save: {:#?}", Path::new(&args.path).join("images").join("val").join(i.to_string() + ".jpg")).as_str());
        std::fs::write(Path::new(&args.path).join("labels").join("val").join(i.to_string() + ".txt"), rect.into_iter().map(|(o, s)| format!("0 {} {} {} {}\n", o.x, o.y, s, s)).collect::<String>()).expect(format!("Cannot write: {:#?}", Path::new(&args.path).join("labels").join("val").join(i.to_string() + ".txt")).as_str());
        // paper::test(img, rect).save("test.jpg").unwrap();
        println!("val:\t{}", i);
    });
}