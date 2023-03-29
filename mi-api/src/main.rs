use axum::routing::{delete, get, post};
use axum::Router;
use mi_api::api::auth::{authorize_from_osu_api, cookie_page, login};
use mi_api::api::html::html_router;
use mi_api::api::influence::{
    create_influence, delete_influence, get_influences, update_influence_info,
    update_influence_level,
};
use mi_api::api::redoc::redoc;
use mi_api::api::user::{
    create_user, get_full_user, get_full_user_by_id, get_user, get_user_by_id, update_user,
};
use mi_api::state::SharedState;
use mi_api::ApiDoc;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

fn influence_route() -> Router<SharedState> {
    Router::new()
        .route("/get", post(get_influences))
        .route("/create", post(create_influence))
        .route("/delete", delete(delete_influence))
        .nest(
            "/update",
            Router::new()
                .route("/level", post(update_influence_level))
                .route("/info", post(update_influence_info)),
        )
}

fn user_route() -> Router<SharedState> {
    Router::new()
        .route("/get", get(get_user))
        .route("/get/full", get(get_full_user))
        .route("/get/:user_id", get(get_user_by_id))
        .route("/get/:user_id/full", get(get_full_user_by_id))
        .route("/create", post(create_user))
        .route("/update", post(update_user))
}

fn api_route() -> Router<SharedState> {
    Router::new()
        .nest("/user", user_route())
        .nest("/influence", influence_route())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();
    let port = std::env::var("PORT").expect("env var PORT is not set");
    let app_state = SharedState::new().await;
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/api-docs/", get(redoc))
        .nest("/", html_router())
        .route("/cookie", get(cookie_page))
        .route("/auth", get(authorize_from_osu_api))
        .route("/login", get(login))
        .nest("/api/v1", api_route())
        .layer(CookieManagerLayer::new())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    info!("Listening on {port}");
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
