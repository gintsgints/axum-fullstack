use crate::endpoints::{get_info_handler, login_handler};
use axum::routing::{get, get_service, post};
use axum::Router;
use tower_http::services::ServeDir;

pub async fn build_routes() -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/info", get(get_info_handler))
        .route_service("/", get_service(ServeDir::new("./web")))
}
