use std::str::FromStr;

use crate::board::board::Board;
use crate::solver::solve;

pub mod board;
pub mod solver;

fn main() {
    let puzzles = vec![
        "080200400570000100002300000820090005000715000700020041000006700003000018007009050",
        "210950004090060037000700000000000308920000015805000000000002000680010040100047096",
        "600040001030008700009700000003096000906000103000120500000002400002400080400010002",
        "090000006006890000203706090100020000054908320000070004020607809000035600300000050",
        "004009300010030050800700009006800020005000700020007900200004007040050060001200500",
        "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
    ];

    for puzzle in puzzles.into_iter() {
        let board = Board::from_str(puzzle).unwrap();

        println!("Original board:\n{}\n", board.display_solution());

        match solve(&board) {
            Some(solution) => {
                println!("Solved board:\n{}", solution.display_solution());
            }
            None => {
                println!("No solution found.");
            }
        }

        println!("\n");
    }
}
