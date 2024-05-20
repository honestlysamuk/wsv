// use std::fs::read_to_string;

// use criterion::BenchmarkId;
// use criterion::{criterion_group, criterion_main, Criterion};

// pub use wsv::first;
// pub use wsv::mealy;
// pub use wsv::moore;
// #[cfg(feature = "nom")]
// pub use wsv::nom;
// #[cfg(feature = "pest")]
// pub use wsv::pest;
// pub use wsv::split;
// pub use wsv::state;
//use wsv::WsvValue;
//use wsv::{parse_from_buf_file, parse_from_whole_file};

// fn bench_parse_variants(c: &mut Criterion) {
//     let mut group = c.benchmark_group("StaticParse");

//     let test_files = ["100000lines", "100lines", "1bigline"];

//     let input_list = test_files.iter().map(|&name| {
//         (
//             name,
//             read_to_string::<String>("./tests/example_files/".to_string() + name + ".wsv").unwrap(),
//         )
//     });
//     for (name, inp) in input_list {
//         group.bench_with_input(BenchmarkId::new("state", name), inp.as_str(), |b, i| {
//             b.iter(|| state::parse(i).unwrap())
//         });
//         group.bench_with_input(BenchmarkId::new("nom", name), inp.as_str(), |b, i| {
//             b.iter(|| nom::parse(i).unwrap())
//         });
//         group.bench_with_input(BenchmarkId::new("first", name), inp.as_str(), |b, i| {
//             b.iter(|| first::parse(i).unwrap())
//         });
//         group.bench_with_input(BenchmarkId::new("split", name), inp.as_str(), |b, i| {
//             b.iter(|| split::parse(i).unwrap())
//         });
//         group.bench_with_input(BenchmarkId::new("pest", name), inp.as_str(), |b, i| {
//             b.iter(|| pest::parse(i).unwrap())
//         });
//     }
//     group.finish();
// }

// fn bench_input_variants(c: &mut Criterion) {
//     let mut group = c.benchmark_group("StreamParse");

//     let test_files = ["100000lines"];

//     let contents_list = test_files.iter().map(|&name| {
//         (
//             name,
//             read_to_string::<String>("./tests/example_files/".to_string() + name + ".wsv").unwrap(),
//         )
//     });

//     let path_list = test_files
//         .iter()
//         .map(|&name| (name, "./tests/example_files/".to_string() + name + ".wsv"));

//     let path_list2 = test_files
//         .iter()
//         .map(|&name| (name, "./tests/example_files/".to_string() + name + ".wsv"));

//     // for (name, inp) in contents_list {
//     //     group.bench_with_input(BenchmarkId::new("memory", name), inp.as_str(), |b, i| {
//     //         b.iter(|| {
//     //             parse_from_memory(i).unwrap();
//     //         })
//     //     });
//     // }
//     // for (name, stream) in path_list {
//     //     let id = BenchmarkId::new("stream", name);
//     //     group.bench_with_input(id, &stream, |b, i| {
//     //         b.iter(|| {
//     //             parse_from_buf_file(i).unwrap();
//     //         })
//     //     });
//     // }
//     // for (name, file) in path_list2 {
//     //     group.bench_with_input(BenchmarkId::new("file", name), &file, |b, i| {
//     //         b.iter(|| {
//     //             parse_from_whole_file(i).unwrap();
//     //         })
//     //     });
//     // }
//     group.finish();
// }

// criterion_group!(benches, bench_input_variants);
// criterion_main!(benches);
