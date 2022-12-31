mod bitboard;
mod piece_moves;
mod cmove;
use bitboard::Bitboard;

/// The colors of pieces
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opp(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// All chess piece types
#[derive(Clone, Copy)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King,
}

/// All eight cardinal directions
#[derive(Clone, Copy)]
pub enum Dir {
    Nort, Noea, East, Soea, Sout, Sowe, West, Nowe,
}

impl Dir {
    pub fn neg(&self) -> bool {
        match *self {
            Dir::West | Dir::Sout | Dir::Sowe | Dir::Soea => true,
            _ => false,
        }
    }

    pub fn pos(&self) -> bool {
        match *self {
            Dir::West | Dir::Sout | Dir::Sowe | Dir::Soea => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

use Square::*;

impl Square {
    pub fn from_u16(i: u16) -> Option<Square> {
        match i {
            0 => Some(A1), 1 => Some(B1), 2 => Some(C1), 3 => Some(D1), 4 => Some(E1), 5 => Some(F1), 6 => Some(G1), 7 => Some(H1),
            8 => Some(A2), 9 => Some(B2), 10 => Some(C2), 11 => Some(D2), 12 => Some(E2), 13 => Some(F2), 14 => Some(G2), 15 => Some(H2),
            16 => Some(A3), 17 => Some(B3), 18 => Some(C3), 19 => Some(D3), 20 => Some(E3), 21 => Some(F3), 22 => Some(G3), 23 => Some(H3),
            24 => Some(A4), 25 => Some(B4), 26 => Some(C4), 27 => Some(D4), 28 => Some(E4), 29 => Some(F4), 30 => Some(G4), 31 => Some(H4),
            32 => Some(A5), 33 => Some(B5), 34 => Some(C5), 35 => Some(D5), 36 => Some(E5), 37 => Some(F5), 38 => Some(G5), 39 => Some(H5),
            40 => Some(A6), 41 => Some(B6), 42 => Some(C6), 43 => Some(D6), 44 => Some(E6), 45 => Some(F6), 46 => Some(G6), 47 => Some(H6),
            48 => Some(A7), 49 => Some(B7), 50 => Some(C7), 51 => Some(D7), 52 => Some(E7), 53 => Some(F7), 54 => Some(G7), 55 => Some(H7),
            56 => Some(A8), 57 => Some(B8), 58 => Some(C8), 59 => Some(D8), 60 => Some(E8), 61 => Some(F8), 62 => Some(G8), 63 => Some(H8),
            _ => None,
        }
    }
}

/// The main `Board` struct, which contains 10 bitboards
/// The `piece_bb` array first contains bitboards marking the presense of
/// pawns, knights, bishops, rooks, queens, and kings respectively, 
/// regardless of color. The 7th and 8th boards in the array mark the presense
/// of white and black pieces respectively, which can be intersected with
/// the previous indexed boards to obtain the location of only white or only
/// black pieces.
/// 
/// The `empty_bb` and `occupied_bb` boards mark the absense of and the presense 
/// of pieces, respectively.
pub struct Board {
    piece_bb: [Bitboard; 8],
    empty_bb: Bitboard,
    occupied_bb: Bitboard,
}

impl Board {
    /// Creates a new Bitboard struct with beginning piece
    /// placements for each bitboard
    pub fn new() -> Board {
        Board {
            piece_bb: [
                bitboard::PAWN_START,
                bitboard::KNIGHT_START,
                bitboard::BISHOP_START,
                bitboard::ROOK_START,
                bitboard::QUEEN_START,
                bitboard::KING_START,
                bitboard::WHITE_START,
                bitboard::BLACK_START,
            ],
            empty_bb: bitboard::EMPTY_START,
            occupied_bb: bitboard::OCCUPIED_START,
        }
    }

    /// Returns the appropriate piece bitboard for
    /// piece `p` intersected with the piece bitboard
    /// for the color `c`, if `c` is not `None`
    pub fn piece_bb(&self, c: Option<Color>, p: Piece) -> Bitboard {
        let intersection = match c {
            Some(c) => self.piece_bb[6 + c as usize],
            None => Bitboard(!0),
        };
        self.piece_bb[p as usize] & intersection
    }
}