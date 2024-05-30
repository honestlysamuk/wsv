# Thursday 23rd May

### Do we really need black boxing in the benchmarks?

#### With the black box:

wsv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ io                                           │               │               │               │         │
   ╰─ benchmarks                                │               │               │               │         │
      ├─ reader_comments_100_000                │               │               │               │         │
      │  ├─ First                 18.09 ms      │ 28.99 ms      │ 21.34 ms      │ 22.11 ms      │ 10      │ 10
      │  ├─ Mealy                 176.2 ms      │ 194 ms        │ 180.5 ms      │ 182.2 ms      │ 10      │ 10
      │  ├─ Moore                 163.7 ms      │ 174.6 ms      │ 165.6 ms      │ 167 ms        │ 10      │ 10
      │  ├─ Nom                   45.64 ms      │ 55.94 ms      │ 45.7 ms       │ 46.95 ms      │ 10      │ 10
      │  ├─ Split                 35.82 ms      │ 55.38 ms      │ 35.97 ms      │ 38.11 ms      │ 10      │ 10
      │  ╰─ State                 57.22 ms      │ 64.1 ms       │ 57.36 ms      │ 58.14 ms      │ 10      │ 10
      ├─ reader_lines_100_000                   │               │               │               │         │
      │  ├─ First                 603.7 ms      │ 747.5 ms      │ 608.1 ms      │ 623 ms        │ 10      │ 10
      │  ├─ Mealy                 487.1 ms      │ 511.5 ms      │ 488.7 ms      │ 490.7 ms      │ 10      │ 10
      │  ├─ Moore                 497.2 ms      │ 523.1 ms      │ 498.8 ms      │ 501.3 ms      │ 10      │ 10
      │  ├─ Nom                   480.6 ms      │ 499.3 ms      │ 483.8 ms      │ 485.6 ms      │ 10      │ 10
      │  ├─ Split                 672.8 ms      │ 770.9 ms      │ 679.7 ms      │ 691.2 ms      │ 10      │ 10
      │  ╰─ State                 291.3 ms      │ 334.1 ms      │ 320.6 ms      │ 312.9 ms      │ 10      │ 10
      ╰─ reader_one_big_line                    │               │               │               │         │
         ├─ First                 436.7 µs      │ 5.96 ms       │ 441.3 µs      │ 468 µs        │ 1000    │ 1000
         ├─ Mealy                 335.3 µs      │ 802.2 µs      │ 337.9 µs      │ 349.5 µs      │ 1000    │ 1000
         ├─ Moore                 340.6 µs      │ 984.6 µs      │ 341.9 µs      │ 354.5 µs      │ 1000    │ 1000
         ├─ Nom                   336.2 µs      │ 729.4 µs      │ 337.9 µs      │ 347.3 µs      │ 1000    │ 1000
         ├─ Split                 4.621 ms      │ 9.149 ms      │ 4.642 ms      │ 4.718 ms      │ 1000    │ 1000
         ╰─ State                 204.8 µs      │ 584.1 µs      │ 206.8 µs      │ 223.3 µs      │ 1000    │ 1000


#### Without black boxes:

Timer precision: 32 ns
wsv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ io                                           │               │               │               │         │
   ╰─ benchmarks                                │               │               │               │         │
      ├─ reader_comments_100_000                │               │               │               │         │
      │  ├─ First                 18.54 ms      │ 40.48 ms      │ 21.04 ms      │ 24.71 ms      │ 10      │ 10
      │  ├─ Mealy                 176.6 ms      │ 193.3 ms      │ 180.1 ms      │ 182.8 ms      │ 10      │ 10
      │  ├─ Moore                 163.6 ms      │ 175.1 ms      │ 164.3 ms      │ 165.3 ms      │ 10      │ 10
      │  ├─ Nom                   45.92 ms      │ 60.05 ms      │ 46.02 ms      │ 47.71 ms      │ 10      │ 10
      │  ├─ Split                 38.1 ms       │ 57.25 ms      │ 38.23 ms      │ 40.46 ms      │ 10      │ 10
      │  ╰─ State                 57.23 ms      │ 74.87 ms      │ 57.49 ms      │ 59.32 ms      │ 10      │ 10
      ├─ reader_lines_100_000                   │               │               │               │         │
      │  ├─ First                 604.4 ms      │ 630.8 ms      │ 606.9 ms      │ 609.6 ms      │ 10      │ 10
      │  ├─ Mealy                 478.9 ms      │ 496.2 ms      │ 481.3 ms      │ 483.3 ms      │ 10      │ 10
      │  ├─ Moore                 492 ms        │ 629.4 ms      │ 495.3 ms      │ 508.5 ms      │ 10      │ 10
      │  ├─ Nom                   477 ms        │ 506.3 ms      │ 477.9 ms      │ 481.5 ms      │ 10      │ 10
      │  ├─ Split                 656.5 ms      │ 746.4 ms      │ 662.5 ms      │ 674.4 ms      │ 10      │ 10
      │  ╰─ State                 286.8 ms      │ 328.5 ms      │ 318.2 ms      │ 314 ms        │ 10      │ 10
      ╰─ reader_one_big_line                    │               │               │               │         │
         ├─ First                 438.1 µs      │ 5.778 ms      │ 442.1 µs      │ 491.1 µs      │ 1000    │ 1000
         ├─ Mealy                 331.4 µs      │ 4.923 ms      │ 332.8 µs      │ 481 µs        │ 1000    │ 1000
         ├─ Moore                 339.9 µs      │ 1.217 ms      │ 341.1 µs      │ 370.3 µs      │ 1000    │ 1000
         ├─ Nom                   335 µs        │ 981.6 µs      │ 336.9 µs      │ 361.8 µs      │ 1000    │ 1000
         ├─ Split                 4.598 ms      │ 8.601 ms      │ 4.62 ms       │ 4.673 ms      │ 1000    │ 1000
         ╰─ State                 205.5 µs      │ 822.9 µs      │ 206.8 µs      │ 226.7 µs      │ 1000    │ 1000

#### With black boxes around input and output

wsv                               fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ io                                           │               │               │               │         │
   ╰─ benchmarks                                │               │               │               │         │
      ├─ reader_comments_100_000                │               │               │               │         │
      │  ├─ First                 18.26 ms      │ 39.93 ms      │ 19.33 ms      │ 24.19 ms      │ 10      │ 10
      │  ├─ Mealy                 178 ms        │ 201.4 ms      │ 180.1 ms      │ 183 ms        │ 10      │ 10
      │  ├─ Moore                 163.6 ms      │ 174.1 ms      │ 163.8 ms      │ 165.1 ms      │ 10      │ 10
      │  ├─ Nom                   46.79 ms      │ 65.62 ms      │ 47.11 ms      │ 49.15 ms      │ 10      │ 10
      │  ├─ Split                 36.89 ms      │ 47.65 ms      │ 37.4 ms       │ 39.32 ms      │ 10      │ 10
      │  ╰─ State                 58.12 ms      │ 70.16 ms      │ 58.26 ms      │ 59.6 ms       │ 10      │ 10
      ├─ reader_lines_100_000                   │               │               │               │         │
      │  ├─ First                 611 ms        │ 650.2 ms      │ 620.2 ms      │ 623.9 ms      │ 10      │ 10
      │  ├─ Mealy                 486.5 ms      │ 516.9 ms      │ 491.9 ms      │ 494.7 ms      │ 10      │ 10
      │  ├─ Moore                 490 ms        │ 513.7 ms      │ 494.5 ms      │ 496.9 ms      │ 10      │ 10
      │  ├─ Nom                   491.3 ms      │ 507.3 ms      │ 492.3 ms      │ 494.5 ms      │ 10      │ 10
      │  ├─ Split                 669.8 ms      │ 762.6 ms      │ 685.6 ms      │ 697.3 ms      │ 10      │ 10
      │  ╰─ State                 290.8 ms      │ 328.3 ms      │ 322.6 ms      │ 313.9 ms      │ 10      │ 10
      ╰─ reader_one_big_line                    │               │               │               │         │
         ├─ First                 443.9 µs      │ 5.584 ms      │ 447.4 µs      │ 469 µs        │ 1000    │ 1000
         ├─ Mealy                 334.3 µs      │ 2.044 ms      │ 335.9 µs      │ 351.4 µs      │ 1000    │ 1000
         ├─ Moore                 341.9 µs      │ 1.26 ms       │ 343.7 µs      │ 370.5 µs      │ 1000    │ 1000
         ├─ Nom                   344.5 µs      │ 1.02 ms       │ 346.4 µs      │ 365.8 µs      │ 1000    │ 1000
         ├─ Split                 4.605 ms      │ 8.64 ms       │ 4.628 ms      │ 4.672 ms      │ 1000    │ 1000
         ╰─ State                 207.4 µs      │ 784.6 µs      │ 208.9 µs      │ 227.3 µs      │ 1000    │ 1000


## Conclusion

It appears that none of the tests are affected, meaning that the calculation is doing something sufficiently complex that the compiler doesn't optimise anything additionally anyway. I still feel a need to use the black box anyway, as a good practice, but I do not feel it appropriate here since the tests demonstrate that they are not useful.