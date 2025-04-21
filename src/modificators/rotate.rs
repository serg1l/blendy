use std::path::PathBuf;
use image;
pub fn rotate_image(infile: &PathBuf, outfile: &PathBuf, rotation: String) {
    let img = image::open(infile).expect("failed to open image");
    let img = match rotation.as_str() {
        "left" => img.rotate270(),
        "right" => img.rotate270(),
        "reverse" => img.rotate180(),
        _ => img
    };
    img.save(outfile).expect("failed to write image");
}
