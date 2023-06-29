use nom::is_digit;

use super::super::{Date, DateTime, GedcomxDate, Simple, Time};
use super::approximate;
use helper::*;

named!(take_4_digits, flat_map!(take!(4), check!(is_digit)));

// YYYY
named!(year_prefix, alt!(tag!("+") | tag!("-")));
named!(
    year<i32>,
    chain!(
        pref: complete!(year_prefix) ~
        year: call!(take_4_digits)
        ,
        || {
            match pref {
                b"-" => - buf_to_i32(year),
                _ => buf_to_i32(year)
            }
    })
);

// MM
named!(
    month_zero<u32>,
    chain!(tag!("0") ~ s:char_between!('1', '9') , || buf_to_u32(s))
);
named!(
    month_one<u32>,
    chain!(tag!("1") ~ s:char_between!('0', '2') , || 10 + buf_to_u32(s))
);
named!(
    month<u32>,
    alt!(complete!(month_zero) | complete!(month_one))
);

// DD
named!(
    day_zero<u32>,
    chain!(tag!("0") ~ s:char_between!('1', '9'), || buf_to_u32(s))
);
named!(
    day_one<u32>,
    chain!(tag!("1") ~ s:char_between!('0', '9'), || 10 + buf_to_u32(s))
);
named!(
    day_two<u32>,
    chain!(tag!("2") ~ s:char_between!('0', '9'), || 20 + buf_to_u32(s))
);
named!(
    day_three<u32>,
    chain!(tag!("3") ~ s:char_between!('0', '1'), || 30 + buf_to_u32(s))
);
named!(
    day<u32>,
    alt!(complete!(day_zero) | complete!(day_one) | complete!(day_two) | complete!(day_three))
);

// YYYY[-MM][-DD]
named!(
    ymd<Date>,
    alt!(
        // YYYY-MM-DD
        chain!(y: year ~ complete!(tag!("-")) ~ m: month ~ complete!(tag!("-")) ~ d: day, || Date {year: y, month: Some(m), day: Some(d)}) |
// YYYY-MM
    chain!(y: year ~ complete!(tag!("-")) ~ m: month, || Date {year: y, month: Some(m), day: None}) |
// YYYY
    chain!(y: year, || Date {year: y, month: None, day: None})
    )
);

named!(pub parse_date <Date>, alt!( ymd ) );

// TIME
// HH
named!(
    lower_hour<u32>,
    chain!(f:char_between!('0','1') ~ s:char_between!('0','9') ,
                                       || { buf_to_u32(f) * 10 + buf_to_u32(s) } )
);
named!(
    upper_hour<u32>,
    chain!(tag!("2") ~ s:char_between!('0','4') , || 20 + buf_to_u32(s))
);
named!(hour<u32>, alt!(lower_hour | upper_hour));

// MM
named!(
    below_sixty<u32>,
    chain!(f:char_between!('0','5') ~ s:char_between!('0','9'), || { buf_to_u32(f) * 10 + buf_to_u32(s) } )
);
named!(
    upto_sixty<u32>,
    alt!(below_sixty | map!(tag!("60"), |_| 60))
);

named!(minute<u32>, call!(below_sixty));
named!(second<u32>, call!(upto_sixty));

named!(
    hms<(u32, Option<u32>, Option<u32>)>,
    alt!(
        // hh:mm:ss
        chain!(h: hour ~ complete!(tag!(":")) ~ m: minute ~ complete!(tag!(":")) ~ s: second, || (h, Some(m), Some(s))) |
// hh:mm
    chain!(h: hour ~ complete!(tag!(":")) ~ m: minute, || (h, Some(m), None)) |
// hh
    chain!(h: hour, || (h, None, None))
    )
);

// HH[:MM][:SS][(Z|+...|-...)]
named!(pub parse_time <Time>, chain!(
    hms: hms ~
    z:  opt!(complete!( alt!( timezone_hour | timezone_utc) )),
    || {
        let tz = z.unwrap_or((None, None));
        Time {
            hours: hms.0,
            minutes: hms.1,
            seconds: hms.2,
            tz_offset_hours: tz.0,
            tz_offset_minutes: tz.1
        }
    }
));

named!(
    sign<i32>,
    alt!(
    tag!("-") => { |_| -1 } |
    tag!("+") => { |_| 1 }
    )
);

named!(
    timezone_hour<(Option<i32>, Option<i32>)>,
    chain!(
        s: sign ~
        h: hour ~
        m: empty_or!(
            chain!(
                tag!(":")? ~ m: minute , || { m }
            ))
        ,
        || { (Some(s * (h as i32)), Some(s * (m.unwrap_or(0) as i32))) }
    )
);

named!(
    timezone_utc<(Option<i32>, Option<i32>)>,
    map!(tag!("Z"), |_| (Some(0), Some(0)))
);

// named!(pub parse_datetime <GedcomxDate>, chain!(d:datetime, || GedcomxDate::Simple(d)));

named!(pub simple_date <GedcomxDate>, chain!(
    a: approximate? ~
    d: datetime, || {
        GedcomxDate::Simple(Simple {
            date: d.date,
            time: d.time,
            approximate: a.is_some()
        })
    }
));

named!(pub datetime <DateTime>, chain!(
    d: parse_date ~
    t: opt!(complete!(chain!(tag!("T") ~ time: parse_time, || time)))
    ,
    || {
        DateTime{
            date: d,
            time: t,
        }
    }
));

#[cfg(test)]
mod tests {

    use nom::IResult::*;

    use super::super::super::{Date, Time};
    use super::{day, month, year};
    use super::{hour, minute, second};
    use super::{parse_time, ymd};

    #[test]
    fn test_year() {
        assert_eq!(Done(&[][..], 2015), year(b"+2015"));
        assert_eq!(Done(&[][..], -333), year(b"-0333"));
        assert_eq!(Done(&b"-"[..], 2015), year(b"+2015-"));

        assert!(year(b"2003").is_err());
        assert!(year(b"+abcd").is_err());
        assert!(year(b"+2a03").is_err());

        assert!(year(b"+203").is_incomplete());
    }

    #[test]
    fn test_month() {
        assert_eq!(Done(&[][..], 1), month(b"01"));
        assert_eq!(Done(&[][..], 6), month(b"06"));
        assert_eq!(Done(&[][..], 12), month(b"12"));
        assert_eq!(Done(&b"-"[..], 12), month(b"12-"));

        assert!(month(b"13").is_err());
        assert!(month(b"00").is_err());
        assert!(month(b"1").is_err());
    }

    #[test]
    fn test_day() {
        assert_eq!(Done(&[][..], 1), day(b"01"));
        assert_eq!(Done(&[][..], 12), day(b"12"));
        assert_eq!(Done(&[][..], 20), day(b"20"));
        assert_eq!(Done(&[][..], 28), day(b"28"));
        assert_eq!(Done(&[][..], 30), day(b"30"));
        assert_eq!(Done(&[][..], 31), day(b"31"));
        assert_eq!(Done(&b"-"[..], 31), day(b"31-"));

        assert!(day(b"1").is_err());
        assert!(day(b"00").is_err());
        assert!(day(b"32").is_err());
    }

    #[test]
    fn test_ymd() {
        assert_eq!(
            Done(
                &[][..],
                Date {
                    year: 1988,
                    month: Some(3),
                    day: Some(29),
                }
            ),
            ymd(b"+1988-03-29")
        );
        assert_eq!(
            Done(
                &[][..],
                Date {
                    year: 1988,
                    month: Some(3),
                    day: None,
                }
            ),
            ymd(b"+1988-03")
        );
        assert_eq!(
            Done(
                &[][..],
                Date {
                    year: 1988,
                    month: None,
                    day: None,
                }
            ),
            ymd(b"+1988")
        );

        // assert!(ymd(b"+1988-3-29").is_err());
        // assert!(ymd(b"+1988-3").is_err());
        // assert!(ymd(b"+1988-").is_err());
    }

    #[test]
    fn test_hour() {
        assert_eq!(Done(&[][..], 0), hour(b"00"));
        assert_eq!(Done(&[][..], 1), hour(b"01"));
        assert_eq!(Done(&[][..], 6), hour(b"06"));
        assert_eq!(Done(&[][..], 12), hour(b"12"));
        assert_eq!(Done(&[][..], 13), hour(b"13"));
        assert_eq!(Done(&[][..], 20), hour(b"20"));
        assert_eq!(Done(&[][..], 24), hour(b"24"));

        assert!(hour(b"25").is_err());
        assert!(hour(b"30").is_err());
        assert!(hour(b"ab").is_err());
    }

    #[test]
    fn test_minute() {
        assert_eq!(Done(&[][..], 0), minute(b"00"));
        assert_eq!(Done(&[][..], 1), minute(b"01"));
        assert_eq!(Done(&[][..], 30), minute(b"30"));
        assert_eq!(Done(&[][..], 59), minute(b"59"));

        assert!(minute(b"60").is_err());
        assert!(minute(b"61").is_err());
        assert!(minute(b"ab").is_err());
    }

    #[test]
    fn test_second() {
        assert_eq!(Done(&[][..], 0), second(b"00"));
        assert_eq!(Done(&[][..], 1), second(b"01"));
        assert_eq!(Done(&[][..], 30), second(b"30"));
        assert_eq!(Done(&[][..], 59), second(b"59"));
        assert_eq!(Done(&[][..], 60), second(b"60"));

        assert!(second(b"61").is_err());
        assert!(second(b"ab").is_err());
    }

    #[test]
    fn test_time() {
        assert_eq!(
            Done(
                &[][..],
                Time {
                    hours: 10,
                    minutes: None,
                    seconds: None,
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            ),
            parse_time(b"10")
        );
        assert_eq!(
            Done(
                &[][..],
                Time {
                    hours: 10,
                    minutes: Some(30),
                    seconds: None,
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            ),
            parse_time(b"10:30")
        );
        assert_eq!(
            Done(
                &[][..],
                Time {
                    hours: 10,
                    minutes: Some(30),
                    seconds: Some(29),
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            ),
            parse_time(b"10:30:29")
        );
        assert_eq!(
            Done(
                &b":1:01"[..],
                Time {
                    hours: 10,
                    minutes: None,
                    seconds: None,
                    tz_offset_hours: None,
                    tz_offset_minutes: None,
                }
            ),
            parse_time(b"10:1:01")
        );

        // assert!(parse_time(b"10:1:01").is_err());
    }
}
