```rs
    (0..)
        .scan(State::Default, |state, _| {
            if end_states.contains(&state) {
                None
            } else {
                *state = transition(*state, inputs.next());
                Some(g(*state))
            }
        })
```

The previous code is significantly slower than

```rs
    let mut state = State::Default;
    (0..)
        .map_while(|_| {
            if end_states.contains(&state) {
                None
            } else {
                state = transition(state, inputs.next());
                Some(g(state))
            }
        })
```

by 65%:

```rs
StaticParse/moore/empty_string
                        time:   [5.6451 µs 5.6595 µs 5.6749 µs]
                        change: [+62.511% +65.728% +69.053%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```