use std::fs::read_to_string;

use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};

pub use wsv::first;
pub use wsv::split;
pub use wsv::nom;
pub use wsv::pest;
pub use wsv::state;


fn bench_static_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("StaticParse");

    let test_files = ["empty_string", "big"];

    let input_list = test_files.iter().map(|name| {
        (name, read_to_string::<String>("./tests/example_files/".to_string() + name + ".wsv").unwrap())
    });
    for (name, contents) in input_list {
        group.bench_with_input(BenchmarkId::new("first", name), contents.as_str(), 
            |b, i| b.iter(|| first::parse(i).unwrap()));
        group.bench_with_input(BenchmarkId::new("split", name), contents.as_str(), 
            |b, i| b.iter(|| split::parse(i).unwrap()));
        group.bench_with_input(BenchmarkId::new("nom", name), contents.as_str(), 
            |b, i| b.iter(|| nom::parse(i).unwrap()));
        group.bench_with_input(BenchmarkId::new("pest", name), contents.as_str(), 
            |b, i| b.iter(|| pest::parse(i).unwrap()));
            group.bench_with_input(BenchmarkId::new("state", name), contents.as_str(), 
            |b, i| b.iter(|| state::parse(i).unwrap()));
    }
    group.finish();
}

criterion_group!(benches, bench_static_parse);
criterion_main!(benches);