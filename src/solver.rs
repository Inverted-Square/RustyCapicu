use crate::board::Turn::Player1;
use crate::board::{Board, BoardBuilder};
use std::cmp::{max, min};
use std::time::{Duration, Instant};

pub fn find_best_in(boards: &[Board], is_team1: bool) -> Option<(&Board, i16)> {
    if boards.is_empty() {
        return None;
    }

    let idx_evals = boards.iter().map(best_final_eval).enumerate();

    let (best_idx, best_eval) = if is_team1 {
        idx_evals.max_by_key(|&(_, eval)| eval)?
    } else {
        idx_evals.min_by_key(|&(_, eval)| eval)?
    };

    Some((&boards[best_idx], best_eval))
}

pub fn best_final_eval(board: &Board) -> i16 {
    minimax(board, i16::MIN, i16::MAX)
}

fn minimax(position: &Board, mut alpha: i16, mut beta: i16) -> i16 {
    if position.eval() != 0 {
        return position.eval();
    }

    if position.is_team1() {
        let mut max_eval = i16::MIN;
        for child in position.get_all_moves() {
            let eval = minimax(&child, alpha, beta);
            max_eval = max(max_eval, eval);
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }

        max_eval
    } else {
        let mut min_eval = i16::MAX;
        for child in position.get_all_moves() {
            let eval = minimax(&child, alpha, beta);
            min_eval = min(min_eval, eval);
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }
        }

        min_eval
    }
}

pub fn bench_minimax() {
    let num_bench_tests = 10;
    let times = (1..=num_bench_tests).map(single_bench);
    println!("Average - {:?}", times.sum::<Duration>() / num_bench_tests);
}

fn single_bench(bench_num: u32) -> Duration {
    let positions = BoardBuilder::new().build_all(Player1);
    let start = Instant::now();
    let (_, eval) = find_best_in(&positions, true).unwrap();
    let duration = start.elapsed();
    println!("{bench_num}. Eval: {eval} - Search time: {duration:?}");
    duration
}
