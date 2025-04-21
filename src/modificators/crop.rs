use std::path::PathBuf;
use image;

pub fn crop_image(infile: &PathBuf, outfile: &PathBuf, params: Vec<u32>) {
    let mut img = image::open(infile).expect("failed to open image");
    let img = img.crop(params[0], params[1], params[2], params[3]);
    img.save(outfile).expect("failed to save image");
}
