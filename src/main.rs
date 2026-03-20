use std::str::FromStr;

use crate::board::board::Board;
use crate::solver::solve;

pub mod board;
pub mod solver;

fn main() {
    let puzzle_string = "050703060007000800000816000000030000005000100730040086906000204840572093000409000";
    let board = Board::from_str(puzzle_string).unwrap();

    println!("Original board:\n{}", board.display_solution());

    match solve(&board) {
        Some(solution) => {
            println!("Solved board:\n{}", solution.display_solution());
        }
        None => {
            println!("No solution found.");
        }
    }
}
