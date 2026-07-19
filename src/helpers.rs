use topcoat::{
    context::Cx,
    cookie::{cookies, Cookies, Cookie, SameSite},
};

/// Get the current player's ID from cookies, or create a new one.
pub fn get_or_create_player_id(cx: &Cx) -> String {
    let jar = cookies(cx);
    if let Some(c) = jar.get("player_id") {
        c.value().to_string()
    } else {
        let new_id = uuid::Uuid::new_v4().to_string();
        jar.add(
            Cookie::build(("player_id", new_id.clone()))
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .build()
        );
        new_id
    }
}
