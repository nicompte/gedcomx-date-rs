use super::{DateTimeOrDuration, GedcomxDate};

macro_rules! empty_or(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        if $i.len() == 0 {
            super::super::nom::IResult::Done($i, None)
        } else {
            match $submac!($i, $($args)*) {
                super::super::nom::IResult::Done(i,o)     => super::super::nom::IResult::Done(i, Some(o)),
                super::super::nom::IResult::Error(_)      => super::super::nom::IResult::Done($i, None),
                super::super::nom::IResult::Incomplete(i) => super::super::nom::IResult::Incomplete(i)

            }
        }
    );
);

macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (

    {
      let mut failed = false;
      for idx in 0..$input.len() {
        if !$submac!($input[idx], $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        super::super::nom::IResult::Error(super::super::nom::Err::Position(super::super::nom::ErrorKind::Custom(20),$input))
      } else {
        super::super::nom::IResult::Done(&b""[..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    check!($input, call!($f))
  );
);

macro_rules! char_between(
    ($input:expr, $min:expr, $max:expr) => (
        {
        fn f(c: u8) -> bool { c >= ($min as u8) && c <= ($max as u8)}
        flat_map!($input, take!(1), check!(f))
        }
    );
);

mod duration;
mod range;
mod recurring;
mod simple;

use self::duration::duration;
use self::range::range;
use self::recurring::recurring;
use self::simple::datetime;
use self::simple::simple_date;
use nom::eof;

named!(
    parse_datetime<DateTimeOrDuration>,
    do_parse!(d: datetime >> (DateTimeOrDuration::DateTime(d)))
);
named!(
    parse_duration<DateTimeOrDuration>,
    do_parse!(d: duration >> (DateTimeOrDuration::Duration(d)))
);

named!(pub datetime_or_duration <DateTimeOrDuration>, alt_complete!(parse_duration | parse_datetime));

named!(approximate<bool>, map!(tag!("A"), |_| true));

/// main parse function
/// parse either a recurring, a range, or a simple date
named!(pub parse <GedcomxDate>, do_parse!( d:alt_complete!(recurring | range | simple_date) >> eof!() >> (d)));
