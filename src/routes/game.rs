use topcoat::{
    Result,
    context::{Cx, app_context},
    router::{page, route, see_other, SeeOther, Form, path_param, RouterErrorExt, IntoResponse, query_params},
    view::view,
};
use crate::chess_game::GameManager;
use crate::helpers::get_or_create_player_id;
use crate::views::board::render_inner_game_view;
use crate::ws::GAME_UPDATES_TX;

#[topcoat::router::query_params(error = bad_request)]
struct GameQuery {
    role: Option<String>,
}

#[path_param]
struct GameId(str);

#[derive(serde::Deserialize)]
struct JoinGameForm {
    role: String,
}

#[derive(serde::Deserialize)]
struct MoveForm {
    from_sq: String,
    to_sq: String,
    promo: Option<String>,
}

#[route(POST "/game/{game_id}/join")]
async fn join_game(cx: &Cx, Form(form): Form<JoinGameForm>) -> Result<SeeOther> {
    let game_id = path_param::<GameId>(cx);
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);

    let _ = manager.join_game(game_id, &player_id, &form.role)
        .map_err(|e| topcoat::router::bad_request(e))?;

    let _ = GAME_UPDATES_TX.send(format!("{}:refresh", game_id));

    Ok(see_other(&format!("/game/{}?role={}", game_id, form.role)))
}

#[route(POST "/game/{game_id}/move")]
async fn make_move_route(cx: &Cx, Form(form): Form<MoveForm>) -> Result {
    let game_id = path_param::<GameId>(cx);
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);

    let game = manager.make_move(game_id, &player_id, &form.from_sq, &form.to_sq, form.promo.as_deref())
        .map_err(|e| topcoat::router::bad_request(e))?;

    let _ = GAME_UPDATES_TX.send(format!("{}:refresh", game_id));

    let role_query = query_params::<GameQuery>(cx).ok().and_then(|q| q.role.clone());

    render_inner_game_view(cx, &game, &player_id, role_query.as_deref()).await
}

#[route(GET "/game/{game_id}/board")]
async fn game_board_page(cx: &Cx) -> Result<topcoat::router::Response> {
    let game_id = path_param::<GameId>(cx);
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);
    let game = manager.get_game(game_id).ok_or_not_found()?;

    let role_query = query_params::<GameQuery>(cx).ok().and_then(|q| q.role.clone());

    let view = render_inner_game_view(cx, &game, &player_id, role_query.as_deref()).await?;
    Ok((
        [("Cache-Control", "no-store, no-cache, must-revalidate")],
        view
    ).into_response(cx)?)
}

#[page("/game/{game_id}")]
async fn game_page(cx: &Cx) -> Result {
    let game_id = path_param::<GameId>(cx);
    let player_id = get_or_create_player_id(cx);
    let manager = app_context::<GameManager>(cx);
    let game = manager.get_game(game_id).ok_or_not_found()?;

    let role_query = query_params::<GameQuery>(cx).ok().and_then(|q| q.role.clone());

    let inner = render_inner_game_view(cx, &game, &player_id, role_query.as_deref()).await?;

    view! {
        <div class="flex-1 flex flex-col p-4 md:p-8 max-w-5xl mx-auto w-full">
            <div class="mb-4">
                <a href="/" class="text-gray-400 hover:text-white flex items-center gap-2 text-sm font-semibold">
                    <span>"← Back to Lobby"</span>
                </a>
            </div>

            <div id="game-container" class="w-full flex-1 flex flex-col min-h-0">
                (inner)
            </div>

            <script>
                (topcoat::view::Unescaped::new_unchecked(include_str!("../js/chess_client.js")))
            </script>
        </div>
    }
}
