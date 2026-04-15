## ssolvent

ssolvent (s~~udoku~~ solve~~r~~nt) is a MRV Sudoku solver written in Rust.

Implemented for W26 CSI 3610 with Professor Tianle Ma by Gavin D'Hondt (gavindhondt@oakland.edu).

YouTube video explanation: https://youtu.be/mi_EJzySoFU


## Problem & Approach

Given a partially filled 9x9 Sudoku grid, fill in the rest so every row, column, and 3x3 box contains 1–9 exactly once.
ssolvent treats this as a CSP and solves it with backtracking search plus two standard CSP techniques:

- **Constraint propagation:** assigning a digit to a cell removes it from the
  candidate sets of its 20 peers; peers that collapse to one candidate are
  solved recursively. (ref: `Board::solve_cell`)
- **MRV branching:** at each step the solver branches on the unsolved cell
  with the fewest remaining candidates. (ref: `solve`)

Candidate sets are a 9-bit bitmask packed into a `u16`, so membership and
pruning are single CPU instructions.


## Running

Requires a Rust toolchain.

```
cargo run --release     # solves the demo puzzles in src/main.rs
cargo test              # runs the integration tests
cargo bench             # runs the criterion benchmarks
```

Puzzles are 81-character strings: `1`–`9` are clues, any of `.`, `?`, `0`, or
space is an empty cell. To solve a custom puzzle, edit the `puzzles` vector in
`src/main.rs`.


## Tests

Integration tests live in `tests/solver.rs` (6 tests, all passing).
They solve various difficulty puzzles and expected behavior against edge cases.

Each test validates the output against the Sudoku row/column/box constraints
and checks that every given clue is preserved.


## Analysis

```
solve(board):
    # base case
    if board is fully solved:
        return board

    pick the unsolved cell with the fewest candidates (MRV)

    # invalid, backtrack
    if that cell has zero candidates:
        return None

    for each digit d in that cell's candidate set:
        attempt = copy of board

        if attempt.solve_cell(cell, d):
            result = solve(attempt)

            # propagate recursive solution
            if result is not None:
                return result

    return None
```

The worst-case time complexity is `O(9^m)` where `m` is the number of empty cells.
In the worst case, each empty cell tries up to 9 digits before backtracking.
(Generalized Sudoku on n^2 × n^2 grids is NP-complete.)

In practice the solver is orders of magnitude faster than 9^m suggests, for two reasons:

1. Constraint propagation prunes the tree aggressively.
Every assignment shrinks 20 peers at once, and forced singletons cascade into whole chains of deterministic moves between choice points.
2. MRV keeps the effective branching factor close to 2–3 instead of 9, so contradictions surface after only a couple of bad guesses.

Space complexity is `O(m)` for the recursion stack since each recursive call clones one 81-cell board, which is O(1) since cells are 16-bit bitmasks.


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


## AI Usage

The core algorithm including MRV selection, backtracking, peer-propagation, and the bitmask candidate set was designed and implemented by me.
I coincidentally am also implementing a Poker solver which is where I got the inspiration for much of this.
AI tools were used for: brainstorming some optimization ideas, generating a tedious `(row, col) → box_id` match, drafting the two `display_*` helpers (then refactored), wiring in the `17_clue` / `norvig_hardest` / `norvig_top95` benchmark datasets, and scaffolding the integration tests and this README.
Full log with commits and prompts is in [./docs/ai-usage.md](./docs/ai-usage.md)
