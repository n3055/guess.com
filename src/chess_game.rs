use std::sync::Mutex;
use std::collections::HashMap;
use std::str::FromStr;
use chess::{Board, ChessMove, Color, Piece, Square, BoardStatus, Rank, File};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ChessGame {
    pub id: String,
    pub fen: String,
    pub white_player: Option<String>,
    pub black_player: Option<String>,
    pub status: String, // "waiting", "active", "checkmate", "draw", "stalemate"
    pub winner: Option<String>, // "White", "Black", "Draw", None
    pub moves: Vec<String>,
    pub variant: String, // "standard", "mad_scientist"
    pub white_pocket: Vec<String>,
    pub black_pocket: Vec<String>,
}

fn make_drop_board(
    board: &Board,
    drop_piece: Piece,
    drop_color: Color,
    to_sq: Square,
) -> Result<Board, String> {
    if board.piece_on(to_sq).is_some() {
        return Err("Destination square is occupied".to_string());
    }

    if drop_piece == Piece::Pawn && (to_sq.get_rank() == Rank::First || to_sq.get_rank() == Rank::Eighth) {
        return Err("Pawns cannot be dropped on the 1st or 8th rank".to_string());
    }

    // Build FEN piece placement
    let mut placement = String::new();
    for rank_idx in (0..8).rev() {
        let mut empty_count = 0;
        for file_idx in 0..8 {
            let file = File::from_index(file_idx);
            let rank = Rank::from_index(rank_idx);
            let sq = Square::make_square(rank, file);
            
            let (p, c) = if sq == to_sq {
                (Some(drop_piece), Some(drop_color))
            } else {
                (board.piece_on(sq), board.color_on(sq))
            };

            if let (Some(piece), Some(color)) = (p, c) {
                if empty_count > 0 {
                    placement.push_str(&empty_count.to_string());
                    empty_count = 0;
                }
                let char_repr = match piece {
                    Piece::Pawn => 'p',
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Rook => 'r',
                    Piece::Queen => 'q',
                    Piece::King => 'k',
                };
                let char_repr = if color == Color::White {
                    char_repr.to_ascii_uppercase()
                } else {
                    char_repr
                };
                placement.push(char_repr);
            } else {
                empty_count += 1;
            }
        }
        if empty_count > 0 {
            placement.push_str(&empty_count.to_string());
        }
        if rank_idx > 0 {
            placement.push('/');
        }
    }

    let orig_fen = board.to_string();
    let parts: Vec<&str> = orig_fen.split_whitespace().collect();
    if parts.len() < 6 {
        return Err("Invalid original FEN format".to_string());
    }
    let castling = parts[2];
    let next_color = if drop_color == Color::White { "b" } else { "w" };
    let en_passant = "-";
    let halfmove = "0";
    
    let mut fullmove: u32 = parts[5].parse().unwrap_or(1);
    if drop_color == Color::Black {
        fullmove += 1;
    }

    let new_fen = format!(
        "{} {} {} {} {} {}",
        placement, next_color, castling, en_passant, halfmove, fullmove
    );

    Board::from_str(&new_fen).map_err(|e| format!("Invalid drop position: {}", e))
}

pub struct GameManager {
    pub games: Mutex<HashMap<String, ChessGame>>,
    pub matchmaking_queue_standard: Mutex<Option<String>>,
    pub matchmaking_queue_mad_scientist: Mutex<Option<String>>,
    pub active_matches: Mutex<HashMap<String, String>>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            games: Mutex::new(HashMap::new()),
            matchmaking_queue_standard: Mutex::new(None),
            matchmaking_queue_mad_scientist: Mutex::new(None),
            active_matches: Mutex::new(HashMap::new()),
        }
    }

    pub fn join_matchmaking(&self, player_id: &str, variant: &str) -> Option<String> {
        // Clear any old active match for this player when they start a new search
        {
            let mut active = self.active_matches.lock().unwrap();
            active.remove(player_id);
        }

        let mut queue = if variant == "mad_scientist" {
            self.matchmaking_queue_mad_scientist.lock().unwrap()
        } else {
            self.matchmaking_queue_standard.lock().unwrap()
        };

        if let Some(other_player_id) = queue.clone() {
            if other_player_id == player_id {
                return None; // Already in queue
            }

            // Remove from queue and pair up
            *queue = None;

            // Create game
            let game_id = uuid::Uuid::new_v4().to_string();
            let starting_fen = Board::default().to_string();
            let game = ChessGame {
                id: game_id.clone(),
                fen: starting_fen,
                white_player: Some(other_player_id.clone()),
                black_player: Some(player_id.to_string()),
                status: "active".to_string(),
                winner: None,
                moves: Vec::new(),
                variant: variant.to_string(),
                white_pocket: Vec::new(),
                black_pocket: Vec::new(),
            };

            let mut games = self.games.lock().unwrap();
            games.insert(game_id.clone(), game);

            // Record active match for both players
            let mut active = self.active_matches.lock().unwrap();
            active.insert(other_player_id, game_id.clone());
            active.insert(player_id.to_string(), game_id.clone());

            Some(game_id)
        } else {
            *queue = Some(player_id.to_string());
            None
        }
    }

    pub fn check_matchmaking(&self, player_id: &str) -> Option<String> {
        let active = self.active_matches.lock().unwrap();
        active.get(player_id).cloned()
    }

    pub fn cancel_matchmaking(&self, player_id: &str) {
        {
            let mut queue = self.matchmaking_queue_standard.lock().unwrap();
            if queue.as_deref() == Some(player_id) {
                *queue = None;
            }
        }
        {
            let mut queue = self.matchmaking_queue_mad_scientist.lock().unwrap();
            if queue.as_deref() == Some(player_id) {
                *queue = None;
            }
        }
        let mut active = self.active_matches.lock().unwrap();
        active.remove(player_id);
    }

    pub fn create_game(&self, variant: String) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let starting_fen = Board::default().to_string();
        let game = ChessGame {
            id: id.clone(),
            fen: starting_fen,
            white_player: None,
            black_player: None,
            status: "waiting".to_string(),
            winner: None,
            moves: Vec::new(),
            variant,
            white_pocket: Vec::new(),
            black_pocket: Vec::new(),
        };
        let mut games = self.games.lock().unwrap();
        games.insert(id.clone(), game);
        id
    }

    pub fn get_game(&self, id: &str) -> Option<ChessGame> {
        let games = self.games.lock().unwrap();
        games.get(id).cloned()
    }

    pub fn join_game(&self, id: &str, player_id: &str, role: &str) -> Result<ChessGame, String> {
        let mut games = self.games.lock().unwrap();
        let game = games.get_mut(id).ok_or_else(|| "Game not found".to_string())?;

        // If player is already in the game under a different role, don't allow duplicates
        // (Commented out to allow same-browser/same-player self-play and testing)
        /*
        if game.white_player.as_deref() == Some(player_id) && role == "black" {
            return Err("You are already playing as White".to_string());
        }
        if game.black_player.as_deref() == Some(player_id) && role == "white" {
            return Err("You are already playing as Black".to_string());
        }
        */

        match role {
            "white" => {
                if game.white_player.is_none() {
                    game.white_player = Some(player_id.to_string());
                } else if game.white_player.as_deref() != Some(player_id) {
                    return Err("White slot is already taken".to_string());
                }
            }
            "black" => {
                if game.black_player.is_none() {
                    game.black_player = Some(player_id.to_string());
                } else if game.black_player.as_deref() != Some(player_id) {
                    return Err("Black slot is already taken".to_string());
                }
            }
            "spectator" => {
                // Just viewing, no slots updated
            }
            _ => {
                // Auto assign
                if game.white_player.is_none() {
                    game.white_player = Some(player_id.to_string());
                } else if game.black_player.is_none() && game.white_player.as_deref() != Some(player_id) {
                    game.black_player = Some(player_id.to_string());
                }
            }
        }

        // If both slots are full, start the game
        if game.white_player.is_some() && game.black_player.is_some() && game.status == "waiting" {
            game.status = "active".to_string();
        }

        Ok(game.clone())
    }

    pub fn make_move(&self, id: &str, player_id: &str, from_sq_str: &str, to_sq_str: &str, promo_str: Option<&str>) -> Result<ChessGame, String> {
        let mut games = self.games.lock().unwrap();
        let game = games.get_mut(id).ok_or_else(|| "Game not found".to_string())?;

        if game.status != "active" {
            return Err(format!("Game is not active (status: {})", game.status));
        }

        let board = Board::from_str(&game.fen).map_err(|e| format!("Invalid FEN in game: {}", e))?;
        let side_to_move = board.side_to_move();

        // Verify it is this player's turn
        match side_to_move {
            Color::White => {
                if game.white_player.as_deref() != Some(player_id) {
                    return Err("It is White's turn, but you are not White".to_string());
                }
            }
            Color::Black => {
                if game.black_player.as_deref() != Some(player_id) {
                    return Err("It is Black's turn, but you are not Black".to_string());
                }
            }
        }

        let new_board = if from_sq_str.starts_with("drop:") {
            if game.variant != "mad_scientist" {
                return Err("Drops are only allowed in Mad Scientist variant".to_string());
            }

            let piece_name = from_sq_str.strip_prefix("drop:").unwrap().to_lowercase();
            let drop_piece = match piece_name.as_str() {
                "p" | "pawn" => Piece::Pawn,
                "n" | "knight" => Piece::Knight,
                "b" | "bishop" => Piece::Bishop,
                "r" | "rook" => Piece::Rook,
                "q" | "queen" => Piece::Queen,
                _ => return Err("Invalid drop piece".to_string()),
            };

            let to_sq = Square::from_str(to_sq_str).map_err(|_| format!("Invalid to square: {}", to_sq_str))?;

            // Try to find and remove from pocket
            let pocket = if side_to_move == Color::White {
                &mut game.white_pocket
            } else {
                &mut game.black_pocket
            };
            let idx = pocket.iter().position(|x| x == &piece_name)
                .ok_or_else(|| format!("Piece not available in pocket: {}", piece_name))?;
            pocket.remove(idx);

            let nb = make_drop_board(&board, drop_piece, side_to_move, to_sq)?;
            
            let move_desc = format!("{}@{}", piece_name.to_ascii_uppercase(), to_sq_str);
            game.moves.push(move_desc);
            nb
        } else {
            let from_sq = Square::from_str(from_sq_str).map_err(|_| format!("Invalid from square: {}", from_sq_str))?;
            let to_sq = Square::from_str(to_sq_str).map_err(|_| format!("Invalid to square: {}", to_sq_str))?;

            // Determine if promotion is required (pawn moving to 8th rank for White, 1st rank for Black)
            let piece = board.piece_on(from_sq);
            let is_pawn = piece == Some(Piece::Pawn);
            let is_promo_rank = match side_to_move {
                Color::White => to_sq.get_rank() == Rank::Eighth,
                Color::Black => to_sq.get_rank() == Rank::First,
            };

            let promotion = if is_pawn && is_promo_rank {
                let promo_piece = match promo_str.unwrap_or("q").to_lowercase().as_str() {
                    "n" | "knight" => Piece::Knight,
                    "b" | "bishop" => Piece::Bishop,
                    "r" | "rook" => Piece::Rook,
                    _ => Piece::Queen,
                };
                Some(promo_piece)
            } else {
                None
            };

            let mv = ChessMove::new(from_sq, to_sq, promotion);

            if !board.legal(mv) {
                return Err("Move is not legal".to_string());
            }

            // Track capture
            let captured = if board.piece_on(to_sq).is_some() {
                let cap_piece = board.piece_on(to_sq).unwrap();
                let cap_color = board.color_on(to_sq).unwrap();
                Some((cap_piece, cap_color))
            } else if is_pawn && from_sq.get_file() != to_sq.get_file() {
                // En-passant capture!
                let cap_color = match side_to_move {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                };
                Some((Piece::Pawn, cap_color))
            } else {
                None
            };

            let nb = board.make_move_new(mv);

            if game.variant == "mad_scientist" {
                if let Some((cap_piece, cap_color)) = captured {
                    let piece_str = match cap_piece {
                        Piece::Pawn => "p",
                        Piece::Knight => "n",
                        Piece::Bishop => "b",
                        Piece::Rook => "r",
                        Piece::Queen => "q",
                        Piece::King => "k",
                    }.to_string();
                    if cap_color == Color::White {
                        game.white_pocket.push(piece_str);
                    } else {
                        game.black_pocket.push(piece_str);
                    }
                }
            }

            let move_desc = format!(
                "{}{}{}",
                from_sq_str,
                to_sq_str,
                promo_str.unwrap_or("")
            );
            game.moves.push(move_desc);
            nb
        };

        game.fen = new_board.to_string();

        // Check new board status
        match new_board.status() {
            BoardStatus::Checkmate => {
                game.status = "checkmate".to_string();
                game.winner = Some(match side_to_move {
                    Color::White => "White".to_string(),
                    Color::Black => "Black".to_string(),
                });
            }
            BoardStatus::Stalemate => {
                game.status = "stalemate".to_string();
                game.winner = Some("Draw (Stalemate)".to_string());
            }
            BoardStatus::Ongoing => {}
        }

        Ok(game.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation_and_joining() {
        let manager = GameManager::new();
        let game_id = manager.create_game("standard".to_string());
        
        let game = manager.get_game(&game_id).unwrap();
        assert_eq!(game.status, "waiting");
        assert!(game.white_player.is_none());
        assert!(game.black_player.is_none());

        // Join White
        let player1 = "player_1";
        let game = manager.join_game(&game_id, player1, "white").unwrap();
        assert_eq!(game.white_player.as_deref(), Some(player1));
        assert_eq!(game.status, "waiting"); // Still waiting for Black

        // Joining same role should succeed (idempotent)
        let game = manager.join_game(&game_id, player1, "white").unwrap();
        assert_eq!(game.white_player.as_deref(), Some(player1));

        // Join Black
        let player2 = "player_2";
        let game = manager.join_game(&game_id, player2, "black").unwrap();
        assert_eq!(game.black_player.as_deref(), Some(player2));
        assert_eq!(game.status, "active"); // Both slots filled

        // Try to join when slot is taken
        let player3 = "player_3";
        let res = manager.join_game(&game_id, player3, "white");
        assert!(res.is_err());
    }

    #[test]
    fn test_make_move() {
        let manager = GameManager::new();
        let game_id = manager.create_game("standard".to_string());
        
        let player1 = "player_1";
        let player2 = "player_2";
        
        manager.join_game(&game_id, player1, "white").unwrap();
        manager.join_game(&game_id, player2, "black").unwrap();

        // White move e2 to e4
        let game = manager.make_move(&game_id, player1, "e2", "e4", None).unwrap();
        assert_eq!(game.moves.len(), 1);
        assert_eq!(game.moves[0], "e2e4");

        // Attempting to move out of turn (White moves again)
        let res = manager.make_move(&game_id, player1, "e4", "e5", None);
        assert!(res.is_err());

        // Black moves e7 to e5
        let game = manager.make_move(&game_id, player2, "e7", "e5", None).unwrap();
        assert_eq!(game.moves.len(), 2);
        assert_eq!(game.moves[1], "e7e5");

        // Illegal move (White moves knight through a piece illegally)
        let res = manager.make_move(&game_id, player1, "g1", "g3", None);
        assert!(res.is_err());
    }

    #[test]
    fn test_mad_scientist_drops() {
        let manager = GameManager::new();
        let game_id = manager.create_game("mad_scientist".to_string());
        
        let player1 = "player_1";
        let player2 = "player_2";
        
        manager.join_game(&game_id, player1, "white").unwrap();
        manager.join_game(&game_id, player2, "black").unwrap();

        // 1. Manually add a White pawn to White's pocket and drop it on e4
        {
            let mut games = manager.games.lock().unwrap();
            let game = games.get_mut(&game_id).unwrap();
            game.white_pocket.push("p".to_string());
        }

        // Drop pawn on e4
        let game = manager.make_move(&game_id, player1, "drop:p", "e4", None).unwrap();
        assert_eq!(game.moves.len(), 1);
        assert_eq!(game.moves[0], "P@e4");
        assert_eq!(game.white_pocket.len(), 0);

        // 2. Black moves d7d5
        let _game = manager.make_move(&game_id, player2, "d7", "d5", None).unwrap();
        
        // White e4xd5 captures Black pawn.
        // Under Mad Scientist rules:
        // Black's captured pawn goes to Black's pocket (its owner's pocket)!
        let game = manager.make_move(&game_id, player1, "e4", "d5", None).unwrap();
        assert_eq!(game.black_pocket.len(), 1);
        assert_eq!(game.black_pocket[0], "p");

        // 3. Black drops the pawn back on d4
        let game = manager.make_move(&game_id, player2, "drop:p", "d4", None).unwrap();
        assert_eq!(game.black_pocket.len(), 0);
        assert_eq!(game.moves.len(), 4);
        assert_eq!(game.moves[3], "P@d4");

        // 4. Test dropping pawn on 1st/8th rank fails
        {
            let mut games = manager.games.lock().unwrap();
            let game = games.get_mut(&game_id).unwrap();
            game.white_pocket.push("p".to_string());
        }
        // White tries to drop on e8
        let res = manager.make_move(&game_id, player1, "drop:p", "e8", None);
        assert!(res.is_err()); // Pawn cannot drop on 8th rank
    }
}

