use crate::game::PlayerType::UnfairBot;
use crate::game::{GameSetup, PointsGoal};
use crate::solver::bench_minimax;
use std::env::args;

mod board;
mod game;
mod solver;

fn main() -> Result<(), &'static str> {
    let last_arg = args()
        .nth(1)
        .ok_or("Please give an input argument option: [minimax_bench, normal]")?;
    if last_arg == "minimax_bench" {
        bench_minimax();
    } else {
        GameSetup::new(PointsGoal::TwoHundred)
            .set_all_to(UnfairBot)
            .eval_on()
            .set_up()
            .play();
    }

    Ok(())
}
