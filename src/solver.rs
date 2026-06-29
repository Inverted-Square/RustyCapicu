use crate::board::Turn::Player1;
use crate::board::{Board, BoardBuilder};
use std::cmp::{max, min};
use std::time::{Duration, Instant};

pub fn select_best(boards: &[Board], is_team1: bool) -> Option<(&Board, i16)> {
    if boards.is_empty() {
        return None;
    }

    let mut alpha = i16::MIN;
    let mut beta = i16::MAX;
    let mut best_eval = if is_team1 { i16::MIN } else { i16::MAX };
    let mut best_idx = 0;
    for (idx, board) in boards.iter().enumerate() {
        let eval = minimax(board, alpha, beta);

        if is_team1 {
            if eval > best_eval {
                best_idx = idx;
            }
            best_eval = max(best_eval, eval);
            alpha = max(alpha, eval);
        } else {
            if eval < best_eval {
                best_idx = idx;
            }
            best_eval = min(best_eval, eval);
            beta = min(beta, eval);
        }

        if beta <= alpha {
            break;
        }
    }

    Some((&boards[best_idx], best_eval))
}

pub fn best_final_eval(board: &Board) -> i16 {
    minimax(board, i16::MIN, i16::MAX)
}

fn minimax(position: &Board, mut alpha: i16, mut beta: i16) -> i16 {
    if position.eval() != 0 {
        return position.eval();
    }

    let is_team1 = position.is_team1();
    let mut best_eval = if is_team1 { i16::MIN } else { i16::MAX };
    for child in position.get_all_moves() {
        let eval = minimax(&child, alpha, beta);

        if is_team1 {
            best_eval = max(best_eval, eval);
            alpha = max(alpha, eval);
        } else {
            best_eval = min(best_eval, eval);
            beta = min(beta, eval);
        }

        if beta <= alpha {
            break;
        }
    }

    best_eval
}

pub fn bench_minimax() {
    let num_bench_tests = 1000;
    let times = (1..=num_bench_tests).map(single_bench);
    println!("Average - {:?}", times.sum::<Duration>() / num_bench_tests);
}

fn single_bench(bench_num: u32) -> Duration {
    let positions = BoardBuilder::new().build_all(Player1);
    let start = Instant::now();
    let (_, eval) = select_best(&positions, true).unwrap();
    let duration = start.elapsed();
    println!("{bench_num}. Eval: {eval} - Search time: {duration:?}");
    duration
}
