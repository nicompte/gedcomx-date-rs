
#[cfg(feature = "bench")]
mod bench {
    use test::Bencher;
    use super::super::*;

    #[bench]
    fn bench_simple_date(b: &mut Bencher) {
        b.iter(|| parse("+2003-10-12"));
    }

    #[bench]
    fn bench_approximate_simple_date(b: &mut Bencher) {
        b.iter(|| parse("A+2003-10-12"));
    }

    #[bench]
    fn bench_range(b: &mut Bencher) {
        b.iter(|| parse("+2003-10-12/+2003-10-25"));
    }

    #[bench]
    fn bench_open_range(b: &mut Bencher) {
        b.iter(|| parse("+2003-10-12/"));
    }

    #[bench]
    fn bench_recurring(b: &mut Bencher) {
        b.iter(|| parse("R/+2003-10-12/+2003-10-25"));
    }
}
