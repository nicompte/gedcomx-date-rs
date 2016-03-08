
extern crate gedcomx_date;
extern crate nom;

use gedcomx_date::*;

#[test]
fn test_empty() {
    assert!(parse("").is_err());
}

#[test]
fn test_year() {
    assert!(parse("").is_err());
    assert!(parse("2000-03-01").is_err());
    assert!(parse("+1ooo").is_err());
    match parse("+1000").unwrap() {
        GedcomxDate::Simple(date) => {
            assert_eq!(date.date.year, 1000);
            assert_eq!(date.date.month, None);
            assert_eq!(date.date.day, None);
            assert_eq!(date.time, None);
        }
        _ => {}
    };

    // +YYYY
    assert_eq!(parse("+1000").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: None,
                       day: None,
                   },
                   time: None,
               });

    // -YYYY
    assert_eq!(parse("-0010").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: -10,
                       month: None,
                       day: None,
                   },
                   time: None,
               });

    // -YYYY
    assert_eq!(parse("-0000").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 0,
                       month: None,
                       day: None,
                   },
                   time: None,
               });

    // +YYYYTHH
    assert_eq!(parse("+1000T10").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: None,
                       day: None,
                   },
                   time: Some(Time {
                       hours: 10,
                       minutes: None,
                       seconds: None,
                       tz_offset_hours: None,
                       tz_offset_minutes: None,
                   }),
               });
    assert!(parse("+1000_10").is_err());
}

#[test]
fn test_month() {
    assert!(parse("+1000-1").is_err());
    assert!(parse("+1000-o1-01").is_err());
    assert!(parse("+1000-00").is_err());
    assert!(parse("+1000-13").is_err());

    // +YYYY-MM
    assert_eq!(parse("+0987-04").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 987,
                       month: Some(4),
                       day: None,
                   },
                   time: None,
               });

    // +YYYY-MMTHH
    assert_eq!(parse("+1000-10T10").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(10),
                       day: None,
                   },
                   time: Some(Time {
                       hours: 10,
                       minutes: None,
                       seconds: None,
                       tz_offset_hours: None,
                       tz_offset_minutes: None,
                   }),
               });
    assert!(parse("+1000-10=01").is_err());
}

#[test]
fn test_day() {
    assert!(parse("+1000-10-1").is_err());
    assert!(parse("+1000-01-o1").is_err());
    assert!(parse("+1000-10-00").is_err());
    // TODO: number of days in a month
    // assert!(parse(parse("+1000-04-31").is_err());

    // +YYYY-MM-DD
    assert_eq!(parse("+1600-02-29").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1600,
                       month: Some(2),
                       day: Some(29),
                   },
                   time: None,
               });
    assert!(parse("+1492-03-1501:02:03").is_err());
}

#[test]
fn test_hours() {
    assert!(parse("+1000-10-11T1").is_err());
    assert!(parse("+1000-01-11T1o").is_err());
    assert!(parse("+1000-10-01T25").is_err());

    // +YYYY-MM-DDTHHZ
    assert_eq!(parse("+0987-01-25T24Z").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 987,
                       month: Some(1),
                       day: Some(25),
                   },
                   time: Some(Time {
                       hours: 24,
                       minutes: None,
                       seconds: None,
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(0),
                   }),
               });
    assert!(parse("+1000-10-01T10|30").is_err());
}

#[test]
fn test_minutes() {
    assert!(parse("+1000-10-11T10:1").is_err());
    assert!(parse("+1000-01-11T10:1o").is_err());
    assert!(parse("+1000-10-01T23:60").is_err());

    // +YYYY-MM-DDTHH:MM
    assert_eq!(parse("+1000-10-01T24:15").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(10),
                       day: Some(1),
                   },
                   time: Some(Time {
                       hours: 24,
                       minutes: Some(15),
                       seconds: None,
                       tz_offset_hours: None,
                       tz_offset_minutes: None,
                   }),
               });

    // +YYYY-MM-DDTHH:MM
    assert_eq!(parse("+0987-01-25T23:59").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 987,
                       month: Some(1),
                       day: Some(25),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(59),
                       seconds: None,
                       tz_offset_hours: None,
                       tz_offset_minutes: None,
                   }),
               });

    // +YYYY-MM-DDTHH:MMZ
    assert_eq!(parse("+0987-01-25T23:59Z").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 987,
                       month: Some(1),
                       day: Some(25),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(59),
                       seconds: None,
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(0),
                   }),
               });
    assert!(parse("+1000-10-01T10:30|15").is_err());
}

#[test]
fn test_seconds() {
    assert!(parse("+1000-10-11T10:15:1").is_err());
    assert!(parse("+1000-01-11T10:15:1o").is_err());
    // TODO: 23:24:60
    // assert!(parse("+1000-10-01T23:24:60").is_err());
    // TODO: 24:00:01
    // assert!(parse("+1000-10-01T24:00:01").is_err());

    // +YYYY-MM-DDTHH:MM:SS
    assert_eq!(parse("+0987-01-25T23:59:59").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 987,
                       month: Some(1),
                       day: Some(25),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(59),
                       seconds: Some(59),
                       tz_offset_hours: None,
                       tz_offset_minutes: None,
                   }),
               });

    // +YYYY-MM-DDTHH:MM:SSZ
    assert_eq!(parse("+0987-01-25T23:59:59Z").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 987,
                       month: Some(1),
                       day: Some(25),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(59),
                       seconds: Some(59),
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(0),
                   }),
               });
}

#[test]
fn test_tzhours() {
    // +YYYY-MM-DDTHH:MMZ
    assert_eq!(parse("+1000-01-01T23:15Z").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(1),
                       day: Some(1),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(15),
                       seconds: None,
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(0),
                   }),
               });

    assert!(parse("+1000-10-01T24:00:00ZSTUFF").is_err());
    assert!(parse("+1000-10-01T24:00:00-1").is_err());
    assert!(parse("+1000-10-01T24:00:00_10").is_err());
    assert!(parse("+1000-01-11T10:15:10+1o").is_err());

    // +YYYY-MM-DDTHH:MM+HH
    assert_eq!(parse("+1000-01-01T23:15+15").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(1),
                       day: Some(1),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(15),
                       seconds: None,
                       tz_offset_hours: Some(15),
                       tz_offset_minutes: Some(0),
                   }),
               });

    // +YYYY-MM-DDTHH:MM-HH
    assert_eq!(parse("+1000-01-01T23:15-02").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(1),
                       day: Some(1),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(15),
                       seconds: None,
                       tz_offset_hours: Some(-2),
                       tz_offset_minutes: Some(0),
                   }),
               });

    // +YYYY-MM-DDTHH:MM-HH
    assert_eq!(parse("+1000-01-01T23:15-00").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(1),
                       day: Some(1),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(15),
                       seconds: None,
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(0),
                   }),
               });

    assert!(parse("+1000-10-01T10:30:15-06-30").is_err());
}

#[test]
fn test_tzminutes() {
    assert!(parse("+1000-10-01T24:00:00-10:1").is_err());
    assert!(parse("+1000-01-11T10:15:10+10:1o").is_err());
    assert!(parse("+1000-01-11T10:15:10+10:10blah").is_err());

    // +YYYY-MM-DDTHH:MM-HH:MM
    // TODO: check minutes offset: -30?
    assert_eq!(parse("+1000-01-01T23:15-00:30").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: false,
                   date: Date {
                       year: 1000,
                       month: Some(1),
                       day: Some(1),
                   },
                   time: Some(Time {
                       hours: 23,
                       minutes: Some(15),
                       seconds: None,
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(-30),
                   }),
               });
}

#[test]
fn test_approximate() {
    assert!(parse("A1000").is_err());

    // A+YYYY-MM-DDTHHZ
    assert_eq!(parse("A+0987-01-25T24Z").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: true,
                   date: Date {
                       year: 987,
                       month: Some(1),
                       day: Some(25),
                   },
                   time: Some(Time {
                       hours: 24,
                       minutes: None,
                       seconds: None,
                       tz_offset_hours: Some(0),
                       tz_offset_minutes: Some(0),
                   }),
               });

    // A+YYYY
    assert_eq!(parse("A+0987").unwrap().get_simple_date().unwrap(),
               Simple {
                   approximate: true,
                   date: Date {
                       year: 987,
                       month: None,
                       day: None,
                   },
                   time: None,
               });
}
