use crate::board::BoardBuilder;
use crate::board::Turn::*;
use crate::game::PlayerType::UnfairBot;
use crate::solver::{best_final_eval, find_best_move};

mod board;
mod game;
mod solver;
mod train_displayer;

fn main() {
    random_perfect_first_round();
    println!("-------------------------------------------");
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
    let mut board = BoardBuilder::new()
        .starting_player(Player1, UnfairBot)
        .build();
    println!("{board}");
    while let Some((best_board, eval)) = find_best_move(&board) {
        println!("{best_board}\nMinimax eval: {eval}");
        board = best_board;
    }
}
