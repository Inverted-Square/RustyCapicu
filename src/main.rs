use crate::board::BoardBuilder;
use crate::board::DominoIdx::*;
use crate::board::FirstDominoChoice::*;
use crate::board::Turn::*;

mod board;

fn main() {
    let board1 = BoardBuilder::new().build();
    let board2 = BoardBuilder::new().starting_player(Player1, GivenIdx(Second)).build();
    let board3 = BoardBuilder::new().starting_player(Player3, HumanInput);
}
