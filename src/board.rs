use crate::board::BoardInitialSettings::*;
use crate::board::DominoIdx::*;
use crate::board::FirstDominoChoice::*;
use crate::board::Turn::*;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct BoardBuilder {
    dominoes: [[u8; 2]; 28],
    settings: BoardInitialSettings,
}

#[derive(Clone)]
enum BoardInitialSettings {
    FirstRound,
    StartingPlayer(Turn, FirstDominoChoice),
}

#[derive(Clone)]
pub enum FirstDominoChoice {
    HumanInput,
    GivenIdx(DominoIdx),
}

#[derive(Clone, Copy)]
pub enum DominoIdx {
    First,
    Second,
    Third,
    Forth,
    Fifth,
    Sixth,
    Seventh,
}

impl BoardBuilder {
    pub fn new() -> Self {
        use rand::{rng, seq::SliceRandom};
        let mut dominoes: [[u8; 2]; 28] = [
            [0, 0],
            [0, 1],
            [0, 2],
            [0, 3],
            [0, 4],
            [0, 5],
            [0, 6],
            [1, 1],
            [1, 2],
            [1, 3],
            [1, 4],
            [1, 5],
            [1, 6],
            [2, 2],
            [2, 3],
            [2, 4],
            [2, 5],
            [2, 6],
            [3, 3],
            [3, 4],
            [3, 5],
            [3, 6],
            [4, 4],
            [4, 5],
            [4, 6],
            [5, 5],
            [5, 6],
            [6, 6],
        ];
        dominoes.shuffle(&mut rng());

        Self {
            dominoes,
            settings: FirstRound,
        }
    }

    pub fn starting_player(mut self, turn: Turn, choice: FirstDominoChoice) -> Self {
        self.settings = StartingPlayer(turn, choice);
        self
    }

    pub fn build_all(self, turn: Turn) -> Vec<Board> {
        vec![First, Second, Third, Forth, Fifth, Sixth, Seventh]
            .into_iter()
            .map(|domino_idx| {
                self.clone()
                    .starting_player(turn.clone(), GivenIdx(domino_idx))
                    .build()
            })
            .collect()
    }

    pub fn build(self) -> Board {
        let mut board = Board::new();

        for (idx, &[num1, num2]) in self.dominoes.iter().enumerate() {
            if Self::place_first(&self.settings, idx, num1, num2) {
                board.train_ends = [num1, num2];
                board.turn = Self::idx_as_turn((idx / 7 + 1) % 4);
                continue;
            }

            board.hands[idx / 7][num1 as usize].push(num2);
            if num1 != num2 {
                board.hands[idx / 7][num2 as usize].push(num1);
            }

            board.pip_totals[idx / 7] += num1 + num2;
        }

        if let StartingPlayer(turn, HumanInput) = self.settings {
            todo!("Get the human's first move, play that move, and make sure to progress the turn.")
        }

        board
    }

    fn place_first(settings: &BoardInitialSettings, idx: usize, num1: u8, num2: u8) -> bool {
        match settings {
            FirstRound => num1 == 6 && num2 == 6,
            StartingPlayer(turn, choice) if turn.clone() as usize == idx / 7 => match choice {
                HumanInput => false,
                GivenIdx(domino_idx) => *domino_idx as usize == idx % 7,
            },
            _ => false,
        }
    }

    fn idx_as_turn(idx: usize) -> Turn {
        match idx {
            0 => Player1,
            1 => Player2,
            2 => Player3,
            3 => Player4,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    hands: Vec<Vec<Vec<u8>>>,
    pip_totals: [u8; 4],
    train_ends: [u8; 2],
    turn: Turn,
    eval: i16,
}

impl Board {
    fn new() -> Self {
        Self {
            hands: vec![(0..7).map(|_| Vec::with_capacity(7)).collect(); 4],
            pip_totals: [0; 4],
            train_ends: [u8::MAX; 2],
            turn: Player1,
            eval: 0,
        }
    }

    fn hands_to_string(&self) -> String {
        let mut board_string = String::from("");

        for (player, hand) in self.hands.iter().enumerate() {
            board_string.push_str(&format!("\nPlayer {}'s hand:   ", player + 1));
            for (num1, row) in hand.iter().enumerate() {
                board_string.push_str(&Self::dominoes_to_string(num1 as u8, row));
            }
        }

        board_string
    }

    fn dominoes_to_string(num1: u8, row: &[u8]) -> String {
        let mut dominoes_string = String::from("");

        for &num2 in row {
            if num1 <= num2 {
                dominoes_string.push_str(&format!("|| {num1} | {num2} ||   "));
            }
        }

        dominoes_string
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nPip totals: {:?}\nTrain ends: {:?}\nTurn: {:?}\nEval: {}\n",
            self.hands_to_string(),
            self.pip_totals,
            self.train_ends,
            self.turn,
            self.eval,
        )
    }
}

#[derive(Clone, Debug)]
pub enum Turn {
    Player1,
    Player2,
    Player3,
    Player4,
}

impl Turn {
    pub fn next_player(&mut self) {
        *self = match self {
            Player1 => Player2,
            Player2 => Player3,
            Player3 => Player4,
            Player4 => Player1,
        };
    }
}
