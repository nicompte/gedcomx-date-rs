extern crate gedcomx_date;
extern crate nom;

use gedcomx_date::*;

#[test]
fn test_just_slash() {
    assert!(parse("/").is_err());
}

#[test]
fn test_many_slashes() {
    assert!(parse("+1000/+2000/+3000").is_err());
}

#[test]
fn test_invalid_part1() {
    assert!(parse("+1000_10/").is_err());
}

#[test]
fn test_duration_only() {
    assert!(parse("/P100Y").is_err());
}

#[test]
fn test_invalid_duration() {
    assert!(parse("+1000/P100Q").is_err());
}

#[test]
fn test_invalid_part2() {
    assert!(parse("/+1000_10").is_err());
}

#[test]
fn test_duration() {
    assert_eq!(
        parse("+1000/P1Y2M3DT4H5M6S").unwrap().get_range().unwrap(),
        Range {
            start: Some(DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            }),
            end: Some(DateTimeOrDuration::Duration(Duration {
                years: 1,
                months: 2,
                days: 3,
                hours: 4,
                minutes: 5,
                seconds: 6,
            })),
            approximate: false,
        }
    );
}

#[test]
fn test_closed() {
    assert_eq!(
        parse("+1000/+2000-10-01").unwrap().get_range().unwrap(),
        Range {
            start: Some(DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            }),
            end: Some(DateTimeOrDuration::DateTime(DateTime {
                date: Date {
                    year: 2000,
                    month: Some(10),
                    day: Some(1),
                },
                time: None,
            })),
            approximate: false,
        }
    );
}

#[test]
fn test_open_start() {
    assert_eq!(
        parse("/+2000-10-01").unwrap().get_range().unwrap(),
        Range {
            start: None,
            end: Some(DateTimeOrDuration::DateTime(DateTime {
                date: Date {
                    year: 2000,
                    month: Some(10),
                    day: Some(1),
                },
                time: None,
            })),
            approximate: false,
        }
    );
}

#[test]
fn test_open_end() {
    assert_eq!(
        parse("+1000/").unwrap().get_range().unwrap(),
        Range {
            start: Some(DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            }),
            end: None,
            approximate: false,
        }
    );
}

#[test]
fn test_approximate() {
    assert_eq!(
        parse("A+1000/+2000-10-01").unwrap().get_range().unwrap(),
        Range {
            start: Some(DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            }),
            end: Some(DateTimeOrDuration::DateTime(DateTime {
                date: Date {
                    year: 2000,
                    month: Some(10),
                    day: Some(1),
                },
                time: None,
            })),
            approximate: true,
        }
    );
}

#[test]
fn test_no_date_approximate() {
    assert!(parse("+1000/A+2000-10-01").is_err());
    assert!(parse("AA+1000/A+2000-10-01").is_err());
    assert!(parse("AA+1000/+2000-10-01").is_err());
}
