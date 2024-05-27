use axum::Router;
use axum::routing::{get, post};
use crate::controller::{get_info_handler, login_handler};

pub async fn build_routes() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/info", get(get_info_handler))
}