use super::{DateTimeOrDuration, GedcomxDate};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
mod duration;
mod range;
mod recurring;
mod simple;

use self::duration::duration;
use self::range::range;
use self::recurring::recurring;
use self::simple::{datetime, simple_date};
use nom::IResult;

fn parse_datetime(i: &str) -> IResult<&str, DateTimeOrDuration> {
    map(datetime, DateTimeOrDuration::DateTime)(i)
}

fn parse_duration(i: &str) -> IResult<&str, DateTimeOrDuration> {
    map(duration, DateTimeOrDuration::Duration)(i)
}

fn datetime_or_duration(i: &str) -> IResult<&str, DateTimeOrDuration> {
    alt((parse_duration, parse_datetime))(i)
}

fn approximate(i: &str) -> IResult<&str, bool> {
    map(tag("A"), |_| true)(i)
}

/// main parse function
/// parse either a recurring, a range, or a simple date
pub fn parse(i: &str) -> IResult<&str, GedcomxDate> {
    alt((recurring, range, simple_date))(i)
}
