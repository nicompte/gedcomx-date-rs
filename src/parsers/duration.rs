
use nom::digit;
use super::super::Duration;

use std::str;
use std::str::FromStr;

named!(u32_digit<u32>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

// parse duration
named!(pub duration <Duration>, chain!(
    complete!(tag!("P")) ~
    y: terminated!(u32_digit, tag!("Y"))? ~
    m: terminated!(u32_digit, tag!("M"))? ~
    d: terminated!(u32_digit, tag!("D"))? ~
    dt: duration_time?,
    || {
        let duration_time = match dt {
            Some(d) => d,
            None => (0, 0, 0)
        };
        Duration{
            years: y.unwrap_or(0),
            months: m.unwrap_or(0),
            days: d.unwrap_or(0),
            hours: duration_time.0,
            minutes: duration_time.1,
            seconds: duration_time.2
        }
    }
));

// parse duration time
named!(duration_time <(u32, u32, u32)>, dbg!(chain!(
    complete!(tag!("T")) ~
    h: terminated!(u32_digit, tag!("H"))? ~
    m: terminated!(u32_digit, tag!("M"))? ~
    s: terminated!(u32_digit, tag!("S"))?,
    || {
        (h.unwrap_or(0), m.unwrap_or(0), s.unwrap_or(0))
    }
)));

#[cfg(test)]
mod tests {

    use nom::IResult::*;

    use super::super::super::Duration;
    use super::{duration, duration_time};

    #[test]
    fn test_duration_time() {
        assert_eq!(Done(&[][..], (10, 10, 10)), duration_time(b"T10H10M10S"));

        assert_eq!(Done(&[][..], (10, 10, 0)), duration_time(b"T10H10M0S"));

        assert_eq!(Done(&[][..], (10, 0, 0)), duration_time(b"T10H0S"));

        assert_eq!(Done(&[][..], (0, 0, 200)), duration_time(b"T200S"));

        assert!(duration_time(b"10H10M10S").is_err());
        assert!(duration_time(b"10H10S10M").is_err());
    }

    #[test]
    fn test_duration() {
        assert_eq!(
            Done(
                &[][..],
                Duration {
                    years: 10,
                    months: 20,
                    days: 30,
                    hours: 10,
                    minutes: 20,
                    seconds: 30,
                }
            ),
            duration(b"P10Y20M30DT10H20M30S")
        );

        assert_eq!(
            Done(
                &[][..],
                Duration {
                    years: 10,
                    months: 0,
                    days: 0,
                    hours: 10,
                    minutes: 0,
                    seconds: 0,
                }
            ),
            duration(b"P10YT10H0S")
        );

        assert_eq!(
            Done(
                &[][..],
                Duration {
                    years: 0,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 1000,
                    seconds: 0,
                }
            ),
            duration(b"PT1000M0S")
        );

        assert_eq!(
            Done(
                &[][..],
                Duration {
                    years: 10,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                }
            ),
            duration(b"P10YT0S")
        );

        assert_eq!(
            Done(
                &[][..],
                Duration {
                    years: 0,
                    months: 10,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                }
            ),
            duration(b"P10MT0S")
        );

        assert_eq!(
            Done(
                &[][..],
                Duration {
                    years: 0,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 10,
                    seconds: 0,
                }
            ),
            duration(b"PT10M0S")
        );

        assert!(duration(b"YT1000M").is_err());
    }
}
