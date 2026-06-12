use crate::board::BoardBuilder;
use crate::board::DominoIdx::*;
use crate::board::FirstDominoChoice::*;
use crate::board::Turn::*;
use crate::solver::*;

mod board;
mod solver;

fn main() {
    random_perfect_first_round();
    random_perfect_round();
}

fn random_perfect_first_round() {
    let mut board = BoardBuilder::new().build();
    println!("{board}\nMinimax eval: {}", best_final_eval(&board));
    while let Some((best_board, eval)) = find_best_move(&board) {
        println!("{best_board}\nMinimax eval: {eval}");
        board = best_board;
    }
}

fn random_perfect_round() {
    let boards = BoardBuilder::new().build_all(Player1);
    let (board, eval) = find_best_in(&boards, true).unwrap();
    println!("{board}\nMinimax eval: {eval}");

    let mut board = board.clone();
    while let Some((best_board, eval)) = find_best_move(&board) {
        println!("{best_board}\nMinimax eval: {eval}");
        board = best_board;
    }
}
