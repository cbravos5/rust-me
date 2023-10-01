use chrono::{FixedOffset,DateTime};

fn days_between(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> i32 {
  let not_rounded_days = ((end.timestamp() - start.timestamp())/86400) as f64;

  not_rounded_days.floor() as i32
}

pub fn execute() {
  println!("{}", days_between(
    DateTime::parse_from_str("2019 Jun 14 00:00:00.000 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap(),
    DateTime::parse_from_str("2019 Jun 20 00:00:00.000 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap()
  ));
  println!("{}", days_between(
    DateTime::parse_from_str("2018 Dec 29 00:00:00.000 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap(),
    DateTime::parse_from_str("2019 Jan 01 00:00:00.000 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap()
  ));
  println!("{}", days_between(
    DateTime::parse_from_str("2024 Feb 28 00:00:00.000 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap(),
    DateTime::parse_from_str("2024 Mar 01 00:00:00.000 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap()
  ));
}