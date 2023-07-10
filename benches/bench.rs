use criterion::{criterion_group, criterion_main, Criterion};
use gedcomx_date::parse;

fn bench_simple_date(c: &mut Criterion) {
    c.bench_function("bench_simple_date", |c| c.iter(|| parse("+2003-10-12")));
}

fn bench_approximate_simple_date(c: &mut Criterion) {
    c.bench_function("bench_approximate_simple_date", |c| {
        c.iter(|| parse("A+2003-10-12"))
    });
}

fn bench_range(c: &mut Criterion) {
    c.bench_function("bench_range", |c| {
        c.iter(|| parse("+2003-10-12/+2003-10-25"))
    });
}

fn bench_open_range(c: &mut Criterion) {
    c.bench_function("bench_open_range", |c| c.iter(|| parse("+2003-10-12/")));
}

fn bench_recurring(c: &mut Criterion) {
    c.bench_function("bench_recurring", |c| {
        c.iter(|| parse("R/+2003-10-12/+2003-10-25"))
    });
}

criterion_group!(
    benches,
    bench_simple_date,
    bench_approximate_simple_date,
    bench_range,
    bench_open_range,
    bench_recurring
);
criterion_main!(benches);
