use chess::{Board, Color, Piece, Rank, File, ALL_SQUARES};
use std::collections::HashMap;

/// Tracks how many of each piece type have been captured.
pub struct CapturedPieces {
    pub pawns: usize,
    pub knights: usize,
    pub bishops: usize,
    pub rooks: usize,
    pub queens: usize,
}

/// Count how many pieces of a given color have been captured from the board.
pub fn get_captured_pieces(board: &Board, color: Color) -> CapturedPieces {
    let mut counts = HashMap::new();
    for sq in ALL_SQUARES.iter() {
        if let Some(piece) = board.piece_on(*sq) {
            if board.color_on(*sq) == Some(color) {
                *counts.entry(piece).or_insert(0) += 1;
            }
        }
    }

    let get_captured = |piece: Piece, starting: usize| {
        let current = *counts.get(&piece).unwrap_or(&0);
        if starting > current {
            starting - current
        } else {
            0
        }
    };

    CapturedPieces {
        pawns: get_captured(Piece::Pawn, 8),
        knights: get_captured(Piece::Knight, 2),
        bishops: get_captured(Piece::Bishop, 2),
        rooks: get_captured(Piece::Rook, 2),
        queens: get_captured(Piece::Queen, 1),
    }
}

/// Board traversal constants — ranks ordered for White's perspective (8→1).
pub const RANKS_WHITE: [Rank; 8] = [
    Rank::Eighth, Rank::Seventh, Rank::Sixth, Rank::Fifth,
    Rank::Fourth, Rank::Third, Rank::Second, Rank::First,
];

/// Board traversal constants — files ordered for White's perspective (A→H).
pub const FILES_WHITE: [File; 8] = [
    File::A, File::B, File::C, File::D,
    File::E, File::F, File::G, File::H,
];

/// Board traversal constants — ranks ordered for Black's perspective (1→8).
pub const RANKS_BLACK: [Rank; 8] = [
    Rank::First, Rank::Second, Rank::Third, Rank::Fourth,
    Rank::Fifth, Rank::Sixth, Rank::Seventh, Rank::Eighth,
];

/// Board traversal constants — files ordered for Black's perspective (H→A).
pub const FILES_BLACK: [File; 8] = [
    File::H, File::G, File::F, File::E,
    File::D, File::C, File::B, File::A,
];

/// Map a chess piece + color to its Lichess SVG image URL.
pub fn get_piece_image_url(piece: Piece, color: Color) -> &'static str {
    match (color, piece) {
        (Color::White, Piece::Pawn) => "https://lichess1.org/assets/piece/cburnett/wP.svg",
        (Color::White, Piece::Knight) => "https://lichess1.org/assets/piece/cburnett/wN.svg",
        (Color::White, Piece::Bishop) => "https://lichess1.org/assets/piece/cburnett/wB.svg",
        (Color::White, Piece::Rook) => "https://lichess1.org/assets/piece/cburnett/wR.svg",
        (Color::White, Piece::Queen) => "https://lichess1.org/assets/piece/cburnett/wQ.svg",
        (Color::White, Piece::King) => "https://lichess1.org/assets/piece/cburnett/wK.svg",
        (Color::Black, Piece::Pawn) => "https://lichess1.org/assets/piece/cburnett/bP.svg",
        (Color::Black, Piece::Knight) => "https://lichess1.org/assets/piece/cburnett/bN.svg",
        (Color::Black, Piece::Bishop) => "https://lichess1.org/assets/piece/cburnett/bB.svg",
        (Color::Black, Piece::Rook) => "https://lichess1.org/assets/piece/cburnett/bR.svg",
        (Color::Black, Piece::Queen) => "https://lichess1.org/assets/piece/cburnett/bQ.svg",
        (Color::Black, Piece::King) => "https://lichess1.org/assets/piece/cburnett/bK.svg",
    }
}
