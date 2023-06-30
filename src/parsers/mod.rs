use super::{DateTimeOrDuration, GedcomxDate};

macro_rules! empty_or(
    ($i:expr, $submac:ident!( $($args:tt)* )) => (
        if $i.len() == 0 {
            Ok(($i, None))
        } else {
              match $submac!($i, $($args)*) {
                Ok((i, o)) => Ok((i, Some(o))),
                Err(super::super::nom::Err::Incomplete(i)) => Err(super::super::nom::Err::Incomplete(i)),
                Err(super::super::nom::Err::Error(_)) => Ok(($i, None)),
                Err(super::super::nom::Err::Failure(_)) => Ok(($i, None)),
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
        Err(super::super::nom::Err::Error(super::super::nom::Context::Code($input, super::super::nom::ErrorKind::Custom(20u32))))
      } else {
        Ok((&b""[..], $input))
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
