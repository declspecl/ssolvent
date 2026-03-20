use crate::board::{board::Board, digit::Digit, position::Position};

pub mod board;

fn main() {
    let mut board = Board::new();
    board.solve_cell(Position::new(Digit::ONE, Digit::ONE), Digit::ONE);
    board.solve_cell(Position::new(Digit::ONE, Digit::TWO), Digit::TWO);
    board.solve_cell(Position::new(Digit::ONE, Digit::THREE), Digit::THREE);
    board.solve_cell(Position::new(Digit::ONE, Digit::FOUR), Digit::FOUR);
    board.solve_cell(Position::new(Digit::ONE, Digit::FIVE), Digit::FIVE);
    board.solve_cell(Position::new(Digit::ONE, Digit::SIX), Digit::SIX);
    board.solve_cell(Position::new(Digit::ONE, Digit::SEVEN), Digit::SEVEN);
    board.solve_cell(Position::new(Digit::ONE, Digit::EIGHT), Digit::EIGHT);
    board.solve_cell(Position::new(Digit::ONE, Digit::NINE), Digit::NINE);

    println!("Board candidates:\n{}", board.display_candidates());

    println!("Board solution:\n{}", board.display_solution());
}
