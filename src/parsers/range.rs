use super::super::{GedcomxDate, Range};
use super::approximate;
use super::simple::datetime;
use super::{datetime_or_duration, parse_datetime};

named!(range_marker<bool>, map!(tag!("/"), |_| true));

named!(pub range <GedcomxDate>,
    chain!(
        a: opt!(approximate) ~
        dates: alt_complete!(
            chain!(start:datetime ~ complete!(range_marker) ~ end:datetime_or_duration, || (Some(start), Some(end))) |
            chain!(complete!(range_marker) ~ end:parse_datetime, || (None, Some(end))) |
            chain!(start:datetime ~ complete!(range_marker), || (Some(start), None))
        ),
        || GedcomxDate::Range(Range {start: dates.0, end: dates.1, approximate: a.is_some() })
));
