# Monday 13th May 2024

Below is the full benchmark as of today. I added two more for each mealy and moore machine to see if the form of the state machine logic made any difference. It does consistently, but not by much. However, the procedural (and easier to read) version is the faster one, so I'm keeping that. For those interested, the other version is 

```rs
fn parse_line((row_index, line): (usize, &str)) -> Result<Vec<WsvValue>, Error> {
    let row = row_index + 1;
    
    let mut inputs = line.chars();
    let mut state = State::Default;
    let end_state = State::Finished;
    (0..)
        .map_while(|_| match end_state == state {
            true => None,
            false => {
                let next_input = inputs.next();
                state = transition(state, next_input);
                Some(g(state, &next_input))
            }
        })
        .fold(Data::new(row), |data, o| data.apply(o))
        .reconcile()
}
```

It's always fun to write things in terms of iterators. In this case, I learned how to write a generator! If you're really intent on not declaring any variables before, then you can use scan() instead of map_while(). I found, informally, that the implementation was slower if I used

```rs
Benchmarking StaticParse/state/100lines: Collecting 100 samples in estimated 6.2152 s (25k it
StaticParse/state/100lines
                        time:   [259.35 µs 261.45 µs 265.41 µs]
                        change: [-8.1407% -1.9578% +5.1813%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) high mild
  16 (16.00%) high severe
Benchmarking StaticParse/moore/100lines: Collecting 100 samples in estimated 6.7480 s (15k it
StaticParse/moore/100lines
                        time:   [435.88 µs 436.32 µs 436.82 µs]
                        change: [-6.4684% -3.5648% -1.2239%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe
Benchmarking StaticParse/mooreimp/100lines: Collecting 100 samples in estimated 6.4706 s (15k
StaticParse/mooreimp/100lines
                        time:   [422.20 µs 422.79 µs 423.48 µs]
                        change: [-1.2856% -0.2089% +1.0161%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe
Benchmarking StaticParse/mealy/100lines: Collecting 100 samples in estimated 7.2317 s (15k it
StaticParse/mealy/100lines
                        time:   [465.89 µs 466.74 µs 467.77 µs]
                        change: [-35.640% -32.787% -29.811%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe
Benchmarking StaticParse/mealyimp/100lines: Collecting 100 samples in estimated 6.9167 s (15k
StaticParse/mealyimp/100lines
                        time:   [433.33 µs 433.86 µs 434.49 µs]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe
Benchmarking StaticParse/nom/100lines: Collecting 100 samples in estimated 6.6510 s (15k iter
StaticParse/nom/100lines
                        time:   [437.10 µs 442.47 µs 449.87 µs]
                        change: [-3.9543% -1.3612% +1.9007%] (p = 0.38 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) high mild
  13 (13.00%) high severe
Benchmarking StaticParse/first/100lines: Collecting 100 samples in estimated 5.4687 s (10k it
StaticParse/first/100lines
                        time:   [535.40 µs 539.07 µs 544.83 µs]
                        change: [-3.6884% -1.3879% +1.0266%] (p = 0.24 > 0.05)
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) high mild
  16 (16.00%) high severe
Benchmarking StaticParse/split/100lines: Collecting 100 samples in estimated 6.5393 s (10k it
StaticParse/split/100lines
                        time:   [638.17 µs 643.93 µs 652.06 µs]
                        change: [-14.846% -10.665% -6.7101%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
Benchmarking StaticParse/pest/100lines: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.5s, enable flat sampling, or reduce sample count to 60.
Benchmarking StaticParse/pest/100lines: Collecting 100 samples in estimated 6.4753 s (5050 it
StaticParse/pest/100lines
                        time:   [1.2657 ms 1.2739 ms 1.2871 ms]
                        change: [-11.583% -7.8062% -4.2469%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
Benchmarking StaticParse/state/1bigline: Collecting 100 samples in estimated 5.0392 s (20k it
StaticParse/state/1bigline
                        time:   [206.36 µs 208.71 µs 213.45 µs]
                        change: [-1.9566% +1.3018% +5.5826%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe
Benchmarking StaticParse/moore/1bigline: Collecting 100 samples in estimated 5.7245 s (15k it
StaticParse/moore/1bigline
                        time:   [373.83 µs 375.31 µs 377.70 µs]
                        change: [-1.1716% +0.2485% +1.5932%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe
Benchmarking StaticParse/mooreimp/1bigline: Collecting 100 samples in estimated 5.6119 s (15k
StaticParse/mooreimp/1bigline
                        time:   [362.77 µs 365.69 µs 370.90 µs]
                        change: [-4.1295% -0.5821% +2.9777%] (p = 0.77 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
Benchmarking StaticParse/mealy/1bigline: Collecting 100 samples in estimated 6.3052 s (15k it
StaticParse/mealy/1bigline
                        time:   [403.08 µs 403.68 µs 404.46 µs]
                        change: [-17.581% -11.747% -6.4411%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
Benchmarking StaticParse/mealyimp/1bigline: Collecting 100 samples in estimated 5.7395 s (15k
StaticParse/mealyimp/1bigline
                        time:   [372.82 µs 373.31 µs 373.97 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
Benchmarking StaticParse/nom/1bigline: Collecting 100 samples in estimated 5.4850 s (15k iter
StaticParse/nom/1bigline
                        time:   [359.37 µs 360.03 µs 360.88 µs]
                        change: [-8.6985% -5.8508% -3.4074%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 21 outliers among 100 measurements (21.00%)
  8 (8.00%) high mild
  13 (13.00%) high severe
Benchmarking StaticParse/first/1bigline: Collecting 100 samples in estimated 6.9663 s (15k it
StaticParse/first/1bigline
                        time:   [447.85 µs 449.15 µs 451.32 µs]
                        change: [-28.093% -22.835% -16.981%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
Benchmarking StaticParse/split/1bigline: Collecting 100 samples in estimated 5.1566 s (1400 i
StaticParse/split/1bigline
                        time:   [3.6453 ms 3.6717 ms 3.7060 ms]
                        change: [-0.8370% -0.0519% +0.8608%] (p = 0.92 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
Benchmarking StaticParse/pest/1bigline: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.2s, enable flat sampling, or reduce sample count to 60.
Benchmarking StaticParse/pest/1bigline: Collecting 100 samples in estimated 6.1985 s (5050 it
StaticParse/pest/1bigline
                        time:   [1.2113 ms 1.2178 ms 1.2286 ms]
                        change: [-6.2930% -2.6596% +1.0779%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe
```