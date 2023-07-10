use gedcomx_date::*;

#[test]
fn test_just_r() {
    assert!(parse("R").is_err());
    assert!(parse("R/").is_err());
}

#[test]
fn test_many_slashes() {
    assert!(parse("R/+1000/+2000/+3000").is_err());
}

#[test]
fn test_invalid_part1() {
    assert!(parse("R/+1000_10/+1001").is_err());
}

#[test]
fn test_duration_only() {
    assert!(parse("R/P100Y").is_err());
}

#[test]
fn test_invalid_duration() {
    assert!(parse("R/+1000/P100Q").is_err());
}

#[test]
fn test_invalid_part2() {
    assert!(parse("R/+1000/+1000_10").is_err());
}

#[test]
fn test_simple() {
    assert_eq!(
        parse("R/+1000/+2000-10-01")
            .unwrap()
            .get_recurring()
            .unwrap(),
        Recurring {
            count: None,
            start: DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            },
            end: DateTimeOrDuration::DateTime(DateTime {
                date: Date {
                    year: 2000,
                    month: Some(10),
                    day: Some(1),
                },
                time: None,
            }),
        }
    );
}

#[test]
fn test_count() {
    assert_eq!(
        parse("R3/+1000/+2000-10-01")
            .unwrap()
            .get_recurring()
            .unwrap(),
        Recurring {
            start: DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            },
            end: DateTimeOrDuration::DateTime(DateTime {
                date: Date {
                    year: 2000,
                    month: Some(10),
                    day: Some(1),
                },
                time: None,
            }),
            count: Some(3),
        }
    );
}

#[test]
fn test_duration() {
    assert_eq!(
        parse("R/+1000/P1Y2M3DT4H5M6S")
            .unwrap()
            .get_recurring()
            .unwrap(),
        Recurring {
            start: DateTime {
                date: Date {
                    year: 1000,
                    month: None,
                    day: None,
                },
                time: None,
            },
            end: DateTimeOrDuration::Duration(Duration {
                years: 1,
                months: 2,
                days: 3,
                hours: 4,
                minutes: 5,
                seconds: 6,
            }),
            count: None,
        }
    );
}

#[test]
fn test_no_date_approximate() {
    assert!(parse("+1000/A+2000-10-01").is_err());
    assert!(parse("AA+1000/A+2000-10-01").is_err());
    assert!(parse("AA+1000/+2000-10-01").is_err());
}
