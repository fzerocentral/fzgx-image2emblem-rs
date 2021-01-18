use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;

/// Converts images to F-Zero GX emblems
#[derive(StructOpt, Debug)]
#[structopt(name = "image2emblem")]
struct Program {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Crops the edges of the emblem.
    ///
    /// If the edges are not cropped, they will be stretched to cover
    /// all of the custom vehicle when applied.
    #[structopt(short, long)]
    crop_edges: bool,

    /// Specify a custom emblem filename to put in place of the default timestamp.
    #[structopt(long)]
    emblem_filename: String,

    /// Specify where the output should be placed.
    #[structopt(short, long)]
    output_path: String,

    /// Specifies which region the save game should be generated for. Accepts NTSC and PAL.
    #[structopt(short, long, default_value="NTSC")]
    region: image2emblem::gamecube::memcard::Region
}

fn main() {
    // let matches = App::new("image2emblem")
    //     .version("1.0.0")
    //     .author("Ricardo Mendes <rokusu@gmail.com>")
    //     .about("Converts images to F-Zero GX emblems")
    //     .arg(
    //         Arg::with_name("INPUT")
    //             .help("Sets the input file to use")
    //             .required(true)
    //             .index(1),
    //     )
    //     .arg(
    //         Arg::with_name("crop-edges")
    //             .short("c")
    //             .long("--crop-edges")
    //             .help("Crops the edges"),
    //     )
    //     .arg(
    //         Arg::with_name("region")
    //             .short("r")
    //             .long("--region")
    //             .help("Specifies which region the save game should be generated for. Accepts NTSC and PAL.")
    //             .possible_values(&["NTSC", "PAL"])
    //             .default_value("NTSC"),
    //     )
    //     .arg(
    //         Arg::with_name("output-path")
    //             .short("o")
    //             .long("--output-path")
    //             .value_name("PATH")
    //             .help("Specify where the output should be placed"),
    //     )
    //     .arg(
    //         Arg::with_name("emblem-filename")
    //             .help("Specify a custom emblem filename to put in place of the default timestamp."),
    //     )
    //     .get_matches();

    // let region = match matches.value_of("region").unwrap() {
    //     "NTSC" => image2emblem::gamecube::memcard::Region::NTSC,
    //     "PAL" => image2emblem::gamecube::memcard::Region::PAL,
    //     unknown_region => panic!("'{}' is not valid.  Region must be NTSC for PAL.", unknown_region),
    // };

    // let path = Path::new(matches.value_of("INPUT").unwrap());
    // let mut img = match image::open(&path) {
    //     Ok(image) => image,
    //     Err(why) => panic!("couldn't open '{:?}': {}", path, why),
    // };
    // let now = chrono::UTC::now();
    // let alpha_threshold: i8 = 1;
    // let seconds_since_2000 = image2emblem::seconds_since_2000(now);
    // let short_name = image2emblem::short_name(seconds_since_2000);
    // let full_name = image2emblem::full_name(&short_name);
    // let output_path = Path::new(matches.value_of("output-path").unwrap_or("")).join(full_name);

    // let emblem = image2emblem::make_emblem(
    //     &mut img,
    //     &matches,
    //     short_name,
    //     seconds_since_2000,
    //     now,
    //     alpha_threshold,
    //     region
    // );

    // let mut emblem_file = match File::create(output_path) {
    //     Ok(name) => name,
    //     Err(why) => panic!("couldn't create file: {}", why),
    // };
    // let result = emblem_file.write_all(&emblem.as_bytes());

    // match result {
    //     Ok(_) => exit(0),
    //     Err(err) => panic!("Was not possible to write to file: {}", err),
    // }
    let opt = Program::from_args();
    println!("{:#?}", opt);
}
