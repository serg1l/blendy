use std::path::PathBuf;
use image;
pub fn blur_image(infile: &PathBuf, outfile: &PathBuf, sigma: f32) {
    let img = image::open(infile).expect("error while reading image");
    let img = img.blur(sigma);
    img.save(outfile).expect("error while writing image");
}
