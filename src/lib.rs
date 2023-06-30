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

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "bench", feature(test))]

#[macro_use]
extern crate nom;
use nom::Err;

#[macro_use]
mod helper;
#[macro_use]
mod parsers;
mod bench;

// used for benchmarks
#[cfg(feature = "bench")]
extern crate test;

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
    match parsers::parse(string.as_bytes()) {
        Ok((_, parsed)) => Ok(parsed),
        Err(Err::Incomplete(_)) => Err("Parsing error".to_string()),
        Err(Err::Error(e)) => Err("Parsing error".to_string()),
        Err(Err::Failure(_)) => Err("Parsing error".to_string()),
    }
}
