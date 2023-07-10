use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::sequence::Tuple;
use nom::IResult;

use crate::{DateTime, DateTimeOrDuration};

use super::super::{GedcomxDate, Range};
use super::approximate;
use super::simple::datetime;
use super::{datetime_or_duration, parse_datetime};

pub fn full(i: &str) -> IResult<&str, (Option<DateTime>, Option<DateTimeOrDuration>)> {
    let (remaining, (start, _, end)) = (datetime, tag("/"), datetime_or_duration).parse(i)?;
    Ok((remaining, (Some(start), Some(end))))
}

pub fn trailing(i: &str) -> IResult<&str, (Option<DateTime>, Option<DateTimeOrDuration>)> {
    let (remaining, (_, end)) = (tag("/"), parse_datetime).parse(i)?;
    Ok((remaining, (None, Some(end))))
}

pub fn leading(i: &str) -> IResult<&str, (Option<DateTime>, Option<DateTimeOrDuration>)> {
    let (remaining, (start, _)) = (datetime, tag("/")).parse(i)?;
    Ok((remaining, (Some(start), None)))
}

pub fn dates(i: &str) -> IResult<&str, (Option<DateTime>, Option<DateTimeOrDuration>)> {
    alt((full, trailing, leading))(i)
}

pub fn range(i: &str) -> IResult<&str, GedcomxDate> {
    let (remaining, (a, dates)) = (opt(approximate), dates).parse(i)?;
    Ok((
        remaining,
        GedcomxDate::Range(Range {
            start: dates.0,
            end: dates.1,
            approximate: a.is_some(),
        }),
    ))
}
