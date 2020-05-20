pub mod checksum;
pub mod emblem;
pub mod gamecube {
    pub mod memcard;
}
pub mod image;

use crate::gamecube::memcard::Region;

extern crate chrono;
use chrono::*;

use ::image as img;

pub fn short_name(seconds: f64) -> String {
    let multiplier: f64 = 40500000f64;
    let tick: u64 = (seconds * multiplier) as u64;

    format!("fze0200002000{:14X}.dat", tick as u64)
}

pub fn full_name(filename: &str) -> String {
    format!("8P-GFZE-{}.gci", filename)
}

fn python_total_seconds(microseconds: i64) -> f64 {
    microseconds as f64 / 10i64.pow(6) as f64
}

pub fn seconds_since_2000(now: chrono::datetime::DateTime<UTC>) -> f64 {
    let year_2000 = chrono::UTC.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let duration = now - year_2000;

    match duration.num_microseconds() {
        Some(ms) => python_total_seconds(ms),
        None => panic!("No microseconds!"),
    }
}

pub fn make_emblem(
    img: &mut self::img::DynamicImage,
    matches: &clap::ArgMatches,
    short_name: String,
    seconds_since_2000: f64,
    now: chrono::datetime::DateTime<UTC>,
    alpha_threshold: i8,
    region: Region
) -> self::emblem::Emblem {
    let mut emblem = self::emblem::Emblem::default();
    let mut img64 = self::image::crop(img);
    let img32 = self::image::resize(&mut img64);

    if matches.is_present("crop-edges") {
        self::image::trim_edges(&mut img64);
    }

    emblem.set_gamecode(region);
    emblem.set_filename(short_name);
    emblem.set_timestamp(seconds_since_2000 as u32);
    let comment = format!(
        "{} (Created using Rust awesomeness)",
        now.format("%y/%m/%d %H:%M")
    );

    emblem.set_comment(comment);
    emblem.set_emblem_data(img64, alpha_threshold);
    emblem.set_banner_data(img32, alpha_threshold);
    emblem.set_icon_data();
    emblem.set_checksum();

    return emblem;
}
