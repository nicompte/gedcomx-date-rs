use super::super::Duration;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::sequence::{terminated, Tuple};
use nom::IResult;
use std::str;

fn u32_digit(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

pub fn duration(i: &str) -> IResult<&str, Duration> {
    let p = tag("P");
    let year = opt(terminated(u32_digit, tag("Y")));
    let month = opt(terminated(u32_digit, tag("M")));
    let day = opt(terminated(u32_digit, tag("D")));

    let (remaining, (_, y, m, d, dt)) = (p, year, month, day, opt(duration_time)).parse(i)?;
    let duration_time = dt.unwrap_or((0, 0, 0));
    Ok((
        remaining,
        Duration {
            years: y.unwrap_or(0),
            months: m.unwrap_or(0),
            days: d.unwrap_or(0),
            hours: duration_time.0,
            minutes: duration_time.1,
            seconds: duration_time.2,
        },
    ))
}

fn duration_time(i: &str) -> IResult<&str, (u32, u32, u32)> {
    let t = tag("T");
    let hour = opt(terminated(u32_digit, tag("H")));
    let minute = opt(terminated(u32_digit, tag("M")));
    let second = opt(terminated(u32_digit, tag("S")));

    let (remaining, (_, h, m, s)) = (t, hour, minute, second).parse(i)?;
    Ok((remaining, (h.unwrap_or(0), m.unwrap_or(0), s.unwrap_or(0))))
}

#[cfg(test)]
mod tests {
    use super::super::super::Duration;
    use super::{duration, duration_time};

    #[test]
    fn test_duration_time() {
        assert_eq!(Ok(("", (10, 10, 10))), duration_time("T10H10M10S"));

        assert_eq!(Ok(("", (10, 10, 0))), duration_time("T10H10M0S"));

        assert_eq!(Ok(("", (10, 0, 0))), duration_time("T10H0S"));

        assert_eq!(Ok(("", (0, 0, 200))), duration_time("T200S"));

        assert!(duration_time("10H10M10S").is_err());
        assert!(duration_time("10H10S10M").is_err());
    }

    #[test]
    fn test_duration() {
        assert_eq!(
            Ok((
                "",
                Duration {
                    years: 10,
                    months: 20,
                    days: 30,
                    hours: 10,
                    minutes: 20,
                    seconds: 30,
                }
            )),
            duration("P10Y20M30DT10H20M30S")
        );

        assert_eq!(
            Ok((
                "",
                Duration {
                    years: 10,
                    months: 0,
                    days: 0,
                    hours: 10,
                    minutes: 0,
                    seconds: 0,
                }
            )),
            duration("P10YT10H0S")
        );

        assert_eq!(
            Ok((
                "",
                Duration {
                    years: 0,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 1000,
                    seconds: 0,
                }
            )),
            duration("PT1000M0S")
        );

        assert_eq!(
            Ok((
                "",
                Duration {
                    years: 10,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                }
            )),
            duration("P10YT0S")
        );

        assert_eq!(
            Ok((
                "",
                Duration {
                    years: 0,
                    months: 10,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                }
            )),
            duration("P10MT0S")
        );

        assert_eq!(
            Ok((
                "",
                Duration {
                    years: 0,
                    months: 0,
                    days: 0,
                    hours: 0,
                    minutes: 10,
                    seconds: 0,
                }
            )),
            duration("PT10M0S")
        );

        assert!(duration("YT1000M").is_err());
    }
}
