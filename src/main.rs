use crate::board::BoardBuilder;
use crate::board::DominoIdx::*;
use crate::board::FirstDominoChoice::*;
use crate::board::Turn::*;

mod board;

fn main() {
    for board in BoardBuilder::new().build_all(Player3) {
        println!("{board}");
    }

    let board = BoardBuilder::new()
        .starting_player(Player3, GivenIdx(Forth))
        .build();
    println!("{board}{board:?}");
}
