# Wednesday 15th May 2024

## First output from Divan



```rs
Timer precision: 38 ns
wsv                       fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ io                                   │               │               │               │         │
│  ├─ buf_file                          │               │               │               │         │
│  │  ├─ 100lines.wsv     289.2 µs      │ 1.495 ms      │ 374.2 µs      │ 455.2 µs      │ 1000    │ 1000
│  │  ╰─ 1bigline.wsv     212.6 µs      │ 834.9 µs      │ 227.2 µs      │ 262.9 µs      │ 1000    │ 1000
│  ╰─ whole_file                        │               │               │               │         │
│     ├─ 100lines.wsv     265.3 µs      │ 1.359 ms      │ 300.9 µs      │ 344.8 µs      │ 1000    │ 1000
│     ╰─ 1bigline.wsv     206.7 µs      │ 717.3 µs      │ 234.6 µs      │ 286.9 µs      │ 1000    │ 1000
╰─ parsers                              │               │               │               │         │
   ├─ first                             │               │               │               │         │
   │  ╰─ micro                          │               │               │               │         │
   │     ├─ 100lines.wsv  1.274 µs      │ 6.726 µs      │ 1.775 µs      │ 1.798 µs      │ 1000    │ 1000
   │     ╰─ 1bigline.wsv  760.9 ns      │ 15.23 µs      │ 1.317 µs      │ 1.267 µs      │ 1000    │ 4000
   ├─ mealy                             │               │               │               │         │
   │  ╰─ micro                          │               │               │               │         │
   │     ├─ 100lines.wsv  1.821 µs      │ 15.66 µs      │ 2.679 µs      │ 2.768 µs      │ 1000    │ 1000
   │     ╰─ 1bigline.wsv  1.915 µs      │ 12.14 µs      │ 2.727 µs      │ 2.777 µs      │ 1000    │ 1000
   ├─ moore                             │               │               │               │         │
   │  ╰─ micro                          │               │               │               │         │
   │     ├─ 100lines.wsv  3.453 µs      │ 44.72 µs      │ 4.846 µs      │ 4.998 µs      │ 1000    │ 1000
   │     ╰─ 1bigline.wsv  3.81 µs       │ 23.25 µs      │ 5.13 µs       │ 5.14 µs       │ 1000    │ 1000
   ├─ nom                               │               │               │               │         │
   │  ╰─ micro                          │               │               │               │         │
   │     ├─ 100lines.wsv  735.2 ns      │ 29.17 µs      │ 1.042 µs      │ 1.127 µs      │ 1000    │ 1000
   │     ╰─ 1bigline.wsv  717.7 ns      │ 12.05 µs      │ 983.7 ns      │ 1.027 µs      │ 1000    │ 4000
   ├─ pest                              │               │               │               │         │
   │  ╰─ micro                          │               │               │               │         │
   │     ├─ 100lines.wsv  6.286 µs      │ 75.55 µs      │ 8.038 µs      │ 8.382 µs      │ 1000    │ 1000
   │     ╰─ 1bigline.wsv  6.337 µs      │ 1.795 ms      │ 7.98 µs       │ 10.97 µs      │ 1000    │ 1000
   ├─ split                             │               │               │               │         │
   │  ╰─ micro                          │               │               │               │         │
   │     ├─ 100lines.wsv  1.515 µs      │ 40.73 µs      │ 2.475 µs      │ 2.467 µs      │ 1000    │ 1000
   │     ╰─ 1bigline.wsv  1.59 µs       │ 15.56 µs      │ 2.512 µs      │ 2.479 µs      │ 1000    │ 1000
   ╰─ state                             │               │               │               │         │
      ╰─ micro                          │               │               │               │         │
         ├─ 100lines.wsv  1.374 µs      │ 11.74 µs      │ 2.019 µs      │ 2.04 µs       │ 1000    │ 1000
         ╰─ 1bigline.wsv  1.314 µs      │ 12.3 µs       │ 1.877 µs      │ 1.981 µs      │ 1000    │ 1000
```
