
use nom::digit;

use super::super::{GedcomxDate, Recurring};
use super::simple::datetime;
use super::datetime_or_duration;

named!(range_marker <bool>, map!(tag!("/"), |_| true));

use std::str;
use std::str::FromStr;

named!(u32_digit<u32>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(pub recurring <GedcomxDate>,
    chain!(
        complete!(tag!("R")) ~
        c: u32_digit? ~
        tag!("/") ~
        dates: alt_complete!(
            chain!(start:datetime ~ complete!(range_marker) ~ end:datetime_or_duration, || (start, end))
        ),
        || GedcomxDate::Recurring(Recurring {start: dates.0, end: dates.1, count: c})
));
