## ssolvent

ssolvent (sudoku solve~~r~~nt) is a MRV Sudoku solver written in Rust.

Implemented for W26 CSI 3610 with Professor Tianle Ma


## Figures

ssolvent can solve Sudoku puzzle difficulties in the following times:

- Easy: 2.4µs (~420,000 puzzles/s)
- Medium: 6.4µs (~160,000 puzzles/s)
- Hard: 9.3µs (~110,000 puzzles/s)
- Expert: 12.4µs (~80,000 puzzles/s)
- Norvig Hardest: 27µs (~40,000 puzzles/s)
- Norvig Top 95: 800µs (~1,250 puzzles/s)
- 17 Clue: 3.1ms (~323 puzzles/s)


## Benchmarks

Specs:
- Ryzen 5 5600 @ ~4.5GHz
- 32GB DDR4 RAM @ 3600MHz


`cargo bench`
```
solve/easy              time:   [2.3768 µs 2.3847 µs 2.3926 µs]
                        change: [−1.4968% −0.5487% +0.4668%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

solve/medium            time:   [6.3196 µs 6.3516 µs 6.3840 µs]
                        change: [−3.0698% −2.0511% −0.9508%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild

solve/hard              time:   [9.3126 µs 9.3592 µs 9.4059 µs]
                        change: [−0.9128% −0.0961% +0.7136%] (p = 0.83 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild

solve/diabolical        time:   [12.303 µs 12.361 µs 12.419 µs]
                        change: [−0.5538% +0.3317% +1.2785%] (p = 0.48 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) low mild
  5 (5.00%) high mild

solve/17_clue           time:   [2.6205 ms 3.1059 ms 3.6381 ms]
                        change: [−21.039% −1.0401% +22.772%] (p = 0.93 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

solve/norvig_top95      time:   [774.72 µs 796.90 µs 818.72 µs]
                        change: [−8.8171% +1.4644% +12.265%] (p = 0.78 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

solve/norvig_hardest    time:   [26.427 µs 26.626 µs 26.892 µs]
                        change: [−3.0872% −2.3324% −1.6118%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
```