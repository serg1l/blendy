use std::path::PathBuf;
use clap::Id;
use blendy::{create_cli, modificators::*};

fn main() {
    let mut commands = create_cli();
    //extract the input and output files from their respective ids
    let mut infile: PathBuf = commands.remove_one("infile").unwrap();
    let outfile: PathBuf = commands.remove_one("outfile").unwrap();
    //obtain all the flags used in OPTIONS (thanks to the group id where they are)
    if let Some(prueba) = commands.get_many::<Id>("modificators"){
        let mut execute_count = 1;
        prueba.for_each(|id| {
            if execute_count > 1 { infile = outfile.clone() };
            let flag_name = id.as_str();

            match id.as_str() {
                "blur" => {
                    let sigma: &f32 = commands.get_one(flag_name).unwrap();
                    blur_image(&infile, &outfile, *sigma);
                    execute_count+=1;
                }

                "crop" => {
                    let mut params:Vec<u32> = Vec::with_capacity(4);
                    commands.get_many(flag_name).unwrap().for_each(|x| params.push(*x));
                    crop_image(&infile, &outfile, params);
                    execute_count+=1;
                }

                "grayscale" => {
                    gray_image(&infile, &outfile);
                    execute_count+=1;
                }

                "brightness" => {
                    let brightness: &i32 = commands.get_one(flag_name).unwrap();
                    bright_image(&infile, &outfile, *brightness);
                    execute_count+=1;
                }

                "rotate" => {
                    let rotation: &String = commands.get_one(flag_name).unwrap();
                    rotate_image(&infile, &outfile, rotation.clone());
                    execute_count+=1;
                }

                "invert" => {
                    invert_image(&infile, &outfile);
                    execute_count+=1;
                }

                _ => { println!("hola"); execute_count+=1;}
            }
        });
    }
}
