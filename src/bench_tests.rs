#[macro_export]
macro_rules! unit_bench {
    () => {
        //const ONE: &str = "1bigline.wsv";
        const TWO: &str = "100000lines.wsv";

        use divan::black_box;
        #[divan::bench(args = [TWO], sample_count = 1000, sample_size = 1000)]
        fn micro(path: &str) {
            black_box(parse(black_box(&["./tests/example_files/", &path].concat())).unwrap());
        }
    };
}
