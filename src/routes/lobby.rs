use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{page, route, see_other, SeeOther, Form},
    view::view,
};
use crate::chess_game::{GameManager, ChessGame};
use crate::helpers::get_or_create_player_id;

#[derive(serde::Deserialize)]
struct CreateGameForm {
    variant: Option<String>,
}

#[route(POST "/game/create")]
async fn create_game(cx: &Cx, Form(form): Form<CreateGameForm>) -> Result<SeeOther> {
    let manager = app_context::<GameManager>(cx);
    let variant = form.variant.unwrap_or_else(|| "standard".to_string());
    let game_id = manager.create_game(variant);

    // Auto-join the creator as White
    let player_id = get_or_create_player_id(cx);
    let _ = manager.join_game(&game_id, &player_id, "white");

    Ok(see_other(&format!("/game/{}?role=white", game_id)))
}

#[derive(serde::Deserialize)]
struct JoinLobbyForm {
    game_id: String,
}

#[route(POST "/game/join")]
async fn join_lobby(_cx: &Cx, Form(form): Form<JoinLobbyForm>) -> Result<SeeOther> {
    Ok(see_other(&format!("/game/{}", form.game_id.trim())))
}

#[page("/")]
async fn lobby(cx: &Cx) -> Result {
    let manager = app_context::<GameManager>(cx);
    let games = {
        let games_lock = manager.games.lock().unwrap();
        games_lock.values().cloned().collect::<Vec<ChessGame>>()
    };

    view! { cx =>
        <div class="p-4 md:p-8 max-w-5xl mx-auto w-full flex-1 flex flex-col justify-center">
            <div class="mb-8 text-center md:text-left flex flex-col md:flex-row items-center justify-between gap-4 border-b border-[#31312f] pb-6">
                <div>
                    <h1 class="text-4xl font-extrabold text-white flex items-center justify-center md:justify-start gap-2">
                        <span class="text-[#769656]">"guess"</span>
                        <span>"chess"</span>
                    </h1>
                    <p class="text-gray-400 mt-1">"Play Chess online with random opponents or invite friends."</p>
                </div>
                <div class="flex items-center gap-2 bg-[#21201e] px-4 py-2 border border-[#31312f] rounded-lg text-sm font-semibold">
                    <span class="w-2.5 h-2.5 rounded-full bg-green-500 animate-pulse"></span>
                    <span class="text-gray-300">"Server Online"</span>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start mb-8">
                <div class="lg:col-span-2 space-y-6">
                    <div class="bg-[#21201e] border border-[#31312f] rounded-xl p-6 md:p-8 shadow-xl flex flex-col justify-between min-h-[300px]">
                        <div class="mb-6">
                            <h2 class="text-2xl font-bold text-white mb-2 flex items-center gap-2">
                                <span class="text-lg">"⚡"</span>
                                <span>"Quick Matchmaking"</span>
                            </h2>
                            <p class="text-gray-400 text-sm">
                                "Find a random opponent instantly. You will be automatically paired with anyone else searching."
                            </p>
                        </div>

                        <div id="matchmake-status" class="w-full flex flex-col sm:flex-row gap-4">
                            <button
                                hx-post="/matchmake/join"
                                hx-vals=("{\"variant\": \"standard\"}")
                                hx-target="#matchmake-status"
                                hx-swap="outerHTML"
                                class="flex-1 bg-[#81b64c] hover:bg-[#95c95d] text-white text-lg font-bold py-4 px-6 rounded-lg shadow-lg transition transform hover:scale-[1.01] text-center flex items-center justify-center gap-2"
                            >
                                <span class="text-xl">"♟"</span>
                                "Play Standard Match"
                            </button>
                            <button
                                hx-post="/matchmake/join"
                                hx-vals=("{\"variant\": \"mad_scientist\"}")
                                hx-target="#matchmake-status"
                                hx-swap="outerHTML"
                                class="flex-1 bg-yellow-600 hover:bg-yellow-500 text-white text-lg font-bold py-4 px-6 rounded-lg shadow-lg transition transform hover:scale-[1.01] text-center flex items-center justify-center gap-2"
                            >
                                <span class="text-xl">"🧪"</span>
                                "Play Mad Scientist"
                            </button>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="bg-[#21201e] border border-[#31312f] rounded-xl p-6 shadow-lg flex flex-col justify-between min-h-[220px]">
                            <div>
                                <h3 class="text-xl font-bold text-white mb-2 flex items-center gap-2">
                                    <span>"👥"</span>
                                    <span>"Play with a Friend"</span>
                                </h3>
                                <p class="text-gray-400 text-xs mb-4">
                                    "Create a custom lobby and share the game URL link with a friend to play together."
                                </p>
                            </div>
                            <form action="/game/create" method="POST" class="m-0 space-y-3">
                                <div>
                                    <label class="block text-xs font-bold text-gray-400 mb-1">"GAME VARIANT"</label>
                                    <select name="variant" class="w-full bg-[#1e1e1c] border border-[#31312f] rounded p-2 text-sm text-white focus:outline-none">
                                        <option value="standard">"Standard Chess"</option>
                                        <option value="mad_scientist">"Mad Scientist Chess"</option>
                                    </select>
                                </div>
                                <button type="submit" class="w-full bg-[#363532] hover:bg-[#454440] text-white font-bold py-3 px-4 rounded-lg shadow transition text-center">
                                    "Create Custom Game"
                                </button>
                            </form>
                        </div>

                        <div class="bg-[#21201e] border border-[#31312f] rounded-xl p-6 shadow-lg flex flex-col justify-between min-h-[220px]">
                            <div>
                                <h3 class="text-xl font-bold text-white mb-2 flex items-center gap-2">
                                    <span>"🔑"</span>
                                    <span>"Enter Game Code"</span>
                                </h3>
                                <p class="text-gray-400 text-xs mb-4">
                                    "Have a custom invite code? Enter the game ID below to join the match."
                                </p>
                            </div>
                            <form action="/game/join" method="POST" class="space-y-3 m-0">
                                <input
                                    type="text"
                                    name="game_id"
                                    placeholder="Game Code / ID"
                                    required="true"
                                    class="w-full bg-[#1a1917] border border-[#31312f] rounded-lg py-2.5 px-4 text-sm text-white focus:outline-none focus:border-[#769656]"
                                />
                                <button type="submit" class="w-full bg-[#2a2926] hover:bg-[#363532] text-gray-300 font-bold py-2.5 px-4 rounded-lg transition text-center text-sm border border-[#31312f]">
                                    "Join Game"
                                </button>
                            </form>
                        </div>
                    </div>
                </div>

                <div class="bg-[#21201e] border border-[#31312f] rounded-xl p-6 shadow-xl lg:h-[540px] flex flex-col">
                    <h2 class="text-xl font-bold text-white mb-4 border-b border-[#31312f] pb-3 flex items-center gap-2">
                        <span class="w-2 h-2 rounded-full bg-red-500"></span>
                        <span>"Live Lobbies"</span>
                    </h2>

                    <div class="flex-1 overflow-y-auto pr-1 space-y-3">
                        if games.is_empty() {
                            <div class="text-center py-12 text-gray-500 italic">
                                <p class="text-sm">"No active games right now."</p>
                                <p class="text-xs mt-1">"Create one to start playing!"</p>
                            </div>
                        } else {
                            for game in games {
                                <div class="bg-[#1a1917] border border-[#31312f] rounded-lg p-3 hover:border-[#769656] transition flex flex-col gap-2">
                                    <div class="flex items-center justify-between">
                                        <span class="font-mono text-xs text-gray-500 truncate max-w-[120px]">
                                            (&game.id[..8])
                                        </span>
                                        <span class="inline-flex items-center px-2 py-0.5 rounded text-[10px] font-medium bg-[#262522] text-[#81b64c] uppercase">
                                            (game.status.clone())
                                        </span>
                                    </div>
                                    <div class="text-xs space-y-0.5">
                                        <div class="flex justify-between">
                                            <span class="text-gray-500">"White:"</span>
                                            if let Some(w) = game.white_player.clone() {
                                                <span class="font-mono text-white">(&w[..8])</span>
                                            } else {
                                                <span class="text-green-500 font-semibold">"Open"</span>
                                            }
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-500">"Black:"</span>
                                            if let Some(b) = game.black_player.clone() {
                                                <span class="font-mono text-white">(&b[..8])</span>
                                            } else {
                                                <span class="text-green-500 font-semibold">"Open"</span>
                                            }
                                        </div>
                                    </div>
                                    <a href=(format!("/game/{}", game.id)) class="w-full text-center bg-[#2a2926] hover:bg-[#363532] text-gray-300 font-bold py-1.5 px-3 rounded text-xs transition border border-[#31312f] block mt-1">
                                        "View / Play"
                                    </a>
                                </div>
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
