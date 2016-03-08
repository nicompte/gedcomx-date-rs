
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
    y: preceded!(tag!("Y"), u32_digit)? ~
    m: preceded!(tag!("M"), u32_digit)? ~
    d: preceded!(tag!("D"), u32_digit)? ~
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
    h: preceded!(tag!("H"), u32_digit)? ~
    m: preceded!(tag!("M"), u32_digit)? ~
    s: preceded!(tag!("S"), u32_digit)?,
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
        assert_eq!(Done(&[][..], (10, 10, 10)), duration_time(b"TH10M10S10"));

        assert_eq!(Done(&[][..], (10, 10, 0)), duration_time(b"TH10M10S0"));

        assert_eq!(Done(&[][..], (10, 0, 0)), duration_time(b"TH10S0"));


        assert_eq!(Done(&[][..], (0, 0, 200)), duration_time(b"TS200"));

        assert!(duration_time(b"H10M10S10").is_err());
        assert!(duration_time(b"H10S10M10").is_err());
    }

    #[test]
    fn test_duration() {
        assert_eq!(Done(&[][..],
                        Duration {
                            years: 10,
                            months: 20,
                            days: 30,
                            hours: 10,
                            minutes: 20,
                            seconds: 30,
                        }),
                   duration(b"PY10M20D30TH10M20S30"));

        assert_eq!(Done(&[][..],
                        Duration {
                            years: 10,
                            months: 0,
                            days: 0,
                            hours: 10,
                            minutes: 0,
                            seconds: 0,
                        }),
                   duration(b"PY10TH10S0"));

        assert_eq!(Done(&[][..],
                        Duration {
                            years: 0,
                            months: 0,
                            days: 0,
                            hours: 0,
                            minutes: 1000,
                            seconds: 0,
                        }),
                   duration(b"PTM1000S0"));

        assert_eq!(Done(&[][..],
                        Duration {
                            years: 10,
                            months: 0,
                            days: 0,
                            hours: 0,
                            minutes: 0,
                            seconds: 0,
                        }),
                   duration(b"PY10TS0"));

        assert_eq!(Done(&[][..],
                        Duration {
                            years: 0,
                            months: 10,
                            days: 0,
                            hours: 0,
                            minutes: 0,
                            seconds: 0,
                        }),
                   duration(b"PM10TS0"));

        assert_eq!(Done(&[][..],
                        Duration {
                            years: 0,
                            months: 0,
                            days: 0,
                            hours: 0,
                            minutes: 10,
                            seconds: 0,
                        }),
                   duration(b"PTM10S0"));

        assert!(duration(b"YTM1000").is_err());
    }

}
