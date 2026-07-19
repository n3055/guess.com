use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{route, Form, IntoResponse},
    view::view,
};
use crate::chess_game::GameManager;
use crate::helpers::get_or_create_player_id;

#[derive(serde::Deserialize)]
struct MatchmakeForm {
    variant: Option<String>,
}

#[route(POST "/matchmake/join")]
async fn join_matchmake(cx: &Cx, Form(form): Form<MatchmakeForm>) -> Result<topcoat::router::Response> {
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);
    let variant = form.variant.unwrap_or_else(|| "standard".to_string());

    if let Some(game_id) = manager.join_matchmaking(&player_id, &variant) {
        let role = if let Some(game) = manager.get_game(&game_id) {
            if game.black_player.as_deref() == Some(&player_id) {
                "black"
            } else {
                "white"
            }
        } else {
            "white"
        };
        return Ok((
            [("HX-Redirect", format!("/game/{}?role={}", game_id, role))],
            ()
        ).into_response(cx)?);
    }

    let view_res = view! { cx =>
        <div
            id="matchmake-status"
            hx-get="/matchmake/status"
            hx-trigger="every 1s"
            hx-swap="outerHTML"
            class="flex flex-col items-center justify-center py-10 px-6 bg-[#21201e] border border-[#31312f] rounded-xl shadow-lg w-full text-center"
        >
            <div class="relative w-16 h-16 mb-4">
                <div class="absolute inset-0 border-4 border-[#769656]/20 border-t-[#81b64c] rounded-full animate-spin"></div>
                <div class="absolute inset-2.5 border-4 border-gray-700/30 border-b-gray-400 rounded-full animate-spin animate-reverse"></div>
            </div>

            <h3 class="text-lg font-bold text-white mb-1">"Finding Opponent..."</h3>
            <p class="text-gray-400 text-xs mb-4 leading-relaxed">
                "Searching for a match. The game will start automatically when paired."
            </p>

            <button
                hx-post="/matchmake/cancel"
                hx-target="#matchmake-status"
                hx-swap="outerHTML"
                class="bg-[#363532] hover:bg-[#454440] text-gray-300 font-bold py-2 px-5 rounded-lg text-sm transition"
            >
                "Cancel Search"
            </button>
        </div>
    };

    Ok(view_res.into_response(cx)?)
}

#[route(GET "/matchmake/status")]
async fn matchmake_status(cx: &Cx) -> Result<topcoat::router::Response> {
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);

    if let Some(game_id) = manager.check_matchmaking(&player_id) {
        let role = if let Some(game) = manager.get_game(&game_id) {
            if game.black_player.as_deref() == Some(&player_id) {
                "black"
            } else {
                "white"
            }
        } else {
            "white"
        };
        return Ok((
            [("HX-Redirect", format!("/game/{}?role={}", game_id, role))],
            ()
        ).into_response(cx)?);
    }

    let view_res = view! { cx =>
        <div
            id="matchmake-status"
            hx-get="/matchmake/status"
            hx-trigger="every 1s"
            hx-swap="outerHTML"
            class="flex flex-col items-center justify-center py-10 px-6 bg-[#21201e] border border-[#31312f] rounded-xl shadow-lg w-full text-center"
        >
            <div class="relative w-16 h-16 mb-4">
                <div class="absolute inset-0 border-4 border-[#769656]/20 border-t-[#81b64c] rounded-full animate-spin"></div>
                <div class="absolute inset-2.5 border-4 border-gray-700/30 border-b-gray-400 rounded-full animate-spin animate-reverse"></div>
            </div>

            <h3 class="text-lg font-bold text-white mb-1">"Finding Opponent..."</h3>
            <p class="text-gray-400 text-xs mb-4 leading-relaxed">
                "Searching for a match. The game will start automatically when paired."
            </p>

            <button
                hx-post="/matchmake/cancel"
                hx-target="#matchmake-status"
                hx-swap="outerHTML"
                class="bg-[#363532] hover:bg-[#454440] text-gray-300 font-bold py-2 px-5 rounded-lg text-sm transition"
            >
                "Cancel Search"
            </button>
        </div>
    };

    Ok(view_res.into_response(cx)?)
}

#[route(POST "/matchmake/cancel")]
async fn cancel_matchmake(cx: &Cx) -> Result<topcoat::router::Response> {
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);

    manager.cancel_matchmaking(&player_id);

    let view_res = view! { cx =>
        <div id="matchmake-status" class="w-full">
            <button
                hx-post="/matchmake/join"
                hx-target="#matchmake-status"
                hx-swap="outerHTML"
                class="w-full bg-[#81b64c] hover:bg-[#95c95d] text-white text-xl font-bold py-4 px-6 rounded-lg shadow-lg transition transform hover:scale-[1.01] text-center"
            >
                "Play / Find Opponent"
            </button>
        </div>
    };

    Ok(view_res.into_response(cx)?)
}
