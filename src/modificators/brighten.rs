use std::path::PathBuf;
use image;
pub fn bright_image(infile: &PathBuf, outfile: &PathBuf, brightness: i32) {
    let img = image::open(infile).expect("failed opening image");
    let img = img.brighten(brightness);
    img.save(outfile).expect("failed writing image");
}
