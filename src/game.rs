use crate::board::{Board, BoardBuilder, Turn};
use crate::solver::*;
use std::io::stdin;
use PlayerType::*;
use PointsGoal::*;

#[derive(Copy, Clone, Debug)]
pub enum PlayerType {
    Human,
    UnfairBot,
    // FairBot,
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
        println!("\nA new game of Capicú has begun!\n\nThe goal has been set to {goal} points.\n");

        Self {
            players: [Human; 4],
            goal,
            eval_on: false,
        }
    }

    pub fn set_player(mut self, turn: Turn, player_type: PlayerType) -> Self {
        self.players[turn as usize] = player_type;
        self
    }

    pub fn set_all_to(mut self, player_type: PlayerType) -> Self {
        self.players = [player_type; 4];
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
            println!("\nThe solver's evaluation will be given throughout the game.");
        }

        Game {
            players: self.players,
            goal: self.goal,
            eval_on: self.eval_on,
            board: BoardBuilder::new().build(),
            team_scores: [0; 2],
            round: 1,
        }
    }
}

pub struct Game {
    players: [PlayerType; 4],
    goal: u16,
    eval_on: bool,
    board: Board,
    team_scores: [u16; 2],
    round: u8,
}

impl Game {
    pub fn play(&mut self) {
        loop {
            self.play_round();
            let turn = self.handle_result();

            if self.team_scores[0] < self.goal && self.team_scores[1] < self.goal {
                break;
            }

            self.round += 1;
            let player_type = self.players[turn as usize];
            self.board = BoardBuilder::new()
                .starting_player(turn, player_type)
                .build();
        }

        // TODO: display message for winning team
    }

    // TODO: refactor this large method (note that in order to display the first move, there may be
    //       some overlapping logic with the while loop, so you should make helper methods for that
    //       to stop there being any code repetition).
    fn play_round(&mut self) {
        println!("\nPress [ENTER] to start round {}...", self.round);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        print!("{esc}c", esc = 27 as char);

        // TODO: display the board for the first move

        while self.board.eval() == 0 {
            println!("{}", self.board);
            let boards = self.board.get_all_moves();
            let player_type = self.players[self.board.turn()];
            let (new_board, eval) = match player_type {
                Human => todo!(
                    "Ask the player what domino they want to play (their options will be based on \
                    the last_placed field in each board in boards)."
                ),
                UnfairBot => find_best_in(&boards, self.board.is_team1()).unwrap(),
            };

            let prev_turn = self.board.turn();
            self.board = new_board.clone();
            let curr_turn = self.board.turn();

            let [num1, num2] = new_board.last_played_domino();
            println!("|| {num1} | {num2} ||   domino was placed.");

            let skipped = Board::skipped_players(prev_turn, curr_turn);
            for player in skipped {
                println!("Player {} has to pass.", player + 1);
            }

            if self.eval_on {
                let eval = if let Human = player_type {
                    best_final_eval(new_board)
                } else {
                    eval
                };
                println!("\nCurrent eval: {eval}.");
            }

            println!("\nPress [ENTER] to continue...");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            print!("{esc}c", esc = 27 as char);
        }
    }

    fn handle_result(&mut self) -> Turn {
        let eval = self.board.eval();
        let winning_team = (eval < 0) as usize;
        self.team_scores[winning_team] += eval.unsigned_abs();

        let last_player = self.board.turn();
        let is_team1 = self.board.is_team1();
        let positive_eval = eval > 0;
        let winning_player = if is_team1 && positive_eval || !is_team1 && !positive_eval {
            last_player
        } else {
            (last_player + 1) % 4
        };

        if self.board.hand_empty() {
            println!(
                "Player {} placed their last domino so the round is over!\nThe team scores \
                are:\nTeam 1: {}\nTeam 2: {}",
                last_player + 1,
                self.team_scores[0],
                self.team_scores[1]
            );
        } else {
            println!(
                "Player {} blocked the game so the round is over!\nOut of player {} and {} \
                player {} has the lower pip count, so team {} wins the round!\nThe team scores \
                are:\nTeam 1: {}\nTeam 2: {}",
                last_player + 1,
                last_player + 1,
                (last_player + 1) % 4 + 1,
                winning_player + 1,
                winning_team + 1,
                self.team_scores[0],
                self.team_scores[1]
            );
        }

        Turn::safe_from(winning_player).unwrap()
    }
}
