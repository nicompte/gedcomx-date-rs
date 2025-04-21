//! gedcomx-date-rs is a [rust](https://rust-lang.org) parser library for the
//! [Gedcomx date format](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md) format.
//!
//!
//! The code is available on [github](https://github.com/nicompte/gedcomx_date_rs).
//!
//! # Example
//!
//! ```rust
//! use gedcomx_date::{parse, GedcomxDate};
//! let date = parse("+1988-03-29T03:19").unwrap();
//! match date {
//!     GedcomxDate::Simple(simple_date) => {
//!         let date = simple_date.date;
//!         println!("{}", date.year); // 1988
//!         println!("{}", date.month.unwrap()); // 3
//!         println!("{}", date.day.unwrap()); // 29
//!         let time = simple_date.time.unwrap();
//!         println!("{}", time.hours); // 3
//!         println!("{}", time.minutes.unwrap()); // 19
//!     },
//!    _ => {}
//! }
//! ```

use std::fmt::{Debug, Display};

use nom::Err;

mod parsers;

/// A date object
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Date {
    pub year: i32,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

/// A time object
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Time {
    pub hours: u32,
    pub minutes: Option<u32>,
    pub seconds: Option<u32>,
    pub tz_offset_hours: Option<i32>,
    pub tz_offset_minutes: Option<i32>,
}

/// Simple date
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Simple {
    pub date: Date,
    pub time: Option<Time>,
    pub approximate: bool,
}

/// DateTime, same as simple date, but cannot be approximate.
/// Used for ranges and approximate
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct DateTime {
    pub date: Date,
    pub time: Option<Time>,
}

/// Duration
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Duration {
    pub years: u32,
    pub months: u32,
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

/// Gedcomx date
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum DateTimeOrDuration {
    DateTime(DateTime),
    Duration(Duration),
}

/// Range
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Range {
    pub start: Option<DateTime>,
    pub end: Option<DateTimeOrDuration>,
    pub approximate: bool,
}

/// Recurring
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Recurring {
    pub start: DateTime,
    pub end: DateTimeOrDuration,
    pub count: Option<u32>,
}

/// Gedcomx date
/// Enum that holds the three types of gedcomx dates
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum GedcomxDate {
    /// Simple date. See
    /// [5.2 Simple date](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md#52-simple-date),
    /// [5.7 Approximate date](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md#57-approximate-date).
    Simple(Simple),
    /// Date range. See
    /// [5.4 Closed date range](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md#54-closed-date-range),
    /// [5.5 Open ended date range](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md#55-open-ended-date-range),
    /// [5.8 Approximate date range](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md#58-approximate-date-range).
    Range(Range),
    /// Recurring date. See
    /// [5.6 Recurring date](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md#56-recurring-date).
    Recurring(Recurring),
}

/// GedxomxDate implementation. You can match over the parse result, or use these methods.
impl GedcomxDate {
    pub fn get_simple_date(&self) -> Option<Simple> {
        match *self {
            GedcomxDate::Simple(date) => Some(date),
            _ => None,
        }
    }
    pub fn get_range(&self) -> Option<Range> {
        match *self {
            GedcomxDate::Range(date) => Some(date),
            _ => None,
        }
    }
    pub fn get_recurring(&self) -> Option<Recurring> {
        match *self {
            GedcomxDate::Recurring(date) => Some(date),
            _ => None,
        }
    }
}

/// Parses a string and extracts a Gedcomx date.
///
///
/// ## Example
///
/// ```rust
/// use gedcomx_date::{parse, GedcomxDate};
/// let date = parse("+1988-03-29T03:19").unwrap();
/// match date {
///     GedcomxDate::Simple(simple_date) => {
///         let date = simple_date.date;
///         println!("{}", date.year); // 1988
///         println!("{}", date.month.unwrap()); // 3
///         println!("{}", date.day.unwrap()); // 29
///         let time = simple_date.time.unwrap();
///         println!("{}", time.hours); // 3
///         println!("{}", time.minutes.unwrap()); // 19
///     },
///    _ => {}
/// }
/// ```
pub fn parse(string: &str) -> Result<GedcomxDate, String> {
    match parsers::parse(string) {
        Ok((remaining, parsed)) => {
            if remaining.is_empty() {
                Ok(parsed)
            } else {
                Err("Parsing error".to_string())
            }
        }
        Err(Err::Incomplete(_)) => Err("Parsing error".to_string()),
        Err(Err::Error(_)) => Err("Parsing error".to_string()),
        Err(Err::Failure(_)) => Err("Parsing error".to_string()),
    }
}

impl std::fmt::Display for GedcomxDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::fmt::Display;
        match self {
            GedcomxDate::Simple(simple) => Display::fmt(simple, f),
            GedcomxDate::Range(range) => Display::fmt(range, f),
            GedcomxDate::Recurring(recurring) => Display::fmt(recurring, f),
        }
    }
}

impl std::fmt::Display for Simple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.approximate {
            f.write_str("A")?;
        }
        f.write_fmt(format_args!("{}", self.date))?;

        if let Some(time) = self.time {
            f.write_fmt(format_args!("T{}", time))?;
        }

        Ok(())
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.approximate {
            f.write_str("A")?;
        }
        if let Some(start) = self.start {
            f.write_fmt(format_args!("{}", start))?;
        }
        f.write_str("/")?;
        if let Some(end) = self.end {
            f.write_fmt(format_args!("{}", end))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Recurring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("R")?;
        if let Some(count) = self.count {
            f.write_fmt(format_args!("{}", count))?;
        }
        f.write_fmt(format_args!("/{}", self.start))?;
        f.write_fmt(format_args!("/{}", self.end))?;
        Ok(())
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.date))?;
        if let Some(time) = self.time {
            f.write_fmt(format_args!("T{}", time))?;
        }
        Ok(())
    }
}

impl std::fmt::Display for DateTimeOrDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DateTime(datetime) => Display::fmt(datetime, f),
            Self::Duration(duration) => Display::fmt(duration, f),
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.year < 0 { "-" } else { "+" })?;
        f.write_fmt(format_args!("{:04}", self.year.abs()))?;
        if let Some(month) = self.month {
            f.write_fmt(format_args!("-{:02}", month))?;
            if let Some(day) = self.day {
                f.write_fmt(format_args!("-{:02}", day))?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("P")?;
        if self.years > 0 {
            f.write_fmt(format_args!("{}Y", self.years))?;
        }
        if self.months > 0 {
            f.write_fmt(format_args!("{}M", self.months))?;
        }
        if self.days > 0 {
            f.write_fmt(format_args!("{}D", self.days))?;
        }
        if self.hours > 0 || self.minutes > 0 || self.seconds > 0 {
            f.write_str("T")?;
        }
        if self.hours > 0 {
            f.write_fmt(format_args!("{}H", self.hours))?;
        }
        if self.minutes > 0 {
            f.write_fmt(format_args!("{}M", self.minutes))?;
        }
        if self.seconds > 0 {
            f.write_fmt(format_args!("{}S", self.seconds))?;
        }
        Ok(())
    }
}
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}", self.hours))?;
        if let Some(minutes) = self.minutes {
            f.write_fmt(format_args!(":{:02}", minutes))?;
        }
        if let Some(seconds) = self.seconds {
            f.write_fmt(format_args!(":{:02}", seconds))?;
        }
        match (self.tz_offset_hours, self.tz_offset_minutes) {
            (Some(tz_offset_hours), None) => {
                f.write_fmt(format_args!("{:02}", tz_offset_hours))?;
            }
            (Some(tz_offset_hours), Some(tz_offset_minutes)) => {
                dbg!(&tz_offset_hours, &tz_offset_minutes);
                if tz_offset_hours == 0 && tz_offset_minutes == 0 {
                    f.write_str("Z")?;
                } else {
                    f.write_fmt(format_args!(
                        "{}{:02}",
                        if tz_offset_hours < 0 { "-" } else { "+" },
                        tz_offset_hours.abs()
                    ))?;
                    if tz_offset_minutes != 0 {
                        f.write_fmt(format_args!(":{:02}", tz_offset_minutes.abs()))?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
