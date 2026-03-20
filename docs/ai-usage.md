# AI Usage Log

## 2026-03-18

Date: 2026-03-18
Commit: 56a2bedd5fafbf5567e34277468980f1cea0e458

Asked Claude to brainstorm some core modeling optimizations.
The only one I found worthwhile was using a bitset to represent the tile possibilities, which is a great optimization.
It lets us store all the possibilities in a single 16 bit int and use bitwise operations which are extremely quick at runtime.
The alternative would be to use a `Set<Digit>` which is WAY less efficient:w


## 2026-03-19

Date: 2026-03-19
Commit: f5ae33be4dc867ce80a7ef7fba84613db78d394b

Had ChatGPT create the big `let box_id = match (position.row(), position.col()) {` match statement, too tedious


## 2026-03-19

Date: 2026-03-19
Commit: 748b4d6dba8ae6135697e1fbf98bd35e9999767a

Had Claude implement the Board display functions.
Resulting code was not very readable so I refactored it to my liking.

Prompt:

"""
@src/board/board.rs:36-40 

Implement these two methods. `display_candidates` should show the
```
123
456
789
```

candidates per cell, or blank in that spot if it's not a candidate.

Versus `display_solution` should show the 9x9 grid solution
```
123 456 789
456 789 123
789 123 456
...
```

and blank in that spot if there's not a single selected candidate yet
"""

## 2026-03-20

Date: 2026-03-20
Commit: 4b8ea2cfa4f7968f98de4583a0bbb19bbcd8cb83

Had Claude set up the 17_clue, norvig_hardest, and norvig_top95 Sudoku board datasets and wire them into the benchmark
