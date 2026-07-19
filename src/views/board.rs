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
        <div class="flex items-center justify-between bg-[#1e1e1c] border border-[#31312f] rounded-lg px-3 py-1.5 my-1 shadow-md w-full">
            <div class="flex items-center space-x-3">
                <div class="flex flex-col">
                    if let Some(pid) = player_id_opt {
                        <span class="font-bold text-white text-sm">
                            (role_label) " (" (&pid[..8]) ")"
                        </span>
                    } else {
                        <span class="font-bold text-gray-500 text-sm italic">
                            (role_label) " (Open)"
                        </span>
                    }

                    <div class="flex items-center space-x-0.5 mt-0.5 min-h-[1rem]">
                        for _ in 0..captured_pieces.pawns {
                            <img src=(p_url) class="w-4 h-4 object-contain" />
                        }
                        for _ in 0..captured_pieces.knights {
                            <img src=(n_url) class="w-4 h-4 object-contain" />
                        }
                        for _ in 0..captured_pieces.bishops {
                            <img src=(b_url) class="w-4 h-4 object-contain" />
                        }
                        for _ in 0..captured_pieces.rooks {
                            <img src=(r_url) class="w-4 h-4 object-contain" />
                        }
                        for _ in 0..captured_pieces.queens {
                            <img src=(q_url) class="w-4 h-4 object-contain" />
                        }
                    </div>

                    if !pocket_items.is_empty() {
                        <div class="flex items-center gap-1.5 mt-1 pt-1 border-t border-[#2d2d2c] w-full">
                            <span class="text-[9px] font-extrabold text-yellow-500 tracking-wider">"POCKET:"</span>
                            for (piece_name, url, count) in pocket_items {
                                <div
                                    data-pocket-piece=(piece_name)
                                    data-pocket-color=(role_to_join)
                                    class="pocket-piece relative flex items-center justify-center bg-[#2d2d2a] hover:bg-[#3d3d3a] border border-[#4d4d4a] rounded p-0.5 w-7 h-7 cursor-pointer select-none transition-all shadow"
                                >
                                    <img
                                        src=(url)
                                        draggable="true"
                                        class="w-6 h-6 object-contain cursor-grab active:cursor-grabbing select-none"
                                    />
                                    if count > 1 {
                                        <span class="absolute -bottom-1 -right-1 bg-yellow-600 text-white font-extrabold text-[8px] px-1 rounded-full pointer-events-none leading-none">
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
                    <button type="submit" class="bg-[#769656] hover:bg-[#85a665] text-white font-bold py-1 px-3 rounded text-xs transition">
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
        <div class="flex flex-col lg:flex-row gap-2 lg:gap-8 items-center lg:items-start w-full h-full min-h-0 justify-between max-w-7xl mx-auto overflow-hidden">
            <div class="w-full lg:flex-1 flex flex-col justify-between h-full min-h-0 max-w-[760px] mx-auto lg:mx-0 py-0.5 sm:py-1 overflow-hidden">
                (top_panel)

                <div
                    id="chess-board"
                    class="grid grid-cols-8 w-full max-w-[min(100%,calc(100dvh-160px))] lg:max-w-[min(100%,calc(100vh-140px),720px)] aspect-square border-4 sm:border-8 border-[#31312f] rounded-xl overflow-hidden relative shadow-[0_15px_50px_rgba(0,0,0,0.85)] select-none bg-[#769656] mx-auto transition-all shrink-0"
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
                                class=(format!("chess-square relative aspect-square flex items-center justify-center {}{} select-none cursor-pointer p-0.5 sm:p-1", sq_color_class, last_move_class))
                            >
                                if let Some(piece) = piece_opt {
                                    if let Some(color) = color_opt {
                                        let img_url = get_piece_image_url(piece, color);
                                        <img
                                            src=(img_url)
                                            draggable="true"
                                            class="w-[96%] h-[96%] object-contain select-none cursor-grab active:cursor-grabbing z-10 hover:scale-105 transition-transform filter drop-shadow-md"
                                        />
                                    }
                                }

                                if f_idx == 0 {
                                    <span class=(format!("absolute top-0.5 left-1 text-[9px] sm:text-xs font-black leading-none select-none pointer-events-none {}", coord_color_class))>
                                        (format!("{}", rank.to_index() + 1))
                                    </span>
                                }
                                if r_idx == 7 {
                                    <span class=(format!("absolute bottom-0.5 right-1 text-[9px] sm:text-xs font-black leading-none select-none pointer-events-none {}", coord_color_class))>
                                        (file_char.to_string())
                                    </span>
                                }
                            </div>
                        }
                    }
                </div>

                (bottom_panel)
            </div>

            <div class="hidden lg:flex w-80 xl:w-96 bg-[#1b1a18] border border-[#31312f] rounded-2xl p-5 flex-col h-full shadow-2xl shrink-0 gap-4 overflow-hidden">
                <div class=(format!("border rounded-xl p-4 font-bold flex flex-col gap-2 shadow-inner transition-all {}", status_bg))>
                    <div class="flex items-center justify-between">
                        <span class="text-xs uppercase tracking-wider font-extrabold text-gray-400">"Game Status"</span>
                        if game.status == "active" {
                            <span class="flex items-center gap-1.5 text-xs uppercase tracking-wider bg-[#262522] px-2.5 py-0.5 rounded-full text-[#81b64c] font-bold border border-[#769656]/40">
                                <span class="w-2 h-2 rounded-full bg-[#81b64c] animate-pulse"></span>
                                "Live"
                            </span>
                        }
                    </div>
                    <div class="text-lg font-black tracking-tight leading-snug">
                        (status_text.clone())
                    </div>
                    <div class="flex items-center justify-between border-t border-white/10 pt-2 text-xs">
                        <span class="text-gray-400">"Mode:"</span>
                        if game.variant == "mad_scientist" {
                            <span class="font-bold text-yellow-400 bg-yellow-950/60 px-2 py-0.5 rounded border border-yellow-600/40">
                                "🧪 Mad Scientist"
                            </span>
                        } else {
                            <span class="font-bold text-gray-300 bg-[#262522] px-2 py-0.5 rounded border border-gray-600/30">
                                "♟ Standard Chess"
                            </span>
                        }
                    </div>
                </div>

                <div class="flex items-center justify-between border-b border-[#31312f] pb-2">
                    <h2 class="text-base font-extrabold text-white uppercase tracking-wider">"Move History"</h2>
                    <span class="text-xs text-gray-500 font-mono">(format!("{} moves", game.moves.len()))</span>
                </div>

                if game.status == "waiting" {
                    <button
                        onclick="navigator.clipboard.writeText(window.location.href); alert('Game link copied to clipboard!');"
                        class="w-full bg-[#769656] hover:bg-[#85a665] text-white font-extrabold py-3 px-4 rounded-xl text-sm transition flex items-center justify-center gap-2 cursor-pointer shadow-lg hover:shadow-[#769656]/20 shrink-0"
                    >
                        <span>"🔗"</span>
                        "Copy Invite Link"
                    </button>
                }

                <div class="flex-1 overflow-y-auto min-h-0 font-mono text-sm space-y-1 pr-1 border border-[#2a2926] rounded-xl p-3 bg-[#141311]">
                    if game.moves.is_empty() {
                        <div class="text-gray-500 text-center py-12 italic text-xs">
                            "No moves played yet. Ready to start!"
                        </div>
                    } else {
                        for i in (0..game.moves.len()).step_by(2) {
                            <div class="flex py-1.5 px-2 rounded hover:bg-[#262522] border-b border-[#31312f]/30 items-center">
                                <span class="w-10 text-gray-500 font-bold text-xs">
                                    (format!("{}.", i / 2 + 1))
                                </span>
                                <span class="flex-1 text-[#81b64c] font-bold">
                                    (game.moves[i].clone())
                                </span>
                                if i + 1 < game.moves.len() {
                                    <span class="flex-1 text-gray-300 font-medium">
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

                <div class="border-t border-[#31312f] pt-3 mt-auto space-y-3 shrink-0">
                    <div class="flex items-center justify-between">
                        <span class="text-[11px] font-bold text-gray-500 uppercase tracking-wider">"GAME ID"</span>
                        <span class="text-xs font-mono text-[#769656] select-all bg-[#262522] px-2 py-0.5 rounded border border-[#31312f]">
                            (game.id.clone())
                        </span>
                    </div>

                    <a
                        href="/"
                        class="w-full text-center bg-[#262522] hover:bg-[#363532] text-gray-200 hover:text-white font-extrabold py-3 px-4 rounded-xl text-sm transition flex items-center justify-center gap-2 border border-[#31312f] shadow"
                    >
                        <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                        </svg>
                        "Back to Lobby"
                    </a>
                </div>
            </div>

            <div id="mobile-details-drawer" class="fixed inset-y-0 right-0 z-50 w-80 bg-[#1b1a18] border-l border-[#31312f] p-5 flex flex-col justify-between transform translate-x-full transition-transform duration-300 ease-in-out lg:hidden shadow-2xl overflow-y-auto">
                <div class="flex items-center justify-between pb-4 border-b border-[#31312f]">
                    <h2 class="text-lg font-black text-white">"Game Details"</h2>
                    <button id="mobile-details-close" type="button" class="p-1 rounded-lg text-gray-400 hover:text-white hover:bg-[#262522]">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>

                <div class="my-4 font-bold flex flex-col gap-2">
                    <div class=(format!("border rounded-xl p-4 font-bold flex flex-col gap-2 shadow-inner transition-all {}", status_bg))>
                        <div class="flex items-center justify-between">
                            <span class="text-xs uppercase tracking-wider font-extrabold text-gray-400">"Status"</span>
                            if game.status == "active" {
                                <span class="flex items-center gap-1.5 text-xs uppercase tracking-wider bg-[#262522] px-2.5 py-0.5 rounded-full text-[#81b64c] font-bold border border-[#769656]/40">
                                    <span class="w-2 h-2 rounded-full bg-[#81b64c] animate-pulse"></span>
                                    "Live"
                                </span>
                            }
                        </div>
                        <div class="text-base font-black tracking-tight leading-snug">
                            (status_text)
                        </div>
                    </div>
                </div>

                <div class="flex items-center justify-between border-b border-[#31312f] pb-2 mb-2">
                    <h3 class="text-xs font-extrabold text-gray-400 uppercase tracking-wider">"Move History"</h3>
                    <span class="text-xs text-gray-500 font-mono">(format!("{} moves", game.moves.len()))</span>
                </div>

                if game.status == "waiting" {
                    <button
                        onclick="navigator.clipboard.writeText(window.location.href); alert('Game link copied to clipboard!');"
                        class="w-full bg-[#769656] hover:bg-[#85a665] text-white font-extrabold py-2.5 px-4 rounded-xl text-sm transition flex items-center justify-center gap-2 mb-3 cursor-pointer shadow-lg"
                    >
                        <span>"🔗"</span>
                        "Copy Invite Link"
                    </button>
                }

                <div class="flex-1 overflow-y-auto min-h-[150px] font-mono text-sm space-y-1 pr-1 border border-[#2a2926] rounded-xl p-3 bg-[#141311] mb-4">
                    if game.moves.is_empty() {
                        <div class="text-gray-500 text-center py-8 italic text-xs">
                            "No moves played yet."
                        </div>
                    } else {
                        for i in (0..game.moves.len()).step_by(2) {
                            <div class="flex py-1.5 px-2 rounded hover:bg-[#262522] border-b border-[#31312f]/30 items-center">
                                <span class="w-10 text-gray-500 font-bold text-xs">
                                    (format!("{}.", i / 2 + 1))
                                </span>
                                <span class="flex-1 text-[#81b64c] font-bold">
                                    (game.moves[i].clone())
                                </span>
                                if i + 1 < game.moves.len() {
                                    <span class="flex-1 text-gray-300 font-medium">
                                        (game.moves[i + 1].clone())
                                    </span>
                                } else {
                                    <span class="flex-1"></span>
                                }
                            </div>
                        }
                    }
                </div>

                <div class="border-t border-[#31312f] pt-3 mt-auto space-y-3">
                    <div class="flex items-center justify-between">
                        <span class="text-[11px] font-bold text-gray-500 uppercase tracking-wider">"GAME ID"</span>
                        <span class="text-xs font-mono text-[#769656] select-all bg-[#262522] px-2 py-0.5 rounded border border-[#31312f]">
                            (game.id.clone())
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
