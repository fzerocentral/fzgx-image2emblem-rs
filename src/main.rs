extern crate image2emblem;
extern crate image;
extern crate chrono;
extern crate byteorder;
extern crate clap;


use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::process::exit;
use clap::{Arg, App, SubCommand};


fn main() {
    let matches = App::new("image2emblem")
        .version("1.0.0")
        .author("Ricardo Mendes <rokusu@gmail.com>")
        .about("Converts images to F-Zero GX emblems")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("crop-edges")
            .short("c")
            .long("--crop-edges")
            .help("Crops the edges"))
        .arg(Arg::with_name("emblem-filename")
            .help("Specify a custom emblem filename to put in place of the default timestamp."))
        .get_matches();

    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    let path = Path::new(matches.value_of("INPUT").unwrap());
    let mut img = match image::open(&path) {
        Ok(image) => image,
        Err(why) => panic!("couldn't open '{:?}': {}", path, why)
    };
    let now = chrono::UTC::now();
    let alpha_threshold: i8 = 1;
    let seconds_since_2000 = image2emblem::seconds_since_2000(now);
    let short_name = image2emblem::short_name(seconds_since_2000);
    let full_name = image2emblem::full_name(&short_name);

    let emblem = image2emblem::make_emblem(&mut img, &matches, short_name, seconds_since_2000, now, alpha_threshold);

    let mut emblem_file = match File::create(full_name) {
        Ok(name) => name,
        Err(why) => panic!("couldn't create file: {}", why)
    };
    let result = emblem_file.write_all(&emblem.as_bytes());

    match result {
        Ok(_) => exit(0),
        Err(err) => panic!("Was not possible to write to file: {}", err)
    }
}
