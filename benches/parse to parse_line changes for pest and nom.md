# Attempt

I wanted to create a single interface with each parser in the form of parse_line, instead of parse, so I could then look into parallelism as a speed boost. This is the initial difference after removing the whole-file parser from each of the tools.




### StaticParse/nom/nulls   time:   [440.54 ns 441.21 ns 441.92 ns]
                        change: [+25.426% +26.149% +26.811%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
### StaticParse/pest/nulls  time:   [3.0888 µs 3.0925 µs 3.0966 µs]
                        change: [+38.839% +39.237% +39.690%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  7 (7.00%) high mild
  2 (2.00%) high severe