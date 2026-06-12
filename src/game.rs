use crate::board::{Board, BoardBuilder, Turn};
use crate::game::PlayerType::Human;
use crate::game::PointsGoal::*;

pub struct Game {
    players: [PlayerType; 4],
    goal: u8,
    eval_on: bool,
    board: Board,
}

#[derive(Copy, Clone)]
pub enum PlayerType {
    Human,
    UnfairBot,
    // FairBot, // TODO: eventually
}

pub enum PointsGoal {
    OneHundred,
    TwoHundred,
}

impl Game {
    pub fn new(goal: PointsGoal, eval_on: bool) -> Self {
        let goal = match goal {
            OneHundred => 100,
            TwoHundred => 200,
        };

        Self {
            players: [Human; 4],
            goal,
            eval_on,
            board: BoardBuilder::new().build(),
        }
    }

    pub fn set_player_type(&mut self, turn: Turn, player_type: PlayerType) {
        self.players[turn as usize] = player_type;
    }

    pub fn play_turn(&self) {}
}
