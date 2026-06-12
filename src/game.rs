use crate::board::{Board, BoardBuilder, Turn};
use PlayerType::*;
use PointsGoal::*;

#[derive(Copy, Clone, Debug)]
pub enum PlayerType {
    Human,
    UnfairBot,
    // FairBot, // TODO: eventually
}

pub struct GameSetup {
    players: [PlayerType; 4],
    goal: u16,
    eval_on: bool,
}

pub enum PointsGoal {
    OneHundred,
    TwoHundred,
}

impl GameSetup {
    pub fn new(goal: PointsGoal) -> Self {
        let goal = match goal {
            OneHundred => 100,
            TwoHundred => 200,
        };

        print!("{esc}c", esc = 27 as char);
        println!("\nA new game of Capicú has begun!\n\nThe goal has been set to {goal} points.");

        Self {
            players: [Human; 4],
            goal,
            eval_on: false,
        }
    }

    pub fn set_type(mut self, turn: Turn, player_type: PlayerType) -> Self {
        self.players[turn as usize] = player_type;
        self
    }

    pub fn eval_on(mut self) -> Self {
        self.eval_on = true;
        self
    }

    pub fn set_up(self) -> Game {
        for (idx, &player_type) in self.players.iter().enumerate() {
            println!("Player {} has been set as {player_type:?}", idx + 1);
        }

        if self.eval_on {
            println!("The solver's evaluation will be given throughout the game.");
        }

        Game {
            players: self.players,
            goal: self.goal,
            eval_on: self.eval_on,
            board: BoardBuilder::new().build(),
            team_scores: [0; 2],
        }
    }
}

pub struct Game {
    players: [PlayerType; 4],
    goal: u16,
    eval_on: bool,
    board: Board,
    team_scores: [u16; 2],
}

impl Game {
    pub fn play(&mut self) {
        while self.team_scores[0] < self.goal && self.team_scores[1] < self.goal {
            self.play_round();
        }
    }

    fn play_round(&mut self) {
        todo!()
    }
}
