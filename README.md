# gedcomx-date-rs

[![Circle CI](https://circleci.com/gh/nicompte/gedcomx-date-rs.svg?style=svg)](https://circleci.com/gh/nicompte/gedcomx-date-rs)

Gedcomx date parsing written with [nom](https://github.com/Geal/nom) in [rust](https://rust-lang.org).
See documentation [here](http://barbotte.net/gedcomx-date-rs/doc/gedcomx_date).

```rust
let date = gedcomx_date::parse("2015-06-26T16:43:23+02:00").unwrap();
```

Will give you:

```rust
GedcomxDate::Simple {
    date: Date {
        year: 2015,
        month: Some(6),
        day: Some(26),
    },
    time: Some(Time {
        hours: 16,
        minutes: Some(43),
        seconds: Some(23),
        tz_offset_hours: Some(2),
        tz_offset_minutes: Some(0),
    }),
};
```

## Usage

Update your `Cargo.toml`:

```toml
[dependencies]
gedcomx_date = "0.0.1"
```

```rust
fn main() {
    let date = gedcomx_date::parse("+1988-03-29T03:19+01");
    print!("{:?}", date);
}
```

## TODO

- don't cheat on the duration tests
- validate dates (i.e. 30/02/2016)
- validate hours (i.e. 24:31)
- report parsing errors

## License

MIT License Copyright (c) 2016 Nicolas Barbotte
