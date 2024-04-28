use std::fs::read_to_string;

use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};

pub use wsv::primitive;
pub use wsv::split;
pub use wsv::nom;
pub use wsv::pest;

fn bench_static_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("StaticParse");

    let contents =
    read_to_string::<String>("./tests/example_files/".to_string() + "big.wsv").unwrap();
    let inp = contents.as_str();

    let contents2 =
    read_to_string::<String>("./tests/example_files/".to_string() + "nulls.wsv").unwrap();
    let inp2 = contents2.as_str();

    for &i in [("big", inp), ("nulls", inp2)].iter() {
        group.bench_with_input(BenchmarkId::new("primitive", i.0), i.1, 
            |b, i| b.iter(|| primitive::parse(i).unwrap()));
        group.bench_with_input(BenchmarkId::new("primitive2", i.0), i.1, 
            |b, i| b.iter(|| split::parse(i).unwrap()));
        group.bench_with_input(BenchmarkId::new("nom", i.0), i.1, 
            |b, i| b.iter(|| nom::parse(i).unwrap()));
        group.bench_with_input(BenchmarkId::new("pest", i.0), i.1, 
            |b, i| b.iter(|| pest::parse(i).unwrap()));
    }
    group.finish();
}

criterion_group!(benches, bench_static_parse);
criterion_main!(benches);