use std::str::FromStr;
use chess::{Board, Square, Color, Piece};
use topcoat::{
    Result,
    context::Cx,
    view::view,
};
use crate::chess_game::ChessGame;
use super::pieces::{
    CapturedPieces, get_captured_pieces, get_piece_image_url,
    RANKS_WHITE, FILES_WHITE, RANKS_BLACK, FILES_BLACK,
};

/// Render a player info bar (avatar, captured pieces, pocket, join button).
pub async fn render_player_line(
    cx: &Cx,
    role_label: &str,
    player_id_opt: Option<&str>,
    captured_pieces: &CapturedPieces,
    is_white_pieces: bool,
    game_id: &str,
    role_to_join: &str,
    pocket: &[String],
) -> Result {
    let p_color = if is_white_pieces { Color::White } else { Color::Black };
    let p_url = get_piece_image_url(Piece::Pawn, p_color);
    let n_url = get_piece_image_url(Piece::Knight, p_color);
    let b_url = get_piece_image_url(Piece::Bishop, p_color);
    let r_url = get_piece_image_url(Piece::Rook, p_color);
    let q_url = get_piece_image_url(Piece::Queen, p_color);

    let avatar_color = if role_to_join == "white" {
        "bg-[#f9f9f7]"
    } else {
        "bg-[#1b1a17]"
    };

    let avatar_text = if role_to_join == "white" {
        "text-[#1b1a17]"
    } else {
        "text-[#f9f9f7]"
    };

    let mut pawns_count = 0;
    let mut knights_count = 0;
    let mut bishops_count = 0;
    let mut rooks_count = 0;
    let mut queens_count = 0;
    for piece in pocket {
        match piece.as_str() {
            "p" | "pawn" => pawns_count += 1,
            "n" | "knight" => knights_count += 1,
            "b" | "bishop" => bishops_count += 1,
            "r" | "rook" => rooks_count += 1,
            "q" | "queen" => queens_count += 1,
            _ => {}
        }
    }

    let mut pocket_items = Vec::new();
    if pawns_count > 0 { pocket_items.push(("p".to_string(), p_url, pawns_count)); }
    if knights_count > 0 { pocket_items.push(("n".to_string(), n_url, knights_count)); }
    if bishops_count > 0 { pocket_items.push(("b".to_string(), b_url, bishops_count)); }
    if rooks_count > 0 { pocket_items.push(("r".to_string(), r_url, rooks_count)); }
    if queens_count > 0 { pocket_items.push(("q".to_string(), q_url, queens_count)); }

    view! { cx =>
        <div class="flex items-center justify-between bg-[#1e1e1c] border border-[#31312f] rounded-lg p-3 my-2 shadow-md">
            <div class="flex items-center space-x-3">
                <div class=(format!("w-10 h-10 rounded-full flex items-center justify-center font-bold text-lg {} {}", avatar_color, avatar_text))>
                    (if role_to_join == "white" { "W" } else { "B" })
                </div>

                <div class="flex flex-col">
                    if let Some(pid) = player_id_opt {
                        <span class="font-bold text-white text-base">
                            (role_label) " (" (&pid[..8]) ")"
                        </span>
                    } else {
                        <span class="font-bold text-gray-500 text-base italic">
                            (role_label) " (Open)"
                        </span>
                    }

                    <div class="flex items-center space-x-0.5 mt-0.5 min-h-[1.25rem]">
                        for _ in 0..captured_pieces.pawns {
                            <img src=(p_url) class="w-5 h-5 object-contain" />
                        }
                        for _ in 0..captured_pieces.knights {
                            <img src=(n_url) class="w-5 h-5 object-contain" />
                        }
                        for _ in 0..captured_pieces.bishops {
                            <img src=(b_url) class="w-5 h-5 object-contain" />
                        }
                        for _ in 0..captured_pieces.rooks {
                            <img src=(r_url) class="w-5 h-5 object-contain" />
                        }
                        for _ in 0..captured_pieces.queens {
                            <img src=(q_url) class="w-5 h-5 object-contain" />
                        }
                    </div>

                    if !pocket_items.is_empty() {
                        <div class="flex items-center gap-2 mt-2 pt-2 border-t border-[#2d2d2c] w-full">
                            <span class="text-[10px] font-bold text-yellow-500 tracking-wider">"POCKET:"</span>
                            for (piece_name, url, count) in pocket_items {
                                <div
                                    data-pocket-piece=(piece_name)
                                    data-pocket-color=(role_to_join)
                                    class="pocket-piece relative flex items-center justify-center bg-[#2d2d2a] hover:bg-[#3d3d3a] border border-[#4d4d4a] rounded p-1 w-9 h-9 cursor-pointer select-none transition-all shadow"
                                >
                                    <img
                                        src=(url)
                                        draggable="true"
                                        class="w-7 h-7 object-contain cursor-grab active:cursor-grabbing select-none"
                                    />
                                    if count > 1 {
                                        <span class="absolute -bottom-1 -right-1 bg-yellow-600 text-white font-extrabold text-[9px] px-1 rounded-full pointer-events-none leading-none">
                                            (format!("x{}", count))
                                        </span>
                                    }
                                </div>
                            }
                        </div>
                    }
                </div>
            </div>

            if player_id_opt.is_none() {
                <form action=(format!("/game/{}/join", game_id)) method="POST" class="m-0">
                    <input type="hidden" name="role" value=(role_to_join) />
                    <button type="submit" class="bg-[#769656] hover:bg-[#85a665] text-white font-bold py-1.5 px-4 rounded text-sm transition">
                        "Join"
                    </button>
                </form>
            }
        </div>
    }
}

/// Render the full inner game view: status bar, player panels, chess board, and sidebar.
pub async fn render_inner_game_view(cx: &Cx, game: &ChessGame, player_id: &str, role_query: Option<&str>) -> Result {
    let board = Board::from_str(&game.fen).unwrap_or_default();

    let captured_white = get_captured_pieces(&board, Color::White);
    let captured_black = get_captured_pieces(&board, Color::Black);

    let is_black_perspective = if let Some(role) = role_query {
        role == "black"
    } else {
        game.black_player.as_deref() == Some(player_id)
    };

    let player_role = if let Some(role) = role_query {
        role
    } else if game.white_player.as_deref() == Some(player_id) {
        "white"
    } else if game.black_player.as_deref() == Some(player_id) {
        "black"
    } else {
        "spectator"
    };

    let is_my_turn = match board.side_to_move() {
        Color::White => player_role == "white",
        Color::Black => player_role == "black",
    };

    let last_move = game.moves.last().map(|s| s.as_str());

    let ranks = if is_black_perspective {
        &RANKS_BLACK[..]
    } else {
        &RANKS_WHITE[..]
    };
    let files = if is_black_perspective {
        &FILES_BLACK[..]
    } else {
        &FILES_WHITE[..]
    };

    let status_bg = match game.status.as_str() {
        "waiting" => "bg-[#363532] text-yellow-400 border-yellow-500/30",
        "active" => "bg-[#1e1e1c] text-[#81b64c] border-[#769656]/30",
        _ => "bg-[#1e1e1c] text-red-400 border-red-500/30",
    };

    let turn_str = match board.side_to_move() {
        Color::White => "White",
        Color::Black => "Black",
    };

    let status_text = match game.status.as_str() {
        "waiting" => "Waiting for players to join... Send the URL to invite a friend.".to_string(),
        "active" => {
            if is_my_turn {
                format!("Your turn! Play as {}", turn_str)
            } else {
                format!("{}'s turn to move...", turn_str)
            }
        }
        "checkmate" => {
            let winner_name = game.winner.as_deref().unwrap_or("Unknown");
            format!("Checkmate! Winner: {}", winner_name)
        }
        "stalemate" => "Draw (Stalemate)".to_string(),
        _ => format!("Game Over: {}", game.status),
    };

    let top_panel = if is_black_perspective {
        render_player_line(
            cx,
            "White",
            game.white_player.as_deref(),
            &captured_black,
            false,
            &game.id,
            "white",
            &game.white_pocket,
        ).await?
    } else {
        render_player_line(
            cx,
            "Black",
            game.black_player.as_deref(),
            &captured_white,
            true,
            &game.id,
            "black",
            &game.black_pocket,
        ).await?
    };

    let bottom_panel = if is_black_perspective {
        render_player_line(
            cx,
            "Black",
            game.black_player.as_deref(),
            &captured_white,
            true,
            &game.id,
            "black",
            &game.black_pocket,
        ).await?
    } else {
        render_player_line(
            cx,
            "White",
            game.white_player.as_deref(),
            &captured_black,
            false,
            &game.id,
            "white",
            &game.white_pocket,
        ).await?
    };

    view! { cx =>
        <div class="flex flex-col md:flex-row gap-8 items-start w-full min-h-0">
            <div class="w-full md:w-auto flex-1 flex flex-col min-h-0 max-w-[640px] mx-auto md:mx-0">
                <div class=(format!("border rounded-lg p-3 mb-2 font-bold flex items-center justify-between text-sm {}", status_bg))>
                    <div class="flex items-center gap-2">
                        <span class="w-2.5 h-2.5 rounded-full bg-current animate-pulse"></span>
                        <span>(status_text)</span>
                    </div>
                    <div class="flex items-center gap-2">
                        if game.variant == "mad_scientist" {
                            <div class="relative group flex items-center gap-1 cursor-pointer">
                                <div class="text-[10px] uppercase font-extrabold tracking-wider bg-yellow-600 px-2 py-0.5 rounded text-white shadow shadow-yellow-500/20 animate-pulse">
                                    "Mad Scientist Mode"
                                </div>
                                <svg class="w-4 h-4 text-yellow-500 hover:text-yellow-400 transition" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                <div class="absolute bottom-full right-0 mb-2 w-72 p-3 bg-[#1e1e1c] text-xs text-gray-200 border border-yellow-500/30 rounded-lg shadow-2xl opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity duration-200 z-50 leading-relaxed normal-case font-normal">
                                    <span class="font-bold text-yellow-500 block mb-1">"🧪 Mad Scientist Chess Rules:"</span>
                                    "1. Captured pieces go into the OWNER's pocket (keep their color)."
                                    <br/>
                                    "2. On your turn, you can drop a piece from your pocket onto any empty square (pawns cannot drop on 1st/8th ranks)."
                                    <br/>
                                    "3. Drop moves are drag-and-drop or select-and-click!"
                                </div>
                            </div>
                        } else {
                            <div class="relative group flex items-center gap-1 cursor-pointer">
                                <div class="text-xs uppercase tracking-wider bg-[#262522] px-2 py-0.5 rounded text-gray-400">
                                    "Standard Chess"
                                </div>
                                <svg class="w-4 h-4 text-gray-400 hover:text-gray-200 transition" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                <div class="absolute bottom-full right-0 mb-2 w-72 p-3 bg-[#1e1e1c] text-xs text-gray-200 border border-gray-600/30 rounded-lg shadow-2xl opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity duration-200 z-50 leading-relaxed normal-case font-normal">
                                    <span class="font-bold text-white block mb-1">"♟ Standard Chess Rules:"</span>
                                    "Standard chess. Protect your king, capture opponent pieces, block checks, and checkmate the opponent king to win!"
                                </div>
                            </div>
                        }
                        if game.status == "active" {
                            <div class="text-xs uppercase tracking-wider bg-[#262522] px-2 py-0.5 rounded text-gray-400">
                                "Live"
                            </div>
                        }
                    </div>
                </div>

                (top_panel)

                <div
                    id="chess-board"
                    class="grid grid-cols-8 w-full aspect-square border-4 border-[#31312f] rounded-lg overflow-hidden relative shadow-2xl select-none bg-[#769656]"
                    data-game-id=(game.id.clone())
                    data-my-role=(player_role)
                    data-my-turn=(if is_my_turn { "true" } else { "false" })
                >
                    for (r_idx, &rank) in ranks.iter().enumerate() {
                        for (f_idx, &file) in files.iter().enumerate() {
                            let square = Square::make_square(rank, file);
                            let sq_name = square.to_string();
                            let is_light = (r_idx + f_idx) % 2 == 0;
                            let sq_color_class = if is_light {
                                "bg-[#eeeed2]"
                            } else {
                                "bg-[#769656]"
                            };

                            let is_last_move = if let Some(lm) = last_move {
                                lm.starts_with(&sq_name) || lm[2..4].starts_with(&sq_name)
                            } else {
                                false
                            };

                            let last_move_class = if is_last_move { " last-move-sq" } else { "" };

                            let piece_opt = board.piece_on(square);
                            let color_opt = board.color_on(square);

                            let coord_color_class = if is_light {
                                "text-[#769656]"
                            } else {
                                "text-[#eeeed2]"
                            };

                            let file_char = (b'a' + file.to_index() as u8) as char;

                            <div
                                data-square=(sq_name.clone())
                                data-piece-color=(color_opt.map(|c| if c == Color::White { "white" } else { "black" }).unwrap_or("none"))
                                data-piece-type=(piece_opt.map(|p| match p { Piece::Pawn => "pawn", _ => "other" }).unwrap_or("none"))
                                class=(format!("chess-square relative aspect-square flex items-center justify-center {}{} select-none cursor-pointer", sq_color_class, last_move_class))
                            >
                                if let Some(piece) = piece_opt {
                                    if let Some(color) = color_opt {
                                        let img_url = get_piece_image_url(piece, color);
                                        <img
                                            src=(img_url)
                                            draggable="true"
                                            class="w-[92%] h-[92%] object-contain select-none cursor-grab active:cursor-grabbing z-10 hover:scale-105 transition-transform"
                                        />
                                    }
                                }

                                if f_idx == 0 {
                                    <span class=(format!("absolute top-0.5 left-1 text-[9px] md:text-xs font-bold leading-none select-none pointer-events-none {}", coord_color_class))>
                                        (format!("{}", rank.to_index() + 1))
                                    </span>
                                }
                                if r_idx == 7 {
                                    <span class=(format!("absolute bottom-0.5 right-1 text-[9px] md:text-xs font-bold leading-none select-none pointer-events-none {}", coord_color_class))>
                                        (file_char.to_string())
                                    </span>
                                }
                            </div>
                        }
                    }
                </div>

                (bottom_panel)
            </div>

            <div class="w-full md:w-80 bg-[#1e1e1c] border border-[#31312f] rounded-xl p-6 flex flex-col h-[550px] shadow-lg">
                <h2 class="text-xl font-bold text-white mb-4 border-b border-[#31312f] pb-3">"Game Details"</h2>

                if game.status == "waiting" {
                    <button
                        onclick="navigator.clipboard.writeText(window.location.href); alert('Game link copied to clipboard!');"
                        class="w-full bg-[#769656] hover:bg-[#85a665] text-white font-bold py-2.5 px-4 rounded-lg text-sm transition flex items-center justify-center gap-2 mb-4 cursor-pointer"
                    >
                        <span>"🔗"</span>
                        "Copy Invite Link"
                    </button>
                }

                <div class="flex-1 overflow-y-auto mb-4 font-mono text-sm space-y-1 pr-2">
                    if game.moves.is_empty() {
                        <div class="text-gray-500 text-center py-12 italic">
                            "No moves made yet."
                        </div>
                    } else {
                        for i in (0..game.moves.len()).step_by(2) {
                            <div class="flex py-1.5 px-2 rounded hover:bg-[#262522] border-b border-[#31312f]/30">
                                <span class="w-10 text-gray-500 font-semibold">
                                    (format!("{}.", i / 2 + 1))
                                </span>
                                <span class="flex-1 text-[#81b64c] font-bold">
                                    (game.moves[i].clone())
                                </span>
                                if i + 1 < game.moves.len() {
                                    <span class="flex-1 text-gray-300">
                                        (game.moves[i + 1].clone())
                                    </span>
                                } else {
                                    <span class="flex-1"></span>
                                }
                            </div>
                        }
                    }
                </div>

                <form id="move-form" hx-post=(format!("/game/{}/move", game.id)) hx-target="#game-container" hx-swap="innerHTML" class="hidden">
                    <input type="hidden" name="from_sq" id="move-from" />
                    <input type="hidden" name="to_sq" id="move-to" />
                    <input type="hidden" name="promo" id="move-promo" />
                </form>

                <div class="border-t border-[#31312f] pt-4 mt-auto">
                    <div class="text-xs text-gray-500 mb-2">"GAME ID:"</div>
                    <div class="bg-[#262522] px-3 py-2 rounded text-[#769656] font-mono text-xs select-all truncate border border-[#31312f] flex justify-between items-center mb-3">
                        (game.id.clone())
                    </div>
                    <div class="flex gap-2">
                        <a
                            href="/"
                            class="w-full text-center bg-[#363532] hover:bg-[#454440] text-gray-300 font-bold py-2.5 px-4 rounded-lg text-sm transition"
                        >
                            "Exit to Lobby"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
