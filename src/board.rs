use crate::{Piece, PieceKinds, Square};


// Board must be accessed like this board[row][col]
pub const BOARDSIZE: usize = 8;
pub type BoardType = [[Square; BOARDSIZE]; BOARDSIZE];

// Board functions (make it a module)
enum PlaySide {
    WHITE,
    BLACK
}

pub(crate) struct Board {
    state: BoardType,
    turn: PlaySide,
}
// ♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ 

impl Board {
    pub fn new() -> Self {
        let mut a = Self {
            state: [[None; BOARDSIZE]; BOARDSIZE],
            turn: PlaySide::WHITE
        };
        a.init();
        a
    }

    pub fn load_fen(&mut self, fen: &str) -> Result<(), &str> {
        // https://www.chess.com/terms/fen-chess
        // Pieces
        // - Lowercase letters describe the black pieces. Just like in PGN, "p" stands for pawn, "r" for rook, "n" for knight, "b" for bishop, "q" for queen, and "k" for king. The same letters are used for the white pieces, but they appear in uppercase. Empty squares are denoted by numbers from one to eight, depending on how many empty squares are between two pieces.
        // who moves next.
        // - This field always appears in lowercase, and "w" specifies that it is White's turn to move, while "b" indicates that Black plays next.
        // castling rights. 
        // - The letter "k" indicates that kingside castling is available, while "q" means that a player may castle queenside. The symbol "-" designates that neither side may castle. 
        // Possible En Passant Targets. 
        // - the FEN string adds the square behind the pawn in algebraic notation in its fourth field. If no en passant targets are available, the "-" symbol is used.
        // Halfmove Clock
        // - total no of halfmoves moved so far
        // Fullmove Number
        // - The sixth and last field of the FEN code shows the number of completed turns in the game. This number is incremented by one every time Black moves. Chess programmers call this a fullmove.
        let parts: Vec<_> = fen.split(' ').collect();

        if parts.len() != 6 {
            return Err("Invalid FEN, must have 6 space separated parts");
        }

        self.fen_piece_placement(parts[0])?;

        Ok(())

    }

    fn fen_piece_placement(&mut self, pieces: &str) -> Result<(), &str> {
        use PieceKinds::*;
        const ROW_SEPARATOR: char = '/';
        // First segment of the FEN string


        if pieces.matches(ROW_SEPARATOR).count() != 7 {
            return Err("must have 8 rows separated by '/'");
        }

        for ( row_idx, row ) in pieces.split(ROW_SEPARATOR).enumerate() {

            if row_idx > BOARDSIZE {
                return Err("Too many rows specified in FEN")
            }
            let mut col_idx = 0;
            for c in row.chars() {

                if let Some(squares_to_skip) = c.to_digit(10) {

                    if col_idx + squares_to_skip > BOARDSIZE as u32 {
                        return Err("Too many columns specified in a row")
                    }

                    for i in 0..squares_to_skip {
                        let idx = col_idx + i;
                        self.state[row_idx][idx as usize] = None;
                    }

                    // Incremented column counter with the no of squares skipped
                    col_idx += squares_to_skip;
                }
                else {
                    let piece = match c {
                        // lowercase letters mean black pieces
                        'k' => Piece::new(KING, true),
                        'q' => Piece::new(QUEEN, true),
                        'b' => Piece::new(BISHOP, true),
                        'n' => Piece::new(KNIGHT, true),
                        'r' => Piece::new(ROOK, true),
                        'p' => Piece::new(PAWN, true),
                        // uppercase letters represent white pieces
                        'K' => Piece::new(KING, false),
                        'Q' => Piece::new(QUEEN, false),
                        'B' => Piece::new(BISHOP, false),
                        'N' => Piece::new(KNIGHT, false),
                        'R' => Piece::new(ROOK, false),
                        'P' => Piece::new(PAWN, false),
                        _ => return Err("Invalid character in FEN piece placement")
                    };

                    if col_idx > BOARDSIZE as u32 {
                        return Err("Too many columns in a single row")
                    }

                    self.state[row_idx][col_idx as usize] = Some(piece);
                    col_idx += 1;
                }
            }
        }

        Ok(())

    }

    pub fn init(&mut self) {
        use PieceKinds::*;
        self.state[0] = [
            Some(Piece::new(ROOK, true)),
            Some(Piece::new(KNIGHT, true)),
            Some(Piece::new(BISHOP, true)),
            Some(Piece::new(QUEEN, true)),
            Some(Piece::new(KING, true)),
            Some(Piece::new(BISHOP, true)),
            Some(Piece::new(KNIGHT, true)),
            Some(Piece::new(ROOK, true)),
        ];

        self.state[1] = [Some(Piece::new(PAWN, true)); 8];

        self.state[7] = [
            Some(Piece::new(ROOK, false)),
            Some(Piece::new(KNIGHT, false)),
            Some(Piece::new(BISHOP, false)),
            Some(Piece::new(QUEEN, false)),
            Some(Piece::new(KING, false)),
            Some(Piece::new(BISHOP, false)),
            Some(Piece::new(KNIGHT, false)),
            Some(Piece::new(ROOK, false)),
        ];

        self.state[6] = [Some(Piece::new(PAWN, false)); 8];
    }


    pub fn print(&self) {
        for (idx, row) in self.state.iter().enumerate() {
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

}
