use std::ops::Deref;
use Branch::*;
use Direction::*;
use OccupiedType::*;

const WIDTH: usize = 189 / 6; // terminal width / worst case width of domino unit
const HEIGHT: usize = (31 - 3) / 3; // (terminal height - lines of space for info) / worst case height of domino unit

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum OccupiedType {
    Empty,
    Occupied,
    HalfOccupied,
}

type Attachment = (Domino, Direction);
type AttachmentPoint = ([usize; 2], Direction);

pub struct DominoTrain {
    starting_domino: Domino,
    left_branch: Vec<Attachment>,
    right_branch: Vec<Attachment>,
    left_attachment_points: Vec<AttachmentPoint>,
    right_attachment_points: Vec<AttachmentPoint>,
    occupied_squares: [[OccupiedType; WIDTH]; HEIGHT],
}

pub enum Branch {
    LeftBranch,
    RightBranch,
}

impl DominoTrain {
    pub fn new(starting_domino: Domino) -> Self {
        let mut occupied_squares = [[Empty; WIDTH]; HEIGHT];
        occupied_squares[HEIGHT / 2][WIDTH / 2] = Occupied;

        let mut left_attachment_points = Vec::with_capacity(3);
        left_attachment_points.push(([HEIGHT / 2, WIDTH / 2 - 1], Left));

        let mut right_attachment_points = Vec::with_capacity(3);

        if starting_domino[0] == starting_domino[1] {
            occupied_squares[HEIGHT / 2 - 1][WIDTH / 2] = HalfOccupied;
            occupied_squares[HEIGHT / 2 + 1][WIDTH / 2] = HalfOccupied;
            right_attachment_points.push(([HEIGHT / 2, WIDTH / 2 + 1], Right));
        } else {
            occupied_squares[HEIGHT / 2][WIDTH / 2 + 1] = Occupied;
            right_attachment_points.push(([HEIGHT / 2, WIDTH / 2 + 2], Right));
        }

        Self {
            starting_domino,
            left_branch: vec![],
            right_branch: vec![],
            left_attachment_points,
            right_attachment_points,
            occupied_squares,
        }
    }

    pub fn add_domino_to(&mut self, domino: Domino, branch: Branch) {
        let (train_branch, attachment_points) = match branch {
            LeftBranch => (&mut self.left_branch, &mut self.left_attachment_points),
            RightBranch => (&mut self.right_branch, &mut self.right_attachment_points),
        };


    }
}

pub struct Domino([u8; 2]);

impl Domino {
    fn new(pip_vals: [u8; 2]) -> Option<Self> {
        if pip_vals[0] <= 6 && pip_vals[1] <= 6 {
            Some(Domino(pip_vals))
        } else {
            None
        }
    }
}

impl Deref for Domino {
    type Target = [u8; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: the concept of an attachment needs to become a struct
// TODO: also, update both ends of attachments as some could be blocked off on one end, as you add dominoes to anther end
// TODO: also new attachments can become possible after a domino is added so this has to be handled too

// TODO: Each non-double domino takes a 1x2 or 2x1 area where both squares are considered occupied
// TODO: the expansion points of a domino can't be an occupied square.
// TODO: doubles take a 1x3 or 3x1 area (2 'half' occupied squares on either side of an occupied square).
//   - A half occupied square is either vertical or horizontal.
//   - An expansion point may lie on a half occupied square, if the orientations are correct
//     (e.g. a downwards expansion could go onto a horizontal half occupied square but not a vertical one).

// println!(
//     "+---+
// | 5 |
// |---| +-------+
// | 6 | | 6 | 0 |
// +---+ +-------+\n"
// );
//
// println!(
//     "+---+
// | 6 | +-------+
// |---| | 6 | 0 |
// | 6 | +-------+
// +---+\n"
// );
//
// println!(
//     "+-------+
// | 1 | 1 |
// +---+---+
// +---+
// | 1 |
// |---|
// | 2 |
// +---+\n"
// );
//
// println!(
//     "+-------+
// | 1 | 1 |
// +---+---+
//   +---+
//   | 1 |
//   |---|
//   | 2 |
//   +---+\n"
// );
