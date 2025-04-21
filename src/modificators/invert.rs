use std::path::PathBuf;
use image;
pub fn invert_image(infile: &PathBuf, outfile: &PathBuf){
    let mut img = image::open(infile).expect("failed to open image");
    img.invert();
    img.save(outfile).expect("failed to write image");
}
