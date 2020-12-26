use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion};
use kml::Kml;

fn parse_benchmark(c: &mut Criterion) {
    c.bench_function("parse (countries.kml)", |bencher| {
        let kml_str = include_str!("../fixtures/countries.kml");
        bencher.iter(|| {
            let _ = Kml::<f64>::from_str(kml_str).unwrap();
        });
    });

    c.bench_function("parse (sample.kml)", |bencher| {
        let kml_str = include_str!("../fixtures/sample.kml");
        bencher.iter(|| {
            let _ = Kml::<f64>::from_str(kml_str).unwrap();
        });
    });
}

criterion_group!(benches, parse_benchmark);
criterion_main!(benches);
