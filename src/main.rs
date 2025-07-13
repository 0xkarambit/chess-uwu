
mod board;
use board::Board;

// First we create a board and all the pieces !
// Row, Col only needs to represent 8 distinct values so we only need 3 bits
type Row = u8;
type Col = u8;
type Position = (Row, Col); // position is (col, row)
type Square = Option<Piece>;

#[derive(Clone, Copy, Debug)]
enum PieceKinds {
    KING,
    QUEEN,
    BISHOP,
    KNIGHT,
    ROOK,
    PAWN
}

#[derive(Clone, Copy, Debug)]
struct Piece {
    kind: PieceKinds,
    // does a piece need to store its position ? or on the other hand... do we need to store a 2D Array for the chess board.
    // I think we need to store the positions in a global 2D array because we need to be able to get the pieces by positions...
    // Maybe we could use a HashMap<(Row, Col), Piece> for it ?
    // Okie, I dont think it matters that much for now UwU
    position: Position, 
    is_black: bool,
    has_moved_before: bool
}

impl Piece {
    fn new(kind: PieceKinds, is_black:bool) -> Self {
        Self {
            kind,
            is_black,
            position: (0,0),
            has_moved_before: false,
        }
    }
    pub fn legal_moves(&self, board: &board::BoardType) -> Vec<Position> {
        !unimplemented!()
    }
} 


// Pieces

const FEN: &'static str = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";
//  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b - - 99 50";


fn main() {
    // let mut board: BoardType = [[None; BOARDSIZE]; BOARDSIZE];
    let mut board = Board::new();
    match board.load_fen(FEN) {
        Ok(()) => board.print(),
        Err(msg) => panic!("{}", msg),
    }
}
