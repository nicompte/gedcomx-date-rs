use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::one_of;
use nom::character::is_digit;
use nom::combinator::{map, opt};
use nom::sequence::Tuple;
use nom::IResult;

use super::super::{Date, DateTime, GedcomxDate, Simple, Time};
use super::approximate;

fn take_4_digits(i: &str) -> IResult<&str, &str> {
    take_while_m_n(4, 4, |c: char| c.is_ascii() && is_digit(c as u8))(i)
}

// YYYY
fn year_prefix(i: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-")))(i)
}

fn year(i: &str) -> IResult<&str, i32> {
    let (remaining, (pref, year)) = (year_prefix, take_4_digits).parse(i)?;

    let year = match pref {
        "-" => -year.parse::<i32>().unwrap(),
        _ => year.parse::<i32>().unwrap(),
    };

    Ok((remaining, year))
}

// MM

fn month_zero(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("0"), one_of("123456789")).parse(i)?;
    Ok((remaining, s.to_digit(10).unwrap()))
}

fn month_one(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("1"), one_of("012")).parse(i)?;
    Ok((remaining, 10 + s.to_digit(10).unwrap()))
}

fn month(i: &str) -> IResult<&str, u32> {
    alt((month_zero, month_one))(i)
}

// DD

fn day_zero(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("0"), one_of("123456789")).parse(i)?;
    Ok((remaining, s.to_digit(10).unwrap()))
}

fn day_one(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("1"), one_of("0123456789")).parse(i)?;
    Ok((remaining, 10 + s.to_digit(10).unwrap()))
}

fn day_two(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("2"), one_of("0123456789")).parse(i)?;
    Ok((remaining, 20 + s.to_digit(10).unwrap()))
}

fn day_three(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("3"), one_of("01")).parse(i)?;
    Ok((remaining, 30 + s.to_digit(10).unwrap()))
}

fn day(i: &str) -> IResult<&str, u32> {
    alt((day_zero, day_one, day_two, day_three))(i)
}

// YYYY-MM-DD
fn yyyymmdd(i: &str) -> IResult<&str, Date> {
    let (remaining, (y, _, m, _, d)) = (year, tag("-"), month, tag("-"), day).parse(i)?;
    Ok((
        remaining,
        Date {
            year: y,
            month: Some(m),
            day: Some(d),
        },
    ))
}

// YYYY-MM
fn yyyymm(i: &str) -> IResult<&str, Date> {
    let (remaining, (y, _, m)) = (year, tag("-"), month).parse(i)?;
    Ok((
        remaining,
        Date {
            year: y,
            month: Some(m),
            day: None,
        },
    ))
}

// YYYY
fn yyyy(i: &str) -> IResult<&str, Date> {
    year(i).map(|(remaining, year)| {
        (
            remaining,
            Date {
                year,
                month: None,
                day: None,
            },
        )
    })
}

// YYYY[-MM][-DD]
fn ymd(i: &str) -> IResult<&str, Date> {
    alt((yyyymmdd, yyyymm, yyyy))(i)
}

fn parse_date(i: &str) -> IResult<&str, Date> {
    ymd(i)
}

// TIME
// HH

fn lower_hour(i: &str) -> IResult<&str, u32> {
    let (remaining, (f, s)) = (one_of("01"), one_of("0123456789")).parse(i)?;
    Ok((
        remaining,
        f.to_digit(10).unwrap() * 10 + s.to_digit(10).unwrap(),
    ))
}

fn upper_hour(i: &str) -> IResult<&str, u32> {
    let (remaining, (_, s)) = (tag("2"), one_of("01234")).parse(i)?;
    Ok((remaining, 20 + s.to_digit(10).unwrap()))
}

fn hour(i: &str) -> IResult<&str, u32> {
    alt((lower_hour, upper_hour))(i)
}

// MM

fn below_sixty(i: &str) -> IResult<&str, u32> {
    let (remaining, (f, s)) = (one_of("012345"), one_of("0123456789")).parse(i)?;
    Ok((
        remaining,
        f.to_digit(10).unwrap() * 10 + s.to_digit(10).unwrap(),
    ))
}

fn upto_sixty(i: &str) -> IResult<&str, u32> {
    alt((below_sixty, map(tag("60"), |_| 60)))(i)
}

fn minute(i: &str) -> IResult<&str, u32> {
    below_sixty(i)
}

fn second(i: &str) -> IResult<&str, u32> {
    upto_sixty(i)
}

// hh:mm:ss
fn hhmmss(i: &str) -> IResult<&str, (u32, Option<u32>, Option<u32>)> {
    let (remaining, (h, _, m, _, s)) = (hour, tag(":"), minute, tag(":"), second).parse(i)?;
    Ok((remaining, (h, Some(m), Some(s))))
}

// hh:mm
fn hhmm(i: &str) -> IResult<&str, (u32, Option<u32>, Option<u32>)> {
    let (remaining, (h, _, m)) = (hour, tag(":"), minute).parse(i)?;
    Ok((remaining, (h, Some(m), None)))
}

// hh
fn hh(i: &str) -> IResult<&str, (u32, Option<u32>, Option<u32>)> {
    hour(i).map(|(remaining, hour)| (remaining, (hour, None, None)))
}

fn hms(i: &str) -> IResult<&str, (u32, Option<u32>, Option<u32>)> {
    alt((hhmmss, hhmm, hh))(i)
}

// HH[:MM][:SS][(Z|+...|-...)]
fn parse_time(i: &str) -> IResult<&str, Time> {
    let (remaining, (hms, z)) = (hms, opt(alt((timezone_hour, timezone_utc)))).parse(i)?;
    let tz = z.unwrap_or((None, None));
    Ok((
        remaining,
        Time {
            hours: hms.0,
            minutes: hms.1,
            seconds: hms.2,
            tz_offset_hours: tz.0,
            tz_offset_minutes: tz.1,
        },
    ))
}

fn sign(i: &str) -> IResult<&str, i32> {
    alt((tag("-"), tag("+")))(i).map(|(remaining, s)| {
        (
            remaining,
            match s {
                "+" => 1,
                _ => -1,
            },
        )
    })
}

fn timezone_minute(i: &str) -> IResult<&str, i32> {
    let (remaining, (_, m)) = (opt(tag(":")), opt(minute)).parse(i)?;
    Ok((remaining, m.unwrap_or(0) as i32))
}

fn timezone_hour(i: &str) -> IResult<&str, (Option<i32>, Option<i32>)> {
    let (remaining, (s, h, m)) = (sign, hour, timezone_minute).parse(i)?;
    Ok((remaining, (Some(s * (h as i32)), Some(s * m))))
}

fn timezone_utc(i: &str) -> IResult<&str, (Option<i32>, Option<i32>)> {
    map(tag("Z"), |_| (Some(0), Some(0)))(i)
}

pub fn simple_date(i: &str) -> IResult<&str, GedcomxDate> {
    let (remaining, (a, d)) = (opt(approximate), datetime).parse(i)?;
    Ok((
        remaining,
        GedcomxDate::Simple(Simple {
            date: d.date,
            time: d.time,
            approximate: a.is_some(),
        }),
    ))
}

fn time_piece(i: &str) -> IResult<&str, Time> {
    let (remaining, (_, t)) = (tag("T"), parse_time).parse(i)?;
    Ok((remaining, t))
}

pub fn datetime(i: &str) -> IResult<&str, DateTime> {
    let (remaining, (d, t)) = (parse_date, opt(time_piece)).parse(i)?;
    Ok((remaining, DateTime { date: d, time: t }))
}

#[cfg(test)]
mod tests {
    use nom::error::Error;

    use super::super::super::{Date, Time};
    use super::{day, month, year};
    use super::{hour, minute, second};
    use super::{parse_time, ymd};

    #[test]
    fn test_year() {
        assert_eq!(Ok(("", 2015)), year("+2015"));
        assert_eq!(Ok(("", -333)), year("-0333"));
        assert_eq!(Ok(("-", 2015)), year("+2015-"));

        assert!(year("2003").is_err());
        assert!(year("+abcd").is_err());
        assert!(year("+2a03").is_err());

        assert_eq!(
            year("+203"),
            Err(nom::Err::Error(Error {
                input: "203",
                code: nom::error::ErrorKind::TakeWhileMN
            }))
        );
    }

    #[test]
    fn test_month() {
        assert_eq!(Ok(("", 1)), month("01"));
        assert_eq!(Ok(("", 6)), month("06"));
        assert_eq!(Ok(("", 12)), month("12"));
        assert_eq!(Ok(("-", 12)), month("12-"));

        assert!(month("13").is_err());
        assert!(month("00").is_err());
        assert!(month("1").is_err());
    }

    #[test]
    fn test_day() {
        assert_eq!(Ok(("", 1)), day("01"));
        assert_eq!(Ok(("", 12)), day("12"));
        assert_eq!(Ok(("", 20)), day("20"));
        assert_eq!(Ok(("", 28)), day("28"));
        assert_eq!(Ok(("", 30)), day("30"));
        assert_eq!(Ok(("", 31)), day("31"));
        assert_eq!(Ok(("-", 31)), day("31-"));

        assert!(day("1").is_err());
        assert!(day("00").is_err());
        assert!(day("32").is_err());
    }

    #[test]
    fn test_ymd() {
        assert_eq!(
            Ok((
                "",
                Date {
                    year: 1988,
                    month: Some(3),
                    day: Some(29),
                }
            )),
            ymd("+1988-03-29")
        );
        assert_eq!(
            Ok((
                "",
                Date {
                    year: 1988,
                    month: Some(3),
                    day: None,
                }
            )),
            ymd("+1988-03")
        );
        assert_eq!(
            Ok((
                "",
                Date {
                    year: 1988,
                    month: None,
                    day: None,
                }
            )),
            ymd("+1988")
        );

        // assert!(ymd("+1988-3-29").is_err());
        // assert!(ymd("+1988-3").is_err());
        // assert!(ymd("+1988-").is_err());
    }

    #[test]
    fn test_hour() {
        assert_eq!(Ok(("", 0)), hour("00"));
        assert_eq!(Ok(("", 1)), hour("01"));
        assert_eq!(Ok(("", 6)), hour("06"));
        assert_eq!(Ok(("", 12)), hour("12"));
        assert_eq!(Ok(("", 13)), hour("13"));
        assert_eq!(Ok(("", 20)), hour("20"));
        assert_eq!(Ok(("", 24)), hour("24"));

        assert!(hour("25").is_err());
        assert!(hour("30").is_err());
        assert!(hour("a").is_err());
    }

    #[test]
    fn test_minute() {
        assert_eq!(Ok(("", 0)), minute("00"));
        assert_eq!(Ok(("", 1)), minute("01"));
        assert_eq!(Ok(("", 30)), minute("30"));
        assert_eq!(Ok(("", 59)), minute("59"));

        assert!(minute("60").is_err());
        assert!(minute("61").is_err());
        assert!(minute("a").is_err());
    }

    #[test]
    fn test_second() {
        assert_eq!(Ok(("", 0)), second("00"));
        assert_eq!(Ok(("", 1)), second("01"));
        assert_eq!(Ok(("", 30)), second("30"));
        assert_eq!(Ok(("", 59)), second("59"));
        assert_eq!(Ok(("", 60)), second("60"));

        assert!(second("61").is_err());
        assert!(second("a").is_err());
    }

    #[test]
    fn test_time() {
        assert_eq!(
            Ok((
                "",
                Time {
                    hours: 10,
                    minutes: None,
                    seconds: None,
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            )),
            parse_time("10")
        );
        assert_eq!(
            Ok((
                "",
                Time {
                    hours: 10,
                    minutes: Some(30),
                    seconds: None,
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            )),
            parse_time("10:30")
        );
        assert_eq!(
            Ok((
                "",
                Time {
                    hours: 10,
                    minutes: Some(30),
                    seconds: Some(29),
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            )),
            parse_time("10:30:29")
        );
        assert_eq!(
            Ok((
                ":1:01",
                Time {
                    hours: 10,
                    minutes: None,
                    seconds: None,
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            )),
            parse_time("10:1:01")
        );

        // assert!(parse_time("10:1:01").is_err());
    }
}
