use std::path::PathBuf;
use clap::{value_parser, Arg, ArgGroup, ArgMatches, Command};

pub mod modificators {
    mod blur;
    mod crop;
    mod grayscale;
    mod invert;
    mod rotate;
    mod brighten;

    pub use {
        blur::blur_image, crop::crop_image, grayscale::gray_image,
        invert::invert_image, rotate::rotate_image, brighten::bright_image
    };
}
//definiton of the cli
pub fn create_cli() -> ArgMatches {
    let commands = Command::new("blendy")
        .about("blendy: a simple cli for image modification")
        .groups([
            ArgGroup::new("files")
                .args(["infile","outfile"])
                .multiple(true),

            ArgGroup::new("modificators")
                .args(["blur", "crop", "grayscale", "brightness", "invert", "rotate"])
                .multiple(true)
                .requires("files")
        ])
        .args([
            Arg::new("blur")
                .long("blur")
                .short('b')
                .value_name("intensity")
                .num_args(1)
                .value_parser(value_parser!(f32)),

            Arg::new("crop")
                .long("crop")
                .short('c')
                .value_names(["x", "y", "width", "height"])
                .num_args(4)
                .value_parser(value_parser!(u32)),

            Arg::new("grayscale")
                .long("grayscale")
                .short('g')
                .num_args(0),

            Arg::new("brightness")
                .long("bright")
                .alias("bt")
                .short('t')
                .num_args(1)
                .value_name("brightness")
                .value_parser(value_parser!(i32))
                .allow_hyphen_values(true),

            Arg::new("invert")
                .long("invert")
                .short('i')
                .num_args(0),

            Arg::new("rotate")
                .long("rotate")
                .short('r')
                .value_name("direction")
                .num_args(1)
                .value_parser(["right", "left", "reverse"])
        ])
        .args([
            Arg::new("infile").value_name("INFILE").required(true).num_args(1)
                .value_parser(value_parser!(PathBuf)).help("file to be modified").group("files").index(1),
            Arg::new("outfile").value_name("OUTFILE").required(true).num_args(1)
                .value_parser(value_parser!(PathBuf)).help("path to store a new modified file").group("files").index(2),
        ])
        .get_matches();
    commands
}
