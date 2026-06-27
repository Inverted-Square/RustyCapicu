use crate::game::PlayerType::UnfairBot;
use crate::game::{GameSetup, PointsGoal};

mod board;
mod game;
mod solver;

fn main() {
    GameSetup::new(PointsGoal::TwoHundred)
        .set_all_to(UnfairBot)
        .eval_on()
        .set_up()
        .play();
}
