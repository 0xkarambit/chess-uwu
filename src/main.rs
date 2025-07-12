

// First we create a board and all the pieces !
// Row, Col only needs to represent 8 distinct values so we only need 3 bits
type Row = u8;
type Col = u8;
type Position = (Row, Col); // position is (col, row)
type Square = Option<Piece>;

// Board must be accessed like this board[row][col]
const BOARDSIZE: usize = 8;
type Board = [[Square; BOARDSIZE]; BOARDSIZE];

// Board functions (make it a module)

// ♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ 
pub mod board_mod {
    use super::*;

    pub(crate) fn print(board: &Board) {
        for (idx, row) in board.iter().enumerate() {
            print!("{} ", BOARDSIZE - idx);
            for square in row {
                match square {
                    Some(piece) => {
                        match (piece.is_black, piece.kind) {
                            (true, PieceKinds::KING) => print!("♚"),
                            (true, PieceKinds::QUEEN) => print!("♛"),
                            (true, PieceKinds::BISHOP) => print!("♝"),
                            (true, PieceKinds::KNIGHT) => print!("♞"),
                            (true, PieceKinds::ROOK) => print!("♜"),
                            (true, PieceKinds::PAWN) => print!("♟"),
                            // White pieces
                            (false, PieceKinds::KING) => print!("♔"),
                            (false, PieceKinds::QUEEN) => print!("♕"),
                            (false, PieceKinds::BISHOP) => print!("♗"),
                            (false, PieceKinds::KNIGHT) => print!("♘"),
                            (false, PieceKinds::ROOK) => print!("♖"),
                            (false, PieceKinds::PAWN) => print!("♙"),
                        }
                        print!(" ");
                    },
                    None => print!("  ")
                }
            }
            println!();
        }
        // offset for the vertical numbers
        print!("  ");
        for i in 'a'..='h' {
            print!("{} ", i)
        }
    }


    pub(crate) fn init(board: &mut Board) {
        use PieceKinds::*;
        
        board[0] = [
            Some(Piece::new(ROOK, true)),
            Some(Piece::new(KNIGHT, true)),
            Some(Piece::new(BISHOP, true)),
            Some(Piece::new(QUEEN, true)),
            Some(Piece::new(KING, true)),
            Some(Piece::new(BISHOP, true)),
            Some(Piece::new(KNIGHT, true)),
            Some(Piece::new(ROOK, true)),
        ];

        board[1] = [Some(Piece::new(PAWN, true)); 8];

        board[7] = [
            Some(Piece::new(ROOK, false)),
            Some(Piece::new(KNIGHT, false)),
            Some(Piece::new(BISHOP, false)),
            Some(Piece::new(QUEEN, false)),
            Some(Piece::new(KING, false)),
            Some(Piece::new(BISHOP, false)),
            Some(Piece::new(KNIGHT, false)),
            Some(Piece::new(ROOK, false)),
        ];

        board[6] = [Some(Piece::new(PAWN, false)); 8];
}
}

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
    pub fn legal_moves(&self, board: &Board) -> Vec<Position> {
        !unimplemented!()
    }
} 


// Pieces

fn main() {
    let mut board: Board = [[None; BOARDSIZE]; BOARDSIZE];
    board_mod::init(&mut board);
    board_mod::print(&board);
}
