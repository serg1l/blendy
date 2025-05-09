use image;
use std::path::PathBuf;
pub fn gray_image(infile: &PathBuf, outfile: &PathBuf) {
    let img = image::open(infile).expect("failed to open image");
    let img = img.grayscale();
    img.save(outfile).expect("failed to write image");
}
