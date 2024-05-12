# Sunday 12th May

Results from the comparison.

The separation of concerns overhead is around 1 microsecond, or 40% slower. The additional indirection costs, and this is how much. Still, both mealy and moore are faster than every other implementation so far, and they are far more universally applicable. Still need to enter serde into the mix.

```rs
Benchmarking StaticParse/first/empty_string: Collecting 100 samples in estimated 5.
StaticParse/first/empty_string
                        time:   [4.7021 µs 4.7218 µs 4.7440 µs]

Benchmarking StaticParse/state/empty_string: Collecting 100 samples in estimated 5.
StaticParse/state/empty_string
                        time:   [2.5511 µs 2.5666 µs 2.5912 µs]

Benchmarking StaticParse/moore/empty_string: Collecting 100 samples in estimated 5.
StaticParse/moore/empty_string
                        time:   [3.5378 µs 3.6019 µs 3.6991 µs]

Benchmarking StaticParse/mealy/empty_string: Collecting 100 samples in estimated 5.
StaticParse/mealy/empty_string
                        time:   [3.4970 µs 3.5075 µs 3.5184 µs]
  ```