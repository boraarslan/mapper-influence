use axum::routing::get;
use axum::Router;
use mi_api::api::auth::{authorize_from_osu_api, main_page};
use mi_api::state::SharedState;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let app_state = SharedState::new().await;
    let app = Router::new()
        .route("/", get(main_page))
        .route("/auth", get(authorize_from_osu_api))
        .layer(CookieManagerLayer::new())
        .with_state(app_state);

    println!("Listening on 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
