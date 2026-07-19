mod chess_game;
mod ws;
mod helpers;
mod views;
mod routes;

use chess_game::GameManager;
use topcoat::router::{Router, RouterBuilderDiscoverExt};
use topcoat::cookie::RouterBuilderCookieExt;

#[tokio::main]
async fn main() {
    tokio::spawn(ws::start_ws_server());

    topcoat::start(
        Router::builder()
            .discover()
            .cookies()
            .app_context(GameManager::new())
            .build()
    ).await.unwrap();
}
