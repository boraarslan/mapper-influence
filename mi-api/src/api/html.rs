use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::state::SharedState;

fn serve_file(name: &str) -> ServeFile {
    ServeFile::new("mi-api/pages/".to_string() + name)
}

fn serve_dir(name: &str) -> ServeDir {
    ServeDir::new("mi-api/pages/".to_string() + name)
}

pub fn html_router() -> Router<SharedState> {
    Router::new()
        // Pages
        .route_service("/", serve_file("index.html"))
        .route_service("/profile", serve_file("profile.html"))
        .route_service("/profile/:user_id", serve_file("/profile/[mapperId].html"))
        .route_service("/oauth", serve_file("oauth.html"))
        // Assets
        .nest_service("/assets/_next", serve_dir("_next"))
        .nest_service("/svg", serve_dir("pages/svg"))
}
