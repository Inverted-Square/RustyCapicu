use crate::board::BoardInitialSettings::*;
use crate::board::FirstDominoChoice::*;
use crate::game::PlayerType;
use crate::game::PlayerType::*;
use crate::solver::find_best_in;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct BoardBuilder {
    dominoes: [[u8; 2]; 28],
    settings: BoardInitialSettings,
}

#[derive(Clone)]
enum BoardInitialSettings {
    FirstRound,
    StartingPlayer(usize, FirstDominoChoice),
}

#[derive(Clone, Copy)]
enum FirstDominoChoice {
    HumanInput,
    GivenIdx(Option<usize>),
}

pub enum Turn {
    Player1,
    Player2,
    Player3,
    Player4,
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

    pub fn starting_player(mut self, turn: Turn, player_type: PlayerType) -> Self {
        match player_type {
            Human => self.settings = StartingPlayer(turn as usize, HumanInput),
            UnfairBot => self.settings = StartingPlayer(turn as usize, GivenIdx(None)),
        }

        self
    }

    fn build_best(self, turn: usize) -> Board {
        let boards = (0..7)
            .map(|domino_idx| self.clone().set_given_idx(turn, domino_idx).build_helper())
            .collect::<Vec<_>>();

        let is_team1 = turn.is_multiple_of(2);
        let (board, eval) = find_best_in(&boards, is_team1).unwrap();
        print!("\nChosen board's eval: {eval}");
        board.clone()
    }

    fn set_given_idx(mut self, turn: usize, domino_idx: usize) -> Self {
        self.settings = StartingPlayer(turn, GivenIdx(Some(domino_idx)));
        self
    }

    pub fn build(self) -> Board {
        if let StartingPlayer(turn, GivenIdx(None)) = self.settings {
            self.build_best(turn)
        } else {
            self.build_helper()
        }
    }

    fn build_helper(self) -> Board {
        let mut board = Board::new();

        for (idx, &[num1, num2]) in self.dominoes.iter().enumerate() {
            if Self::place_first(&self.settings, idx, num1, num2) {
                board.train_ends = [num1, num2];
                board.turn = idx / 7;
                continue;
            }

            board.add_domino_to(idx / 7, num1, num2);
        }

        if let StartingPlayer(turn, HumanInput) = self.settings {
            todo!("Get the human's first move, play that move.")
        }

        let _ = board.set_next_turn();

        board
    }

    fn place_first(settings: &BoardInitialSettings, idx: usize, num1: u8, num2: u8) -> bool {
        match settings {
            FirstRound => num1 == 6 && num2 == 6,
            &StartingPlayer(turn, choice) if turn == idx / 7 => match choice {
                HumanInput => false,
                GivenIdx(domino_idx) => domino_idx.unwrap() == idx % 7,
            },
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    hands: Vec<Vec<Vec<u8>>>,
    pip_totals: [u8; 4],
    train_ends: [u8; 2],
    turn: usize,
    eval: i16,
}

impl Board {
    fn new() -> Self {
        Self {
            hands: vec![(0..7).map(|_| Vec::with_capacity(7)).collect(); 4],
            pip_totals: [0; 4],
            train_ends: [u8::MAX; 2],
            turn: 0,
            eval: 0,
        }
    }

    fn add_domino_to(&mut self, player: usize, num1: u8, num2: u8) {
        self.hands[player][num1 as usize].push(num2);
        if num1 != num2 {
            self.hands[player][num2 as usize].push(num1);
        }

        self.pip_totals[player] += num1 + num2;
    }

    pub fn eval(&self) -> i16 {
        self.eval
    }

    pub fn is_team1(&self) -> bool {
        self.turn.is_multiple_of(2)
    }

    pub fn get_all_moves(&self) -> Vec<Self> {
        let mut new_boards = Vec::with_capacity(7);
        let [train_end1, train_end2] = self.train_ends;

        self.add_moves_at(train_end1, 0, &mut new_boards);
        if train_end1 == train_end2 {
            new_boards
        } else {
            self.add_moves_at(train_end2, 1, &mut new_boards);
            new_boards
        }
    }

    fn add_moves_at(&self, train_end: u8, train_idx: usize, new_boards: &mut Vec<Self>) {
        for (num2_idx, &num2) in self.hands[self.turn][train_end as usize].iter().enumerate() {
            let mut new_board = self.clone();
            new_board.play_domino(self.turn, train_end, num2_idx, train_idx);
            new_board.evaluate(self.turn, train_end, num2);
            new_boards.push(new_board);
        }
    }

    fn play_domino(&mut self, player: usize, num1: u8, num2_idx: usize, train_idx: usize) {
        let num2 = self.hands[player][num1 as usize].swap_remove(num2_idx);
        self.pip_totals[player] -= num1 + num2;
        self.train_ends[train_idx] = num2;

        if num1 == num2 {
            return;
        }

        let other_row = &mut self.hands[player][num2 as usize];
        let num1_idx = other_row.iter().position(|&value| value == num1).unwrap();
        let _ = other_row.swap_remove(num1_idx);
    }

    fn evaluate(&mut self, player: usize, num1: u8, num2: u8) {
        if self.pip_totals[player] == 0 && self.hands[player][0].is_empty() {
            self.eval = self.pip_totals.iter().sum::<u8>() as i16;
            self.eval += 25 * (self.train_ends[0] == self.train_ends[1] && num1 != num2) as i16;
            if player % 2 == 1 {
                self.eval *= -1;
            }

            return;
        }

        if self.set_next_turn() {
            return;
        }

        self.eval = self.pip_totals.iter().sum::<u8>() as i16;
        let curr_pip_total = self.pip_totals[player];
        let next_pip_total = self.pip_totals[(player + 1) % 4];
        let curr_wins = curr_pip_total <= next_pip_total;
        let curr_team2 = player % 2 == 1;
        if curr_wins && curr_team2 || !curr_wins && !curr_team2 {
            self.eval *= -1;
        }
    }

    fn set_next_turn(&mut self) -> bool {
        for _ in 0..4 {
            self.turn = (self.turn + 1) % 4;
            let hand = &self.hands[self.turn];
            let [train_end1, train_end2] = self.train_ends;
            if !(hand[train_end1 as usize].is_empty() && hand[train_end2 as usize].is_empty()) {
                return true;
            }
        }

        false
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

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn team1_runs_out_first() {
        let mut board = Board::new();
        board.add_domino_to(0, 1, 3);
        board.add_domino_to(1, 5, 5);
        board.add_domino_to(2, 2, 6);
        board.add_domino_to(3, 6, 6);
        board.add_domino_to(3, 2, 4);
        board.train_ends = [2, 2];
        board.turn = 2;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 1, 3);
        expected_board.add_domino_to(1, 5, 5);
        expected_board.add_domino_to(3, 6, 6);
        expected_board.add_domino_to(3, 2, 4);
        expected_board.train_ends = [6, 2];
        expected_board.turn = 2;
        expected_board.eval = expected_board.pip_totals.iter().sum::<u8>() as i16;

        assert_eq!(boards[0], expected_board);
    }

    #[test]
    fn team2_runs_out_first() {
        let mut board = Board::new();
        board.add_domino_to(0, 1, 3);
        board.add_domino_to(1, 5, 5);
        board.add_domino_to(2, 2, 6);
        board.add_domino_to(3, 6, 6);
        board.add_domino_to(3, 2, 4);
        board.train_ends = [3, 5];
        board.turn = 1;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 1, 3);
        expected_board.add_domino_to(2, 2, 6);
        expected_board.add_domino_to(3, 6, 6);
        expected_board.add_domino_to(3, 2, 4);
        expected_board.train_ends = [3, 5];
        expected_board.turn = 1;
        expected_board.eval = -(expected_board.pip_totals.iter().sum::<u8>() as i16);

        assert_eq!(boards[0], expected_board);
    }

    #[test]
    fn winning_team_gets_capicu() {
        let mut board = Board::new();
        board.add_domino_to(0, 1, 3);
        board.add_domino_to(1, 5, 5);
        board.add_domino_to(2, 2, 6);
        board.add_domino_to(3, 6, 6);
        board.add_domino_to(3, 2, 4);
        board.train_ends = [3, 1];
        board.turn = 0;
        let boards = board.get_all_moves();

        let mut expected_board1 = Board::new();
        expected_board1.add_domino_to(1, 5, 5);
        expected_board1.add_domino_to(2, 2, 6);
        expected_board1.add_domino_to(3, 6, 6);
        expected_board1.add_domino_to(3, 2, 4);
        expected_board1.train_ends = [1, 1];
        expected_board1.turn = 0;
        expected_board1.eval = (expected_board1.pip_totals.iter().sum::<u8>() + 25) as i16;

        let mut expected_board2 = expected_board1.clone();
        expected_board2.train_ends = [3, 3];

        assert_eq!(boards, [expected_board1, expected_board2]);
    }

    #[test]
    fn game_continues_when_double_0_is_last() {
        let mut board = Board::new();
        board.add_domino_to(0, 2, 4);
        board.add_domino_to(0, 2, 3);
        board.add_domino_to(0, 2, 2);
        board.add_domino_to(1, 0, 0);
        board.add_domino_to(1, 0, 6);
        board.add_domino_to(2, 0, 2);
        board.add_domino_to(2, 0, 4);
        board.add_domino_to(3, 0, 3);
        board.add_domino_to(3, 1, 2);
        board.train_ends = [1, 6];
        board.turn = 1;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 2, 4);
        expected_board.add_domino_to(0, 2, 3);
        expected_board.add_domino_to(0, 2, 2);
        expected_board.add_domino_to(1, 0, 0);
        expected_board.add_domino_to(2, 0, 2);
        expected_board.add_domino_to(2, 0, 4);
        expected_board.add_domino_to(3, 0, 3);
        expected_board.add_domino_to(3, 1, 2);
        expected_board.train_ends = [1, 0];
        expected_board.turn = 2;

        assert_eq!(boards[0], expected_board);
    }

    #[test]
    fn team1_blocks_and_wins() {
        let mut board = Board::new();
        board.add_domino_to(0, 0, 1);
        board.add_domino_to(0, 3, 4);
        board.add_domino_to(1, 1, 6);
        board.add_domino_to(2, 2, 5);
        board.add_domino_to(3, 0, 5);
        board.train_ends = [4, 4];
        board.turn = 0;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 0, 1);
        expected_board.add_domino_to(1, 1, 6);
        expected_board.add_domino_to(2, 2, 5);
        expected_board.add_domino_to(3, 0, 5);
        expected_board.train_ends = [3, 4];
        expected_board.turn = 0;
        expected_board.eval = expected_board.pip_totals.iter().sum::<u8>() as i16;

        assert_eq!(boards[0], expected_board);
    }

    #[test]
    fn team1_blocks_and_loses() {
        let mut board = Board::new();
        board.add_domino_to(0, 0, 1);
        board.add_domino_to(1, 1, 6);
        board.add_domino_to(2, 2, 5);
        board.add_domino_to(2, 3, 4);
        board.add_domino_to(3, 0, 5);
        board.train_ends = [4, 4];
        board.turn = 2;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 0, 1);
        expected_board.add_domino_to(1, 1, 6);
        expected_board.add_domino_to(2, 2, 5);
        expected_board.add_domino_to(3, 0, 5);
        expected_board.train_ends = [3, 4];
        expected_board.turn = 2;
        expected_board.eval = -(expected_board.pip_totals.iter().sum::<u8>() as i16);

        assert_eq!(boards[0], expected_board);
    }

    #[test]
    fn team2_blocks_and_wins() {
        let mut board = Board::new();
        board.add_domino_to(0, 0, 1);
        board.add_domino_to(1, 1, 6);
        board.add_domino_to(1, 3, 4);
        board.add_domino_to(2, 2, 5);
        board.add_domino_to(3, 0, 5);
        board.train_ends = [4, 4];
        board.turn = 1;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 0, 1);
        expected_board.add_domino_to(1, 1, 6);
        expected_board.add_domino_to(2, 2, 5);
        expected_board.add_domino_to(3, 0, 5);
        expected_board.train_ends = [3, 4];
        expected_board.turn = 1;
        expected_board.eval = -(expected_board.pip_totals.iter().sum::<u8>() as i16);

        assert_eq!(boards[0], expected_board);
    }

    #[test]
    fn team2_blocks_and_loses() {
        let mut board = Board::new();
        board.add_domino_to(0, 0, 1);
        board.add_domino_to(1, 1, 6);
        board.add_domino_to(2, 2, 5);
        board.add_domino_to(3, 0, 5);
        board.add_domino_to(3, 3, 4);
        board.train_ends = [4, 4];
        board.turn = 3;
        let boards = board.get_all_moves();
        assert_eq!(boards.len(), 1);

        let mut expected_board = Board::new();
        expected_board.add_domino_to(0, 0, 1);
        expected_board.add_domino_to(1, 1, 6);
        expected_board.add_domino_to(2, 2, 5);
        expected_board.add_domino_to(3, 0, 5);
        expected_board.train_ends = [3, 4];
        expected_board.turn = 3;
        expected_board.eval = expected_board.pip_totals.iter().sum::<u8>() as i16;

        assert_eq!(boards[0], expected_board);
    }
}
