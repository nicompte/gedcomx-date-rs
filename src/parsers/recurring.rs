use super::{datetime, datetime_or_duration};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::sequence::Tuple;
use nom::IResult;

use crate::{DateTime, DateTimeOrDuration};

use super::super::{GedcomxDate, Recurring};

use std::str;

fn u32_digit(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

fn dates(i: &str) -> IResult<&str, (DateTime, DateTimeOrDuration)> {
    let (remaining, (start, _, end)) = (datetime, tag("/"), datetime_or_duration).parse(i)?;
    Ok((remaining, (start, end)))
}

pub fn recurring(i: &str) -> IResult<&str, GedcomxDate> {
    let (remaining, (_, c, _, dates)) = (tag("R"), opt(u32_digit), tag("/"), dates).parse(i)?;
    Ok((
        remaining,
        GedcomxDate::Recurring(Recurring {
            start: dates.0,
            end: dates.1,
            count: c,
        }),
    ))
}
