pub mod checksum;
pub mod emblem;

extern crate chrono;


use chrono::*;


pub fn short_name(seconds: f64) -> String {
  let multiplier: f64 = 40500000f64;
  let tick: u64 = (seconds * multiplier) as u64;

  format!("fze0200002000{:14X}.dat", tick as u64)
}

pub fn full_name(filename: &str) -> String {
  format!("8P-GFZE-{}.gci", filename)
}

pub fn python_total_seconds(microseconds: i64) -> f64 {
    microseconds as f64 / 10i64.pow(6) as f64
}

pub fn seconds_since_2000(now: chrono::datetime::DateTime<UTC>) -> f64 {
    let year_2000 = chrono::UTC.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let duration = now - year_2000;

    match duration.num_microseconds() {
        Some(ms) => python_total_seconds(ms),
        None => panic!("No microseconds!")
    }
}

pub fn make_emblem() {

}
