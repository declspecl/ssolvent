use std::str::FromStr;

use ssolvent::board::board::Board;
use ssolvent::solver::solve;

fn solution_digits(board: &Board) -> String {
    let mut out = String::with_capacity(81);
    for position_id in 0..81u8 {
        let cell = board.at(ssolvent::board::position::Position::from_id(position_id));
        out.push(char::from(b'0' + cell.solved_digit().unwrap()));
    }

    return out;
}

/// verifies the solved board satisfies Sudoku constraints:
/// every row, column, and 3x3 box contains digits 1..=9 exactly once.
fn assert_valid_sudoku(digits: &str) {
    assert_eq!(digits.len(), 81);
    let grid: Vec<u8> = digits.bytes().map(|b| b - b'0').collect();

    for row in 0..9 {
        let mut seen = [false; 10];
        for col in 0..9 {
            let d = grid[row * 9 + col];
            assert!(d >= 1 && d <= 9, "invalid digit {} at row {}", d, row);
            assert!(!seen[d as usize], "row {} has duplicate {}", row, d);
            seen[d as usize] = true;
        }
    }

    for col in 0..9 {
        let mut seen = [false; 10];
        for row in 0..9 {
            let d = grid[row * 9 + col];
            assert!(!seen[d as usize], "col {} has duplicate {}", col, d);
            seen[d as usize] = true;
        }
    }

    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = [false; 10];
            for r in 0..3 {
                for c in 0..3 {
                    let d = grid[(box_row * 3 + r) * 9 + (box_col * 3 + c)];
                    assert!(!seen[d as usize], "box ({},{}) has duplicate {}", box_row, box_col, d);
                    seen[d as usize] = true;
                }
            }
        }
    }
}

/// asserts the solution agrees with the original puzzle's given clues.
fn assert_matches_clues(puzzle: &str, solution: &str) {
    for (p, s) in puzzle.bytes().zip(solution.bytes()) {
        if p >= b'1' && p <= b'9' {
            assert_eq!(p, s, "solution conflicts with given clue");
        }
    }
}

#[test]
fn solves_easy_puzzle() {
    // wikipedia example
    let puzzle = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let expected = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

    let board = Board::from_str(puzzle).unwrap();
    let solution = solve(&board).expect("puzzle should have a solution");
    let digits = solution_digits(&solution);

    assert_eq!(digits, expected);
    assert_valid_sudoku(&digits);
    assert_matches_clues(puzzle, &digits);
}

#[test]
fn solves_hard_puzzle() {
    // Arto Inkala's "world's hardest" puzzle
    let puzzle = "800000000003600000070090200050007000000045700000100030001000068008500010090000400";

    let board = Board::from_str(puzzle).unwrap();
    let solution = solve(&board).expect("hard puzzle should have a solution");
    let digits = solution_digits(&solution);

    assert_valid_sudoku(&digits);
    assert_matches_clues(puzzle, &digits);
}

#[test]
fn solves_expert_puzzle() {
    let puzzle = "004009300010030050800700009006800020005000700020007900200004007040050060001200500";

    let board = Board::from_str(puzzle).unwrap();
    let solution = solve(&board).expect("expert puzzle should have a solution");
    let digits = solution_digits(&solution);

    assert_valid_sudoku(&digits);
    assert_matches_clues(puzzle, &digits);
}

/// edge case: an already-solved puzzle should be returned unchanged.
#[test]
fn already_solved_puzzle_is_idempotent() {
    let solved = "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

    let board = Board::from_str(solved).unwrap();
    assert!(board.is_solved());

    let solution = solve(&board).expect("solved board should remain solved");
    assert_eq!(solution_digits(&solution), solved);
}

/// edge case: an empty board has many valid fillings; the solver should produce one.
#[test]
fn empty_board_produces_valid_filling() {
    let empty = "0".repeat(81);

    let board = Board::from_str(&empty).unwrap();
    let solution = solve(&board).expect("empty board admits a valid filling");

    assert_valid_sudoku(&solution_digits(&solution));
}

/// edge case: a contradictory puzzle (two 5s in the same row) has no solution.
#[test]
fn unsolvable_puzzle_returns_none() {
    // row 0 contains two 5s in columns 0 and 1, a direct contradiction
    let bad = "550000000000000000000000000000000000000000000000000000000000000000000000000000000";

    // Board::from_str will stop propagating once a contradiction is detected,
    // but the resulting board must still be rejected by the solver.
    match Board::from_str(bad) {
        Ok(board) => {
            assert!(solve(&board).is_none(), "contradictory puzzle must be unsolvable");
        }
        Err(_) => {}
    }
}
